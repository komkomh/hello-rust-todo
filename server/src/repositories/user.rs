use crate::entities::prelude::User;
use crate::entities::*;
use anyhow::bail;
use sea_orm::DbErr::RecordNotUpdated;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};

pub struct UserRepository;

impl UserRepository {
    pub async fn search(db: &DatabaseConnection) -> anyhow::Result<Vec<user::Model>> {
        User::find().all(db).await.or_else(|err| bail!(err))
    }

    pub async fn find(db: &DatabaseConnection, id: i32) -> anyhow::Result<Option<user::Model>> {
        User::find_by_id(id).one(db).await.or_else(|err| bail!(err))
    }

    pub async fn insert(
        db: &DatabaseConnection,
        active_model: user::ActiveModel,
    ) -> anyhow::Result<user::Model> {
        active_model.insert(db).await.or_else(|err| bail!(err))
    }

    pub async fn update(
        db: &DatabaseConnection,
        active_model: user::ActiveModel,
    ) -> anyhow::Result<Option<user::Model>> {
        active_model
            .update(db)
            .await
            .map(Some)
            .or_else(|err| match err {
                RecordNotUpdated => Ok(None),
                _ => bail!(err),
            })
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> anyhow::Result<Option<i32>> {
        User::delete_by_id(id)
            .exec(db)
            .await
            .map(|result| match result.rows_affected {
                count if count > 0 => Some(id),
                _ => None,
            })
            .or_else(|err| bail!(err))
    }
}
