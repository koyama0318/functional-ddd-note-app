use super::ErrorResponse;
use crate::adaptor::gateways::note_repository::*;
use crate::domain::note::{command::*, core::*, error::NoteError, workflow::*};
use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};

pub async fn create_note(Json(input): Json<UnvalidatedNote>) -> impl IntoResponse {
    let cmd = CreateNoteCommand { note: input };
    let exsits_user_fn = exsits_user_fn();
    let upsert_note_fn = upsert_note_fn();
    let workflow = Box::new(create_note_workflow(exsits_user_fn, upsert_note_fn));
    match workflow(cmd) {
        Ok(r) => Ok((StatusCode::CREATED, Json(r))),
        Err(e) => Err(handle_error(e)),
    }
}

pub async fn list_note(Path(user_id): Path<String>) -> impl IntoResponse {
    let cmd = ListNoteCommand { user_id };
    let list_note_fn = list_note_fn();
    let workflow = Box::new(list_note_workflow(list_note_fn));
    match workflow(cmd) {
        Ok(r) => Ok((StatusCode::OK, Json(r))),
        Err(e) => Err(handle_error(e)),
    }
}

pub async fn get_note(Path(id): Path<String>) -> impl IntoResponse {
    let cmd = GetNoteCommand { id };
    let get_note_fn = get_note_fn();
    let workflow = Box::new(get_note_workflow(get_note_fn));
    match workflow(cmd) {
        Ok(r) => Ok((StatusCode::OK, Json(r))),
        Err(e) => Err(handle_error(e)),
    }
}

pub async fn update_note(
    Path(id): Path<String>,
    Json(input): Json<UnvalidatedNoteChanges>,
) -> impl IntoResponse {
    let cmd = UpdateNoteCommand { id, changes: input };
    let get_note_fn = get_note_fn();
    let upsert_note_fn = upsert_note_fn();
    let workflow = Box::new(update_note_workflow(get_note_fn, upsert_note_fn));
    match workflow(cmd) {
        Ok(r) => Ok((StatusCode::OK, Json(r))),
        Err(e) => Err(handle_error(e)),
    }
}

pub async fn delete_note(Path(id): Path<String>) -> impl IntoResponse {
    let cmd = DeleteNoteCommand { id };
    let delete_note_fn = delete_note_fn();
    let workflow = Box::new(delete_note_workflow(delete_note_fn));
    match workflow(cmd) {
        Ok(r) => {
            println!("delete_note: {:?}", r);
            Ok((StatusCode::OK, Json(r)))
        }
        Err(e) => Err(handle_error(e)),
    }
}

fn handle_error(e: NoteError) -> ErrorResponse {
    ErrorResponse::new(e.status_code(), &e.to_string())
}
