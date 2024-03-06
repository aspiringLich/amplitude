use sea_orm::{entity::prelude::*, InsertResult, IntoActiveModel};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "session")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub session_id: Uuid,
    #[sea_orm(unique)]
    pub user_id: Uuid,
    pub expiration: DateTime,
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
    pub async fn get(db: &DatabaseConnection, session_id: Uuid) -> Result<Option<Self>, DbErr> {
        Entity::find_by_id(session_id).one(db).await
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
                tracing::error!("orphaned session! removing...");
                self.delete(db).await?;
                Err(DbErr::Custom("Orphaned".to_string()))
            }
            Err(e) => Err(e),
        }
    }

    pub async fn delete_expired(db: &DatabaseConnection) -> Result<u64, DbErr> {
        Entity::delete_many()
            .filter(Column::Expiration.lt(chrono::Utc::now().naive_utc()))
            .exec(db)
            .await
            .map(|r| r.rows_affected)
    }
}