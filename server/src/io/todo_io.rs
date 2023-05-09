use crate::dtos::todo::SearchParams;
use crate::entities::sea_orm_active_enums::Status;
use crate::entities::todo;
use crate::entities::todo::Model;
use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;
use sea_orm::ActiveValue::Set;
use sea_orm::NotSet;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct TodoSearchRequest {
    pub title: Option<String>,
    pub contents: Option<String>,
    pub status: Option<Status>,
    pub stated_date_from: Option<NaiveDate>,
    pub stated_date_to: Option<NaiveDate>,
    pub ended_date_from: Option<NaiveDate>,
    pub ended_date_to: Option<NaiveDate>,
    pub rank_from: Option<Decimal>,
    pub rank_to: Option<Decimal>,
    pub updated_at_from: Option<NaiveDateTime>,
    pub updated_at_to: Option<NaiveDateTime>,
}

impl TodoSearchRequest {
    pub fn into(self, user_id: i32) -> SearchParams {
        SearchParams {
            title: self.title,
            contents: self.contents,
            status: self.status,
            stated_date_from: self.stated_date_from,
            stated_date_to: self.stated_date_to,
            ended_date_from: self.ended_date_from,
            ended_date_to: self.ended_date_to,
            rank_from: self.rank_from,
            rank_to: self.rank_to,
            updated_at_from: self.updated_at_from,
            updated_at_to: self.updated_at_to,
            user_id,
            ..SearchParams::default()
        }
    }
}

#[derive(Deserialize, Debug, Validate)]
pub struct TodoCreateRequest {
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    #[validate(length(min = 0, max = 1000))]
    pub contents: Option<String>,

    pub(crate) status: Status,
    pub(crate) stated_date: Option<NaiveDate>,
    pub(crate) ended_date: Option<NaiveDate>,
    pub(crate) rank: Decimal,
}

impl TodoCreateRequest {
    pub fn into(self, user_id: i32) -> todo::ActiveModel {
        todo::ActiveModel {
            id: NotSet,
            title: Set(self.title),
            contents: Set(self.contents),
            status: Set(self.status),
            started_date: Set(self.stated_date),
            ended_date: Set(self.ended_date),
            rank: Set(self.rank),
            user_id: Set(user_id),
            updated_at: NotSet,
            created_at: NotSet,
        }
    }
}

#[derive(Deserialize, Debug, Validate)]
pub struct TodoUpdateRequest {
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    #[validate(length(min = 0, max = 1000))]
    pub contents: Option<String>,

    pub(crate) status: Status,
    pub(crate) stated_date: Option<NaiveDate>,
    pub(crate) ended_date: Option<NaiveDate>,
    pub(crate) rank: Decimal,
}

impl TodoUpdateRequest {
    pub fn into(self, id: i32, user_id: i32) -> todo::ActiveModel {
        todo::ActiveModel {
            id: Set(id),
            title: Set(self.title),
            contents: Set(self.contents),
            status: Set(self.status),
            started_date: Set(self.stated_date),
            ended_date: Set(self.ended_date),
            rank: Set(self.rank),
            user_id: Set(user_id),
            updated_at: NotSet,
            created_at: NotSet,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoResponse {
    pub(crate) id: i32,
    pub(crate) title: String,
    pub(crate) contents: Option<String>,
    pub(crate) status: Status,
    pub(crate) stated_date: Option<NaiveDate>,
    pub(crate) ended_date: Option<NaiveDate>,
    pub(crate) rank: Decimal,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

impl From<Model> for TodoResponse {
    fn from(value: Model) -> Self {
        TodoResponse {
            id: value.id,
            title: value.title.to_owned(),
            contents: value.contents,
            status: value.status,
            stated_date: value.started_date,
            ended_date: value.ended_date,
            rank: value.rank,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
