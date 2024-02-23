use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GoogleUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GoogleUser::UserId)
                            .uuid()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(GoogleUser::GoogleId)
                            .string()
                            .not_null()
                            .unique_key()
                            .primary_key(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GoogleUser::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum GoogleUser {
    Table,
    UserId,
    GoogleId,
}
