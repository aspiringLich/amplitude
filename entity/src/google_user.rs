use sea_orm::{entity::prelude::*, InsertResult, IntoActiveModel};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "google_user")]
pub struct Model {
    #[sea_orm(unique)]
    pub user_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub google_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::UserId"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub async fn get(db: &DatabaseConnection, google_id: &str) -> Result<Option<Self>, DbErr> {
        Entity::find_by_id(google_id).one(db).await
    }

    pub async fn insert(self, db: &DatabaseConnection) -> Result<InsertResult<ActiveModel>, DbErr> {
        Entity::insert(self.into_active_model()).exec(db).await
    }

    pub async fn find_related_user(
        self,
        db: &DatabaseConnection,
    ) -> Result<super::user::Model, DbErr> {
        match self.find_related(super::user::Entity).one(db).await {
            Ok(Some(user)) => Ok(user),
            Ok(None) => {
                tracing::error!("orphaned google user! removing...");
                self.delete(db).await?;
                Err(DbErr::Custom("Orphaned".to_string()))
            }
            Err(e) => Err(e),
        }
    }
}
