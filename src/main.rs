#![feature(fs_try_exists)]
#![feature(try_trait_v2)]
#![feature(iter_map_windows)]
#![feature(decl_macro)]

use std::{env, fs, sync::Arc, time::Duration};

use axum::Router;
use sea_orm::{ConnectOptions, Database};
use tokio::signal;
use tracing::Level;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::app::AppState;

mod app;
mod config;
mod format;
mod routes;
mod views;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let filter = filter::Targets::new()
        .with_default(Level::INFO)
        .with_target("axum", Level::DEBUG)
        .with_target("tower_http", Level::DEBUG)
        .with_target("sea_orm", Level::DEBUG)
        .with_target("sqlx", Level::DEBUG)
        .with_target("amplitude", Level::TRACE);
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().event_format(format::Format))
        .init();

    tracing::info!("Starting up...");

    let config: config::Config = serde_yaml::from_str(&fs::read_to_string("config.yaml")?)?;
    let secrets: config::Secrets = serde_yaml::from_str(&fs::read_to_string("secrets.yaml")?)?;

    if fs::try_exists(".env")? {
        dotenv::from_filename(".env")?;
    } else {
        tracing::warn!("Add a `.env` file in the project root to set DATABASE_URL");
    }

    let url = env::var("DATABASE_URL")?;
    let mut opt = ConnectOptions::new(&url);
    opt.acquire_timeout(Duration::from_secs_f32(1.0))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug);

    let db = match Database::connect(opt).await {
        Ok(db) => db,
        Err(e) => {
            tracing::error!("Failed to connect to database!");
            return Err(e.into());
        }
    };
    tracing::info!("Connected to database at `{}`", &url);

    let router: Router<_> = routes::routes();
    let state = AppState {
        config,
        secrets,
        db: db.clone(),
    };

    let state = Arc::new(state);
    let router: Router<()> = router.with_state(state);
    let listener = tokio::net::TcpListener::bind("localhost:3000").await?;

    tracing::info!("Listening on `localhost:3000`");

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    db.close().await?;
    tracing::info!("Done!");

    Ok(())
}

/// stolen from https://github.com/maxcountryman/tower-sessions-stores/blob/main/sqlx-store/README.md
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
    _ = ctrl_c => {
            tracing::info!("Ctrl-C! Shutting down...");
        },
        _ = terminate => {
            tracing::info!("Terminate! Shutting down...");
        },
    }
}
