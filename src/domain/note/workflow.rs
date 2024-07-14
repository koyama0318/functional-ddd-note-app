use super::{command::*, core::*, error::NoteError, id::*};
use crate::domain::user::id::UserId;

// MARK: - Workflow
pub trait CreateNoteWorkflow: Fn(CreateNoteCommand) -> Result<Note, NoteError> {}
impl<T> CreateNoteWorkflow for T where T: Fn(CreateNoteCommand) -> Result<Note, NoteError> {}

pub trait ListNoteWorkflow: Fn(ListNoteCommand) -> Result<Vec<Note>, NoteError> {}
impl<T> ListNoteWorkflow for T where T: Fn(ListNoteCommand) -> Result<Vec<Note>, NoteError> {}

pub trait GetNoteWorkflow: Fn(GetNoteCommand) -> Result<Note, NoteError> {}
impl<T> GetNoteWorkflow for T where T: Fn(GetNoteCommand) -> Result<Note, NoteError> {}

pub trait UpdateNoteWorkflow: Fn(UpdateNoteCommand) -> Result<Note, NoteError> {}
impl<T> UpdateNoteWorkflow for T where T: Fn(UpdateNoteCommand) -> Result<Note, NoteError> {}

pub trait DeleteNoteWorkflow: Fn(DeleteNoteCommand) -> Result<NoteId, NoteError> {}
impl<T> DeleteNoteWorkflow for T where T: Fn(DeleteNoteCommand) -> Result<NoteId, NoteError> {}

// MARK: - Workflow factory method
pub fn create_note_workflow<F1: ExistsUserFn, F2: UpsertNoteFn>(
    exsits_user: F1,
    save_note: F2,
) -> impl CreateNoteWorkflow {
    move |cmd| {
        Ok(cmd.note)
            .and_then(validate)
            .and_then(
                |note| match exsits_user(UserId::new(note.clone().user_id).unwrap()) {
                    Ok(_) => Ok(note),
                    Err(_) => Err(NoteError::UserNotExits),
                },
            )
            .and_then(save_note.clone())
    }
}

pub fn list_note_workflow<F: ListNoteFn>(list_note: F) -> impl ListNoteWorkflow {
    move |cmd| {
        Ok(cmd.user_id)
            .and_then(UserId::new)
            .map_err(|_| NoteError::ValidationError)
            .and_then(|user_id| list_note(user_id))
    }
}

pub fn get_note_workflow<F: GetNoteFn>(get_note: F) -> impl GetNoteWorkflow {
    move |cmd| Ok(cmd.id).and_then(NoteId::new).and_then(get_note.clone())
}

pub fn update_note_workflow<F1: GetNoteFn, F2: UpsertNoteFn>(
    get_note: F1,
    save_note: F2,
) -> impl UpdateNoteWorkflow {
    move |cmd| {
        let changes = cmd.changes.clone();
        Ok(cmd.id)
            .and_then(NoteId::new)
            .and_then(get_note.clone())
            .and_then(|note| validate_and_apply(note, changes))
            .and_then(save_note.clone())
    }
}

pub fn delete_note_workflow<F: DeleteNoteFn>(delete_note: F) -> impl DeleteNoteWorkflow {
    move |cmd| {
        Ok(cmd.id)
            .and_then(NoteId::new)
            .and_then(delete_note.clone())
    }
}
