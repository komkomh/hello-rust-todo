use crate::io::user_io::{UserCreateRequest, UserResponse, UserUpdateRequest};
use crate::services::user::UserService;
use crate::{AppState, ValidatedRequest};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use tracing::error;

pub async fn post_user_handler(
    State(state): State<AppState>,
    request: ValidatedRequest<UserCreateRequest>,
) -> anyhow::Result<impl IntoResponse, StatusCode> {
    UserService::create(&state.db, request.0.into())
        .await
        .map(|model| (StatusCode::CREATED, Json(json!(UserResponse::from(model)))))
        .map_err(|err| {
            error!("Unexpected error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub async fn get_user_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> anyhow::Result<impl IntoResponse, StatusCode> {
    UserService::find(&state.db, id)
        .await
        .map(|model| match model {
            Some(user) => (StatusCode::OK, Json(json!(UserResponse::from(user)))),
            None => (StatusCode::NOT_FOUND, Json::default()),
        })
        .map_err(|err| {
            error!("Unexpected error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub async fn put_user_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    request: ValidatedRequest<UserUpdateRequest>,
) -> anyhow::Result<impl IntoResponse, StatusCode> {
    UserService::update(&state.db, request.0.into(id))
        .await
        .map(|option_user| match option_user {
            Some(user) => (StatusCode::OK, Json(json!(UserResponse::from(user)))),
            None => (StatusCode::NOT_FOUND, Json::default()),
        })
        .map_err(|err| {
            error!("Unexpected error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub async fn delete_user_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> anyhow::Result<impl IntoResponse, StatusCode> {
    UserService::delete(&state.db, id)
        .await
        .map(|option_id| match option_id {
            Some(_) => StatusCode::OK,
            None => StatusCode::NOT_FOUND,
        })
        .map_err(|err| {
            error!("Unexpected error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub async fn list_user_handler(State(state): State<AppState>) -> impl IntoResponse {
    UserService::search(&state.db)
        .await
        .map(|users| {
            let responses: Vec<UserResponse> = users
                .iter()
                .map(|u| UserResponse::from(u.clone()))
                .collect();
            (StatusCode::OK, Json(json!(responses)))
        })
        .map_err(|err| {
            error!("Unexpected error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
