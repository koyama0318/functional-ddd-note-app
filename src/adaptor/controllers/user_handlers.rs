use crate::adaptor::gateways::user_repository::{
    delete_user_fn, get_user_fn, get_users_fn, save_user_fn,
};
use crate::domain::user::core::{
    create_user_workflow, delete_user_workflow, get_user_workflow, list_users_workflow,
    update_user_workflow, CreateUserCommand, DeleteUserCommand, GetUserCommand, ListUserCommand,
    UnvalidatedUser, UnvalidatedUserChanges, UpdateUserCommand,
};
use crate::domain::user::error::UserError;
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

pub async fn list_users() -> impl IntoResponse {
    let cmd = ListUserCommand {};
    let get_users_fn = get_users_fn();
    let workflow = Box::new(list_users_workflow(get_users_fn));
    let users = workflow(cmd).map_err(handle_error);
    (StatusCode::OK, Json(users))
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
    }
}
