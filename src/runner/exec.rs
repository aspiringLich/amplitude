use std::{borrow::Cow, collections::HashMap, io::BufRead};

use docker_api::{conn::TtyChunk, Docker};
use eyre::Context;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};

use crate::{app::Templates, routes::exec::ExecRequest};

use super::Runner;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum GeneratorResult {
    Success(GeneratorSuccess),
    Err(ExecutionError),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GeneratorCase {
    pub input: Vec<serde_json::Value>,
    pub output: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratorSuccess {
    pub cases: Vec<GeneratorCase>,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionError {
    pub exit_code: isize,
    pub stdout: String,
    pub stderr: String,
}

impl Runner {
    pub async fn run_generator(
        &self,
        templates: &Templates,
        docker: &Docker,
        cfg: &ExecRequest,
    ) -> eyre::Result<GeneratorResult> {
        let container = self.create_container(docker).await?;

        let gen = templates.render_generator(&self.lang, cfg)?;
        container
            .copy_file_into("/runner/main.py", gen.into_bytes().as_slice())
            .await?;
        container
            .copy_file_into("/runner/gen.py", cfg.content.as_bytes())
            .await?;

        let (mut out_stream, _) = container.attach().await?.split();
        container.start().await?;

        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        while let Some(chunk) = out_stream.try_next().await? {
            match chunk {
                TtyChunk::StdIn(_) => {} // ignore
                TtyChunk::StdOut(data) => stdout.extend(data),
                TtyChunk::StdErr(data) => stderr.extend(data),
            }
        }

        container.wait().await?;
        let exit_code = container
            .inspect()
            .await?
            .state
            .and_then(|s| s.exit_code)
            .unwrap_or(-1);

        if exit_code == 0 {
            let (_, last_line) = stdout
                .trim_ascii_end()
                .rsplit_once(|b| *b == b'\n')
                .unwrap_or((b"", stdout.trim_ascii_end()));
            let cases: Vec<GeneratorCase> =
                serde_json::from_slice(last_line).context("While parsing generator output")?;

            Ok(GeneratorResult::Success(GeneratorSuccess {
                cases,
                stdout: String::from_utf8_lossy_owned(stdout),
                stderr: String::from_utf8_lossy_owned(stderr),
            }))
        } else {
            Ok(GeneratorResult::Err(ExecutionError {
                exit_code,
                stdout: String::from_utf8_lossy_owned(stdout),
                stderr: String::from_utf8_lossy_owned(stderr),
            }))
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use docker_api::Docker;
    use serde_json::json;

    use crate::{
        app::Templates,
        config,
        langs::Languages,
        routes::exec::Type,
        runner::{
            self,
            exec::{ExecRequest, GeneratorCase},
        },
    };

    #[tokio::test]
    async fn test_generator() {
        let cfg: config::Config =
            serde_yaml::from_str(&fs::read_to_string("config.yaml").unwrap()).unwrap();
        let docker = Docker::new(&cfg.docker.host).unwrap();
        let langs = Languages::new().unwrap();
        let mut templates = Templates::new(handlebars::Handlebars::new());
        let reg = runner::generate_registry(&cfg.docker, &docker, &langs, &mut templates)
            .await
            .unwrap();

        let gen = "def gen(ctx):       \n\
                           \tctx.input(1, 1) \n\
                           \tctx.output(2)   \n";

        let runner::exec::GeneratorResult::Success(output) = reg["python"]
            .run_generator(
                &templates,
                &docker,
                &ExecRequest {
                    language: "python".to_string(),
                    content: gen.to_string(),
                    inputs: vec![Type::Int, Type::Int],
                    output: Type::Int,
                    hidden_cases: 0,
                    visible_cases: 0,
                    generate_cases: 2,
                },
            )
            .await
            .unwrap()
        else {
            panic!()
        };

        let expected: Vec<GeneratorCase> = serde_json::from_value(json!([
            {
                "input": [1, 1],
                "output": 2
            },
            {
                "input": [1, 1],
                "output": 2
            }
        ]))
        .unwrap();
        assert_eq!(output.cases, expected)
    }
}
