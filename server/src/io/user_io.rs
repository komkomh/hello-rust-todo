use chrono::{NaiveDateTime};
use sea_orm::ActiveValue::Set;
use sea_orm::NotSet;
use crate::entities::user::Model;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::entities::user;

#[derive(Deserialize, Debug, Validate)]
pub struct UserCreateRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
}

impl From<UserCreateRequest> for user::ActiveModel {
    fn from(value: UserCreateRequest) -> Self {
        user::ActiveModel {
            id: NotSet,
            name: Set(value.name),
            created_at: NotSet,
            updated_at: NotSet,
        }
    }
}

#[derive(Deserialize, Debug, Validate)]
pub struct UserUpdateRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
}

impl UserUpdateRequest {
    pub fn into(self, id : i32) -> user::ActiveModel {
        user::ActiveModel {
            id: Set(id),
            name: Set(self.name),
            created_at: NotSet,
            updated_at: NotSet,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

impl From<Model> for UserResponse {
    fn from(value: Model) -> Self {
        UserResponse {
            id: value.id,
            name: value.name,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
