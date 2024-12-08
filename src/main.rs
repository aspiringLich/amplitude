#![feature(try_trait_v2)]
#![feature(iter_map_windows)]
#![feature(decl_macro)]
#![feature(async_closure)]
#![feature(slice_split_once)]
#![feature(string_from_utf8_lossy_owned)]

use std::{env, fs, sync::Arc, time::Duration};

use app::Templates;
use axum::Router;
use docker_api::Docker;
use langs::Languages;
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr, Statement,
};
use tokio::{signal, task::AbortHandle};
use tracing::Level;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::app::AppState;

mod app;
mod config;
mod format;
mod langs;
mod routes;
mod runner;
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
    let expired_clear_interval = config.session.expired_clear_interval;
    let secrets: config::Secrets = serde_yaml::from_str(&fs::read_to_string("secrets.yaml")?)?;

    if fs::exists(".env")? {
        dotenv::from_filename(".env")?;
    } else {
        tracing::warn!("Add a `.env` file in the project root to set DATABASE_URL");
    }

    let db_full_url = env::var("DATABASE_URL")?;
    let Some((db_url, db_name)) = db_full_url.rsplit_once('/') else {
        panic!("expected env DATABASE_URL to contain slashes")
    };
    let db_opts = |url: &str| {
        let mut o = ConnectOptions::new(url);
        o.acquire_timeout(Duration::from_secs_f32(1.0))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Debug);
        o
    };

    let db = match Database::connect(db_opts(&db_url)).await {
        Ok(db) => db,
        Err(e) => {
            tracing::error!("Failed to connect to postgresql!");
            return Err(e.into());
        }
    };
    tracing::info!("Connected to postgresql at `{}`", &db_url);

    let db = match db.get_database_backend() {
        DbBackend::Postgres => match Database::connect(db_opts(&db_full_url)).await {
            Ok(db) => db,
            Err(DbErr::Conn(_)) => {
                tracing::info!("Creating database `{db_name}`...");
                db.execute(Statement::from_string(
                    db.get_database_backend(),
                    format!("DROP DATABASE IF EXISTS \"{}\";", db_name),
                ))
                .await?;
                db.execute(Statement::from_string(
                    db.get_database_backend(),
                    format!("CREATE DATABASE \"{}\";", db_name),
                ))
                .await?;
                Database::connect(&db_full_url).await?;

                tracing::info!("Please run `cargo r -p migration -- fresh` to setup the database!");
                return Ok(());
            }
            Err(e) => return Err(e.into()),
        },
        _ => unreachable!("expected postgres database"),
    };

    let docker = Docker::new(&config.docker.host)?;
    tracing::info!("Connected to Docker Daemon at `{}`", &config.docker.host);

    let langs = Languages::new()?;
    let mut templates = Templates::new(handlebars::Handlebars::new());
    let runner_registry =
        runner::generate_registry(&config.docker, &docker, &langs, &mut templates).await?;

    let state = AppState {
        config,
        secrets,
        db: db.clone(),
        docker,
        runner_registry,
        templates,
        langs,
    };

    let state = Arc::new(state);
    let router: Router<_> = routes::routes();
    let router: Router<()> = router.with_state(state);
    let listener = tokio::net::TcpListener::bind("localhost:3000").await?;

    tracing::info!("Listening on `localhost:3000`");

    let mut interval = tokio::time::interval(expired_clear_interval);
    let db_ = db.clone();
    let handle = tokio::task::spawn(async move {
        loop {
            interval.tick().await;
            match entity::session::Model::delete_expired(&db_).await {
                Ok(n) if n > 0 => tracing::trace!("Deleted {} expired sessions", n),
                Ok(_) => (),
                Err(e) => tracing::error!("Failed to delete expired sessions: {e}"),
            }
        }
    });

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal(handle.abort_handle(), db))
        .await?;

    tracing::info!("Done!");

    Ok(())
}

/// stolen from https://github.com/maxcountryman/tower-sessions-stores/blob/main/sqlx-store/README.md
async fn shutdown_signal(abort: AbortHandle, db: DatabaseConnection) {
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
            abort.abort();
            db.close().await.expect("Failed to close database connection");
        },
        _ = terminate => {
            tracing::info!("Terminate! Shutting down...");
            abort.abort();
            db.close().await.expect("Failed to close database connection");
        },
    }
}
