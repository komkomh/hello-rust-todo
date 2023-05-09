use axum::extract::FromRequest;
use axum::http::{Request, StatusCode};
use axum::response::{IntoResponse, Response};
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::body::HttpBody;
use axum::routing::{get, post};
use axum::{async_trait, BoxError, Json, Router};
use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::handlers::index::index_handler;
use crate::handlers::todo::{
    delete_todo_handler, get_todo_handler, post_todo_handler, put_todo_handler, search_todo_handler,
};
use crate::handlers::user::{
    delete_user_handler, get_user_handler, list_user_handler, post_user_handler, put_user_handler,
};
use thiserror::Error;

mod dtos;
mod entities;
mod handlers;
mod io;
mod models;
mod repositories;
mod services;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let environment = match env::var("ENVIRONMENT") {
        Ok(val) => val,
        Err(_) => "local".to_string(),
    };
    // 環境毎にファイルを配置して読み込み
    dotenv::from_filename(".env.".to_string() + &environment).ok();
    let database_url = &env::var("DATABASE_URL").unwrap();
    let db = Database::connect(database_url).await.unwrap_or_else(|_| {
        panic!("Cannot connect to the database. Please check your configuration.")
    });
    let app_state = AppState { db: Arc::new(db) };

    // Routerを設定する
    let user_router = Router::new()
        .route("/", get(list_user_handler).post(post_user_handler))
        .route(
            "/:id",
            get(get_user_handler)
                .put(put_user_handler)
                .delete(delete_user_handler),
        );

    let todo_router = Router::new()
        .route("/", post(post_todo_handler))
        .route(
            "/:id",
            get(get_todo_handler)
                .put(put_todo_handler)
                .delete(delete_todo_handler),
        )
        .route("/search", post(search_todo_handler));

    let app = Router::new()
        .route("/", get(index_handler))
        .nest("/users", user_router)
        .nest("/todos", todo_router)
        .with_state(app_state);

    // 起動設定を行う
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("listening on {}", addr);

    // サーバを起動する
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch!"));

    Ok(())
}

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Validation(_) => {
                let msg = format!("Input validation error: [{}]", self).replace('\n', ", ");
                (StatusCode::BAD_REQUEST, msg)
            }
            AppError::JsonRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        }
        .into_response()
    }
}

#[derive(Debug)]
pub struct ValidatedRequest<T>(pub T);

#[async_trait]
impl<S, B, T> FromRequest<S, B> for ValidatedRequest<T>
where
    T: DeserializeOwned + Validate,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedRequest(value))
    }
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),
    #[error(transparent)]
    JsonRejection(#[from] axum::extract::rejection::JsonRejection),
}
