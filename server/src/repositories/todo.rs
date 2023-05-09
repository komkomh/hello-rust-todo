use crate::dtos::todo::SearchParams;
use crate::entities::prelude::Todo;
use crate::entities::todo;
use crate::entities::todo::Model;
use anyhow::{bail, Error};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder,
    QueryTrait,
};

pub struct TodoRepository;

impl TodoRepository {
    pub async fn search(
        db: &DatabaseConnection,
        params: SearchParams,
    ) -> anyhow::Result<Vec<Model>> {
        Todo::find()
            .apply_if(params.title, |query, v| {
                query.filter(todo::Column::Title.contains(v.as_str()))
            })
            .apply_if(params.contents, |query, v| {
                query.filter(todo::Column::Contents.contains(v.as_str()))
            })
            .apply_if(params.status, |query, v| {
                query.filter(todo::Column::Status.eq(v))
            })
            .apply_if(params.stated_date_from, |query, v| {
                query.filter(todo::Column::StartedDate.gt(v))
            })
            .apply_if(params.stated_date_to, |query, v| {
                query.filter(todo::Column::StartedDate.lte(v))
            })
            .apply_if(params.ended_date_from, |query, v| {
                query.filter(todo::Column::Title.gt(v))
            })
            .apply_if(params.ended_date_to, |query, v| {
                query.filter(todo::Column::Title.lte(v))
            })
            .apply_if(params.rank_from, |query, v| {
                query.filter(todo::Column::Rank.gt(v))
            })
            .apply_if(params.rank_to, |query, v| {
                query.filter(todo::Column::Rank.lte(v))
            })
            .apply_if(params.updated_at_from, |query, v| {
                query.filter(todo::Column::UpdatedAt.gt(v))
            })
            .apply_if(params.updated_at_to, |query, v| {
                query.filter(todo::Column::UpdatedAt.lte(v))
            })
            .apply_if(params.created_at_from, |query, v| {
                query.filter(todo::Column::CreatedAt.gt(v))
            })
            .apply_if(params.created_at_to, |query, v| {
                query.filter(todo::Column::CreatedAt.lte(v))
            })
            .filter(todo::Column::UserId.eq(params.user_id))
            .order_by_desc(todo::Column::UpdatedAt)
            .all(db)
            .await
            .or_else(|err| bail!(err))
    }

    pub async fn find(db: &DatabaseConnection, id: i32) -> anyhow::Result<Option<Model>, Error> {
        Todo::find_by_id(id).one(db).await.or_else(|err| bail!(err))
    }

    pub async fn create(
        db: &DatabaseConnection,
        active_model: todo::ActiveModel,
    ) -> anyhow::Result<Model, Error> {
        active_model.insert(db).await.or_else(|err| bail!(err))
    }

    pub async fn update(
        db: &DatabaseConnection,
        active_model: todo::ActiveModel,
    ) -> anyhow::Result<Option<Model>, Error> {
        active_model
            .update(db)
            .await
            .map(Some)
            .or_else(|err| match err {
                DbErr::RecordNotUpdated => Ok(None),
                _ => bail!(err),
            })
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> anyhow::Result<Option<i32>> {
        Todo::delete_by_id(id)
            .exec(db)
            .await
            .map(|result| match result.rows_affected {
                count if count > 0 => Some(id),
                _ => None,
            })
            .or_else(|err| bail!(err))
    }
}
