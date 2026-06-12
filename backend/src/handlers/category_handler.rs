use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use validator::Validate;

use crate::{
    AppState,
    dtos::category_dtos::CategoryCreationDto,
    errors::{AppJson, HttpError},
    services::category_service::CategoryExt,
};

pub fn routes() -> Router {
    Router::new()
        .route("/add", post(add_category))
        .route("/all", get(get_all_categories))
}

pub async fn add_category(
    Extension(app_state): Extension<Arc<AppState>>,
    AppJson(body): AppJson<CategoryCreationDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let category = app_state
        .db_client
        .create_category(&body.name)
        .await
        .map_err(HttpError::from)?;

    Ok((StatusCode::CREATED, Json(category)))
}

pub async fn get_all_categories(
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    let categories = app_state
        .db_client
        .get_all_categories()
        .await
        .map_err(HttpError::from)?;

    Ok(Json(categories))
}
