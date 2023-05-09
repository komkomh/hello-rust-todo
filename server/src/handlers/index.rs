use axum::response::Html;

pub async fn index_handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h>")
}