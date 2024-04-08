use std::collections::BTreeMap;

use docker_api::Container;

use crate::config::DockerConfig;

struct LanguageRunner {}

struct Runner {
    language: String,
    container: Container,
}

impl Runner {
    pub fn new(language: &str, cfg: &DockerConfig) -> Self {
        todo!()
    }
}

type RunnerRegistry = BTreeMap<String, Container>;
