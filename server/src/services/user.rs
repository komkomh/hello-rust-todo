use crate::entities::*;
use crate::repositories::user::UserRepository;
use sea_orm::DatabaseConnection;

pub struct UserService;

impl UserService {
    pub async fn search(db: &DatabaseConnection) -> Result<Vec<user::Model>, anyhow::Error> {
        UserRepository::search(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: user::ActiveModel,
    ) -> anyhow::Result<user::Model> {
        UserRepository::insert(db, model).await
    }

    pub async fn find(db: &DatabaseConnection, id: i32) -> anyhow::Result<Option<user::Model>> {
        UserRepository::find(db, id).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        active_model: user::ActiveModel,
    ) -> anyhow::Result<Option<user::Model>> {
        UserRepository::update(db, active_model).await
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> anyhow::Result<Option<i32>> {
        UserRepository::delete(db, id).await
    }
}
