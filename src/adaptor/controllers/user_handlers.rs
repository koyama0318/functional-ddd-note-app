use crate::adaptor::gateways::user_repository::*;
use crate::domain::user::{command::*, core::*, error::UserError, workflow::*};
use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};

use super::ErrorResponse;

pub async fn create_user(Json(input): Json<UnvalidatedUser>) -> impl IntoResponse {
    let cmd = CreateUserCommand { user: input };
    let upsert_user_fn = upsert_user_fn();
    let workflow = Box::new(create_user_workflow(upsert_user_fn));
    match workflow(cmd) {
        Ok(r) => Ok((StatusCode::CREATED, Json(r))),
        Err(e) => Err(handle_error(e)),
    }
}

pub async fn get_user(Path(id): Path<String>) -> impl IntoResponse {
    let cmd = GetUserCommand { id };
    let get_user_fn = get_user_fn();
    let workflow = Box::new(get_user_workflow(get_user_fn));
    match workflow(cmd) {
        Ok(r) => Ok((StatusCode::OK, Json(r))),
        Err(e) => Err(handle_error(e)),
    }
}

pub async fn update_user(
    Path(id): Path<String>,
    Json(input): Json<UnvalidatedUserChanges>,
) -> impl IntoResponse {
    let cmd = UpdateUserCommand { id, changes: input };
    let get_user_fn = get_user_fn();
    let upsert_user_fn = upsert_user_fn();
    let workflow = Box::new(update_user_workflow(get_user_fn, upsert_user_fn));
    match workflow(cmd) {
        Ok(r) => Ok((StatusCode::OK, Json(r))),
        Err(e) => Err(handle_error(e)),
    }
}

pub async fn delete_user(Path(id): Path<String>) -> impl IntoResponse {
    let cmd = DeleteUserCommand { id };
    let delete_user_fn = delete_user_fn();
    let workflow = Box::new(delete_user_workflow(delete_user_fn));
    match workflow(cmd) {
        Ok(r) => {
            println!("delete_user: {:?}", r);
            Ok((StatusCode::OK, Json(r)))
        }
        Err(e) => Err(handle_error(e)),
    }
}

fn handle_error(e: UserError) -> ErrorResponse {
    ErrorResponse::new(e.status_code(), &e.to_string())
}
