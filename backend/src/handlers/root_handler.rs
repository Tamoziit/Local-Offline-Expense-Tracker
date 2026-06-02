use axum::{Router, routing::get};

pub fn routes() -> Router {
    Router::new().route("/", get(root))
}

pub async fn root() -> &'static str {
    "Server up & Running!"
}
