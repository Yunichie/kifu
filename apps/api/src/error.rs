use std::fmt::Display;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use domain::types::ErrorResponse;

pub enum ApiError {
    BadRequest(String),
    Unauthorized,
    NotFound(String),
    Unprocessable,
    BadGateway(String),
    Internal,
}

impl ApiError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::BadRequest(message.into())
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound(message.into())
    }

    pub fn unprocessable(error: impl Display) -> Self {
        worker::console_error!("invalid Tenhou log: {error}");
        Self::Unprocessable
    }

    pub fn bad_gateway(error: impl Display) -> Self {
        worker::console_error!("upstream request failed: {error}");
        Self::BadGateway("Tenhou log could not be fetched".into())
    }

    pub fn upstream(message: impl Into<String>, error: impl Display) -> Self {
        worker::console_error!("upstream request failed: {error}");
        Self::BadGateway(message.into())
    }

    pub fn internal(error: impl Display) -> Self {
        worker::console_error!("internal API error: {error}");
        Self::Internal
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error) = match self {
            Self::BadRequest(error) => (StatusCode::BAD_REQUEST, error),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized".into()),
            Self::NotFound(error) => (StatusCode::NOT_FOUND, error),
            Self::Unprocessable => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "Tenhou log could not be parsed".into(),
            ),
            Self::BadGateway(error) => (StatusCode::BAD_GATEWAY, error),
            Self::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error".into(),
            ),
        };

        (status, Json(ErrorResponse { error })).into_response()
    }
}
