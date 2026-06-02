use std::sync::Arc;
use tower_http::trace::TraceLayer;

use axum::{Extension, Router};

use crate::{AppState, handlers::root_handler};

fn api_routes() -> Router {
    Router::new().merge(root_handler::routes())
}

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/api/v1", api_routes())
        .layer(TraceLayer::new_for_http())
        .layer(Extension(app_state))
}
