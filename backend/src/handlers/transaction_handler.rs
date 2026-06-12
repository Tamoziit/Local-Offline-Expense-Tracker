use std::sync::Arc;

use axum::{Extension, Json, Router, http::StatusCode, response::IntoResponse, routing::post};
use validator::Validate;

use crate::{
    AppState,
    dtos::transaction_dtos::CreateTransactionDto,
    errors::{AppJson, HttpError},
    services::transaction_service::TransactionExt,
};

pub fn routes() -> Router {
    Router::new().route("/record", post(record_transaction))
}

pub async fn record_transaction(
    Extension(app_state): Extension<Arc<AppState>>,
    AppJson(body): AppJson<CreateTransactionDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let transaction = app_state
        .db_client
        .create_transaction(
            &body.title,
            body.description.as_deref(),
            body.category_id,
            &body.transaction_type,
            &body.transaction_mode,
            body.is_recurring,
            body.from_account_id,
            body.to_account_id,
            body.amount,
            body.transaction_date,
            body.transaction_status,
        )
        .await
        .map_err(HttpError::from)?;

    Ok((StatusCode::CREATED, Json(transaction)))
}
