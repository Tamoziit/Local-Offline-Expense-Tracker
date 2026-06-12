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
    dtos::account_dtos::AccountCreationDto,
    errors::{AppJson, ErrorMessage, HttpError},
    services::account_service::AccountExt,
};

pub fn routes() -> Router {
    Router::new()
        .route("/create-account", post(create_account))
        .route("/my-account", get(get_my_personal_acc))
        .route("/external-accounts", get(get_external_acc))
}

pub async fn create_account(
    Extension(app_state): Extension<Arc<AppState>>,
    AppJson(body): AppJson<AccountCreationDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let account = app_state
        .db_client
        .create_account(&body.name, &body.account_type)
        .await
        .map_err(HttpError::from)?;

    Ok((StatusCode::CREATED, Json(account)))
}

pub async fn get_my_personal_acc(
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    let my_account = app_state
        .db_client
        .get_my_personal_acc()
        .await
        .map_err(HttpError::from)?;

    match my_account {
        Some(account) => Ok((StatusCode::OK, Json(account))),
        None => Err(HttpError::resource_not_found(
            ErrorMessage::ResourceNotFound("Personal Account".to_string()).to_string(),
        )),
    }
}

pub async fn get_external_acc(
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    let external_accounts = app_state
        .db_client
        .get_external_acc()
        .await
        .map_err(HttpError::from)?;

    Ok(Json(external_accounts))
}
