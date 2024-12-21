use axum::{http::StatusCode, response::{IntoResponse, Response}};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    EntityNotFound(String),
    #[error("{0}")]
    ValidationError(#[from] garde::Report)
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = match self {
            Self::EntityNotFound(_) => StatusCode::NOT_FOUND,
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
        };

        status_code.into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;

