use crate::dtos::todo::SearchParams;
use crate::entities::todo;
use crate::entities::todo::Model;
use crate::repositories::todo::TodoRepository;
use anyhow::Error;
use sea_orm::DatabaseConnection;

pub struct TodoService;

impl TodoService {
    pub async fn search(
        db: &DatabaseConnection,
        params: SearchParams,
    ) -> anyhow::Result<Vec<Model>> {
        TodoRepository::search(db, params).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        active_model: todo::ActiveModel,
    ) -> anyhow::Result<Model, Error> {
        TodoRepository::create(db, active_model).await
    }

    pub async fn find(db: &DatabaseConnection, id: i32) -> anyhow::Result<Option<Model>> {
        TodoRepository::find(db, id).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        active_model: todo::ActiveModel,
    ) -> Result<Option<Model>, Error> {
        TodoRepository::update(db, active_model).await
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> anyhow::Result<Option<i32>> {
        TodoRepository::delete(db, id).await
    }
}
