pub use sea_orm_migration::prelude::*;

mod m20240221_034005_user;
mod m20240223_151834_google_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240221_034005_user::Migration),
            Box::new(m20240223_151834_google_user::Migration),
        ]
    }
}
