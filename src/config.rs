use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub debug: DebugConfig,
}

#[derive(Deserialize, Clone)]
pub struct DebugConfig {
    pub forward_port: Option<u16>,
}
