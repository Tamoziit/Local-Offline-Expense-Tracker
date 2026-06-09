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
    errors::{AppJson, ErrorMessage, HttpError},
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

    let response = app_state.db_client.create_category(&body.name).await;

    match response {
        Ok(Some(category)) => Ok((StatusCode::CREATED, Json(category))),
        Ok(None) => Err(HttpError::server_error(
            "Account could not be created".to_string(),
        )),
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                Err(HttpError::unique_constraint_violated(
                    ErrorMessage::CategoryAlreadyExists.to_string(),
                ))
            } else {
                Err(HttpError::server_error(db_err.to_string()))
            }
        }
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}

pub async fn get_all_categories(
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    let categories = app_state
        .db_client
        .get_all_categories()
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(categories))
}
