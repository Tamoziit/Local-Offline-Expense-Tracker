use axum::{Json, Router, routing::get};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/v1", get(root_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

async fn root_handler() -> &'static str {
    "Server up & Running!"
}
