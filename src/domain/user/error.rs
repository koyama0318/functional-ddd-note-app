use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Invalid input")]
    ValidationError,
    #[error("Database error: {0}")]
    DatabaseError(String),
}

impl UserError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            UserError::ValidationError => StatusCode::BAD_REQUEST,
            UserError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
