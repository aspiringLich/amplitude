use std::collections::HashMap;

use docker_api::Docker;
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

type GeneratorOutput = Vec<GeneratorOutputItem>;

#[derive(Debug, Deserialize)]
pub struct GeneratorOutputItem {
    pub inputs: Vec<serde_json::Value>,
    pub output: serde_json::Value,
}

impl Runner {
    pub async fn run_generator(
        &self,
        templates: &Templates,
        docker: &Docker,
        cfg: &ExecConfig<'_>,
    ) -> docker_api::Result<GeneratorOutput> {
        let container = self.get_container(docker);
        todo!()
    }
}
