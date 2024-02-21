use std::{fs, time::Duration};

use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database};
use tracing::Level;

mod config;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt::fmt()
        .with_max_level(Level::INFO)
        .init();
    tracing::info!("Starting up...");

    let config: config::Config = serde_yaml::from_str(&fs::read_to_string("config.yaml")?)?;

    let mut opt = ConnectOptions::new(&config.database_url);
    opt.acquire_timeout(Duration::from_secs_f32(1.0))
        .sqlx_logging(false);

    let db = match Database::connect(opt).await {
        Ok(db) => db,
        Err(e) => {
            tracing::error!("Failed to connect to database!");
            return Err(e.into());
        }
    };
    tracing::info!("Connected to database at `{}`", config.database_url);

    #[cfg(debug_assertions)]
    {
        tracing::info!("DEV: refreshing database...");
        Migrator::fresh(&db).await?;
    }
    
    
    

    Ok(())
}
