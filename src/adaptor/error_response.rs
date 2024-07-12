use axum::response::Response;
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

use crate::domain::user::error::UserError;

#[derive(Serialize)]
pub struct ErrorResponse {
    code: u16,
    message: String,
}

impl ErrorResponse {
    pub fn new(status: StatusCode, message: &str) -> Self {
        ErrorResponse {
            code: status.as_u16(),
            message: message.to_string(),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}

pub fn handle_user_error(e: UserError) -> ErrorResponse {
    match e {
        UserError::ValidationError => ErrorResponse::new(StatusCode::BAD_REQUEST, "Invalid input"),
        UserError::SaveError => {
            ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        }
    }
}
