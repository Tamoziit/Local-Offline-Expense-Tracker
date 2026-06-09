use std::fmt;

use axum::{
    Json,
    extract::{FromRequest, Request, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorMessage {
    ServerError,
    ResourceNotFound(String),
    AccountAlreadyExists,
    PersonalAccountAlreadyExists,
    CategoryAlreadyExists,
}

impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorMessage::ServerError => {
                write!(f, "Server Error. Please try again later")
            }
            ErrorMessage::ResourceNotFound(resource) => {
                write!(f, "{} not found", resource)
            }
            ErrorMessage::AccountAlreadyExists => {
                write!(f, "An account with this name already exists")
            }
            ErrorMessage::PersonalAccountAlreadyExists => {
                write!(f, "A Personal Account already exists on this device")
            }
            ErrorMessage::CategoryAlreadyExists => {
                write!(f, "This Category already exists")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpError {
    pub message: String,
    pub status: StatusCode,
}

impl HttpError {
    pub fn new(message: impl Into<String>, status: StatusCode) -> Self {
        HttpError {
            message: message.into(),
            status,
        }
    }

    pub fn server_error(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: StatusCode::BAD_REQUEST,
        }
    }

    pub fn unique_constraint_violated(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: StatusCode::CONFLICT,
        }
    }

    pub fn resource_not_found(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: StatusCode::NOT_FOUND,
        }
    }

    pub fn into_http_response(self) -> Response {
        let json_response = Json(ErrorResponse {
            status: "fail".to_string(),
            message: self.message.clone(),
        });

        (self.status, json_response).into_response()
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HttpError: message: {}, status: {}",
            self.message, self.status
        )
    }
}

impl std::error::Error for HttpError {}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        self.into_http_response()
    }
}

pub struct AppJson<T>(pub T);

impl<T, S> FromRequest<S> for AppJson<T>
where
    T: serde::de::DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = HttpError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<T>::from_request(req, state).await {
            Ok(Json(value)) => Ok(AppJson(value)),
            Err(rejection) => match rejection {
                JsonRejection::JsonDataError(e) => Err(HttpError::bad_request(e.to_string())),
                JsonRejection::MissingJsonContentType(e) => {
                    Err(HttpError::bad_request(e.to_string()))
                }
                e => Err(HttpError::bad_request(e.to_string())),
            },
        }
    }
}
