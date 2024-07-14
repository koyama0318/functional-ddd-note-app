use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NoteError {
    #[error("Invalid input")]
    ValidationError,
    #[error("User doesn't not exist")]
    UserNotExits,
    #[error("Database error: {0}")]
    DatabaseError(String),
}

impl NoteError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            NoteError::ValidationError => StatusCode::BAD_REQUEST,
            NoteError::UserNotExits => StatusCode::BAD_REQUEST,
            NoteError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
