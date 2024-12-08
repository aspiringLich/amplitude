use std::{borrow::Cow, collections::HashMap, io::BufRead};

use docker_api::{conn::TtyChunk, Docker};
use eyre::Context;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};

use crate::app::Templates;

use super::Runner;

#[derive(Debug, Deserialize, Serialize)]
pub struct ExecConfig<'a> {
    pub content: &'a str,
    pub inputs: Vec<Type>,
    pub output: Type,
    pub hidden_cases: u16,
    pub visible_cases: u16,
    pub generate_cases: u16,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Int,
    Float,
    String,
    // #[serde(untagged)]
    // Array(Box<Type>, Option<u16>),
    // TODO: array support
    // map support?
}

type GeneratorResult = Result<GeneratorSuccess, ExecutionError>;

#[derive(Debug, Deserialize, PartialEq)]
pub struct GeneratorCase {
    pub input: Vec<serde_json::Value>,
    pub output: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct GeneratorSuccess {
    pub cases: Vec<GeneratorCase>,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Deserialize)]
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
        cfg: &ExecConfig<'_>,
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

            Ok(Ok(GeneratorSuccess {
                cases,
                stdout: String::from_utf8_lossy_owned(stdout),
                stderr: String::from_utf8_lossy_owned(stderr),
            }))
        } else {
            Ok(Err(ExecutionError {
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
        runner::{
            self,
            exec::{ExecConfig, GeneratorCase, Type},
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

        let output = reg["python"]
            .run_generator(
                &templates,
                &docker,
                &ExecConfig {
                    content: gen,
                    inputs: vec![Type::Int, Type::Int],
                    output: Type::Int,
                    hidden_cases: 0,
                    visible_cases: 0,
                    generate_cases: 2,
                },
            )
            .await
            .unwrap()
            .expect("no error");

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
