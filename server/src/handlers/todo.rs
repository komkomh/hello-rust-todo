use crate::io::todo_io::{TodoCreateRequest, TodoResponse, TodoSearchRequest, TodoUpdateRequest};
use crate::services::todo::TodoService;
use crate::{AppState, ValidatedRequest};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use tracing::error;

pub async fn post_todo_handler(
    State(state): State<AppState>,
    param: ValidatedRequest<TodoCreateRequest>,
) -> anyhow::Result<impl IntoResponse, StatusCode> {
    // TODO get user_id
    let user_id = 1;
    TodoService::create(&state.db, param.0.into(user_id))
        .await
        .map(TodoResponse::from)
        .map(|res| (StatusCode::CREATED, Json(json!(res))))
        .map_err(|err| {
            error!("Unexpected error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub async fn get_todo_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> anyhow::Result<impl IntoResponse, StatusCode> {
    TodoService::find(&state.db, id)
        .await
        .map(|model| match model {
            Some(model) => (StatusCode::CREATED, Json(json!(TodoResponse::from(model)))),
            None => (StatusCode::NOT_FOUND, Json::default()),
        })
        .map_err(|err| {
            error!("Unexpected error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub async fn put_todo_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    request: ValidatedRequest<TodoUpdateRequest>,
) -> anyhow::Result<impl IntoResponse, StatusCode> {
    let user_id = 1;
    TodoService::update(&state.db, request.0.into(id, user_id))
        .await
        .map(|model| match model {
            Some(model) => (StatusCode::OK, Json(json!(TodoResponse::from(model)))),
            None => (StatusCode::NOT_FOUND, Json::default()),
        })
        .map_err(|err| {
            error!("Unexpected error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub async fn delete_todo_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> anyhow::Result<impl IntoResponse, StatusCode> {
    TodoService::delete(&state.db, id)
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

pub async fn search_todo_handler(
    State(state): State<AppState>,
    request: ValidatedRequest<TodoSearchRequest>,
) -> anyhow::Result<impl IntoResponse, StatusCode> {
    let user_id = 1;
    TodoService::search(&state.db, request.0.into(user_id))
        .await
        .map(|users| {
            let responses: Vec<TodoResponse> = users
                .iter()
                .map(|u| TodoResponse::from(u.clone()))
                .collect();
            (StatusCode::OK, Json(json!(responses)))
        })
        .map_err(|err| {
            error!("Unexpected error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
