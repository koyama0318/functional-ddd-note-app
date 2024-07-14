use crate::{
    adaptor::gateways::note_repository::{delete_note_fn, get_note_fn, get_notes_fn, save_note_fn},
    domain::note::{
        core::{
            create_note_workflow, delete_note_workflow, get_note_workflow, list_notes_workflow,
            update_note_workflow, CreateNoteCommand, DeleteNoteCommand, GetNoteCommand,
            ListNoteCommand, UnvalidatedNote, UnvalidatedNoteChanges, UpdateNoteCommand,
        },
        error::NoteError,
    },
};
use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};

use super::ErrorResponse;

pub async fn create_note(Json(input): Json<UnvalidatedNote>) -> impl IntoResponse {
    let cmd = CreateNoteCommand { note: input };
    let save_note_fn = save_note_fn();
    let workflow = Box::new(create_note_workflow(save_note_fn));
    let note = workflow(cmd).map_err(handle_error);
    (StatusCode::CREATED, Json(note))
}

pub async fn list_notes() -> impl IntoResponse {
    let cmd = ListNoteCommand {};
    let get_notes_fn = get_notes_fn();
    let workflow = Box::new(list_notes_workflow(get_notes_fn));
    let notes = workflow(cmd).map_err(handle_error);
    (StatusCode::OK, Json(notes))
}

pub async fn get_note(Path(id): Path<u64>) -> impl IntoResponse {
    let cmd = GetNoteCommand { id };
    let get_note_fn = get_note_fn();
    let workflow = Box::new(get_note_workflow(get_note_fn));
    let note = workflow(cmd).map_err(handle_error);
    (StatusCode::OK, Json(note))
}

pub async fn update_note(
    Path(id): Path<u64>,
    Json(input): Json<UnvalidatedNoteChanges>,
) -> impl IntoResponse {
    let cmd = UpdateNoteCommand { id, changes: input };
    let get_note_fn = get_note_fn();
    let save_note_fn = save_note_fn();
    let workflow = Box::new(update_note_workflow(get_note_fn, save_note_fn));
    let note = workflow(cmd).map_err(handle_error);
    (StatusCode::OK, Json(note))
}

pub async fn delete_note(Path(id): Path<u64>) -> impl IntoResponse {
    let cmd = DeleteNoteCommand { id };
    let delete_note_fn = delete_note_fn();
    let workflow = Box::new(delete_note_workflow(delete_note_fn));
    let note = workflow(cmd).map_err(handle_error);
    (StatusCode::NO_CONTENT, Json(note))
}

fn handle_error(e: NoteError) -> ErrorResponse {
    match e {
        NoteError::ValidationError => ErrorResponse::new(StatusCode::BAD_REQUEST, "Invalid input"),
    }
}
