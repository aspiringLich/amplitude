use sea_orm::{entity::prelude::*, InsertResult, IntoActiveModel};

use crate::sea_orm_active_enums::Account;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: Uuid,
    pub name: String,
    pub avatar_url: Option<String>,
    pub created: DateTime,
    pub account: Account,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub async fn get(db: &DatabaseConnection, user_id: Uuid) -> Result<Option<Self>, DbErr> {
        Entity::find_by_id(user_id).one(db).await
    }

    pub async fn insert(self, db: &DatabaseConnection) -> Result<InsertResult<ActiveModel>, DbErr> {
        Entity::insert(self.into_active_model()).exec(db).await
    }
}
