use crate::adaptor::gateways::user_repository::{delete_user_fn, get_user_fn, save_user_fn};
use crate::domain::user::command::*;
use crate::domain::user::core::*;
use crate::domain::user::error::UserError;
use crate::domain::user::workflow::*;
use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};

use super::ErrorResponse;

pub async fn create_user(Json(input): Json<UnvalidatedUser>) -> impl IntoResponse {
    let cmd = CreateUserCommand { user: input };
    let save_user_fn = save_user_fn();
    let workflow = Box::new(create_user_workflow(save_user_fn));
    let user = workflow(cmd).map_err(handle_error);
    (StatusCode::CREATED, Json(user))
}

pub async fn get_user(Path(id): Path<u64>) -> impl IntoResponse {
    let cmd = GetUserCommand { id };
    let get_user_fn = get_user_fn();
    let workflow = Box::new(get_user_workflow(get_user_fn));
    let user = workflow(cmd).map_err(handle_error);
    (StatusCode::OK, Json(user))
}

pub async fn update_user(
    Path(id): Path<u64>,
    Json(input): Json<UnvalidatedUserChanges>,
) -> impl IntoResponse {
    let cmd = UpdateUserCommand { id, changes: input };
    let get_user_fn = get_user_fn();
    let save_user_fn = save_user_fn();
    let workflow = Box::new(update_user_workflow(get_user_fn, save_user_fn));
    let user = workflow(cmd).map_err(handle_error);
    (StatusCode::OK, Json(user))
}

pub async fn delete_user(Path(id): Path<u64>) -> impl IntoResponse {
    let cmd = DeleteUserCommand { id };
    let delete_user_fn = delete_user_fn();
    let workflow = Box::new(delete_user_workflow(delete_user_fn));
    let user = workflow(cmd).map_err(handle_error);
    (StatusCode::NO_CONTENT, Json(user))
}

fn handle_error(e: UserError) -> ErrorResponse {
    match e {
        UserError::ValidationError => ErrorResponse::new(StatusCode::BAD_REQUEST, "Invalid input"),
        UserError::AlreadyExists => ErrorResponse::new(StatusCode::BAD_REQUEST, "Already exists"),
    }
}
