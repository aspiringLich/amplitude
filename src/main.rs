#![feature(fs_try_exists)]

use std::{env, fs, sync::Arc, time::Duration};

use axum::Router;
use sea_orm::{ConnectOptions, Database};
use tracing::Level;

use crate::app::AppState;

mod api;
mod app;
mod config;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt::fmt()
        .with_max_level(Level::INFO)
        .init();
    tracing::info!("Starting up...");

    let config: config::Config = serde_yaml::from_str(&fs::read_to_string("config.yaml")?)?;

    if fs::try_exists(".env")? {
        dotenv::from_filename(".env")?;
    } else {
        tracing::warn!("Add a `.env` file in the project root to set DATABASE_URL");
    }

    let url = env::var("DATABASE_URL")?;
    let mut opt = ConnectOptions::new(&url);
    opt.acquire_timeout(Duration::from_secs_f32(1.0))
        .sqlx_logging(false);

    let _db = match Database::connect(opt).await {
        Ok(db) => db,
        Err(e) => {
            tracing::error!("Failed to connect to database!");
            return Err(e.into());
        }
    };
    tracing::info!("Connected to database at `{}`", &url);

    let state = AppState { cfg: config };

    let router: Router<()> = api::routes().with_state(Arc::new(state));
    let listener = tokio::net::TcpListener::bind("localhost:3000").await?;

    tracing::info!("Listening on `localhost:3000`");

    axum::serve(listener, router).await?;

    Ok(())
}
