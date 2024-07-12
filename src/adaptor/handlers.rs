use crate::domain::user::dependency::save_user_fn;
use crate::domain::user::user::{create_user_workflow, CreateUserCommand, UnvalidatedUser};
use crate::Context;
use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::error_response::handle_user_error;

pub async fn create_user(Json(input): Json<UnvalidatedUser>) -> impl IntoResponse {
    let cmd = CreateUserCommand { user: input };
    let save_user_fn = save_user_fn(&Context::global());
    let workflow = Box::new(create_user_workflow(save_user_fn));
    let user = workflow(cmd).map_err(handle_user_error);
    (StatusCode::CREATED, Json(user))
}

pub async fn list_users() -> impl IntoResponse {
    let users = vec![
        User {
            id: 1,
            name: "Alice".to_string(),
        },
        User {
            id: 2,
            name: "Bob".to_string(),
        },
    ];
    Json(json!(users))
}

pub async fn get_user(Path(id): Path<u64>) -> impl IntoResponse {
    let user = User {
        id: id,
        name: "name".to_string(),
    };
    Json(json!(user))
}

pub async fn update_user(
    Path(id): Path<u64>,
    Json(payload): Json<CreateUserInput>,
) -> impl IntoResponse {
    let user = User {
        id: id,
        name: payload.name,
    };
    Json(user)
}

pub async fn delete_user(Path(id): Path<u64>) -> impl IntoResponse {
    StatusCode::NO_CONTENT
}

pub async fn create_note(Json(payload): Json<CreateNoteInput>) -> impl IntoResponse {
    let note = Note {
        id: payload.id,
        name: payload.name,
    };
    (StatusCode::CREATED, Json(note))
}

pub async fn list_notes() -> impl IntoResponse {
    let notes = vec![
        Note {
            id: 1,
            name: "Alice".to_string(),
        },
        Note {
            id: 2,
            name: "Bob".to_string(),
        },
    ];
    Json(notes)
}

pub async fn get_note(Path(id): Path<u64>) -> impl IntoResponse {
    let note = Note {
        id,
        name: "Alice".to_string(),
    };
    Json(note)
}

pub async fn update_note(
    Path(id): Path<u64>,
    Json(payload): Json<CreateNoteInput>,
) -> impl IntoResponse {
    let note = Note {
        id,
        name: payload.name,
    };
    Json(note)
}

pub async fn delete_note(Path(id): Path<u64>) -> impl IntoResponse {
    StatusCode::NO_CONTENT
}

// TODO: 削除
#[derive(Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Note {
    id: u64,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateUserInput {
    id: u64,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateNoteInput {
    pub id: u64,
    pub name: String,
}
