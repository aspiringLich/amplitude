use docker_api::Docker;
use sea_orm::DatabaseConnection;

use crate::{config::{Config, Secrets}, runner::RunnerRegistry};


pub struct AppState {
    pub config: Config,
    pub secrets: Secrets,
    pub db: DatabaseConnection,
    pub docker: Docker,
    pub runner_registry: RunnerRegistry,
}