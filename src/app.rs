use sea_orm::DatabaseConnection;

use crate::config::{Config, Secrets};


pub struct AppState {
    pub config: Config,
    pub secrets: Secrets,
    pub db: DatabaseConnection,
}