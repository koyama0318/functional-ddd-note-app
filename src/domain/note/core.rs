use super::{
    error::NoteError,
    id::{note_id, NoteId},
};
use serde::{Deserialize, Serialize};

// MARK: - Commands
#[derive(Deserialize, Debug)]
pub struct CreateNoteCommand {
    pub(crate) note: UnvalidatedNote,
}

#[derive(Deserialize, Debug)]
pub struct ListNoteCommand {}

#[derive(Deserialize, Debug)]
pub struct GetNoteCommand {
    pub(crate) id: u64,
}

#[derive(Deserialize, Debug)]
pub struct UpdateNoteCommand {
    pub(crate) id: u64,
    pub(crate) changes: UnvalidatedNoteChanges,
}

#[derive(Deserialize, Debug)]
pub struct DeleteNoteCommand {
    pub(crate) id: u64,
}

// MARK: - States
#[derive(Deserialize, Debug)]
pub struct UnvalidatedNote {
    id: u64,
    name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UnvalidatedNoteChanges {
    name: Option<String>,
}

#[derive(Serialize)]
pub struct Note {
    id: NoteId,
    pub(crate) name: String,
}

impl Note {
    pub fn new(id: u64, name: String) -> Self {
        Note {
            id: NoteId::new(id),
            name: name,
        }
    }

    pub fn id(&self) -> u64 {
        self.id.id()
    }
}

// MARK: - Actions
fn validate(note: UnvalidatedNote) -> Result<Note, NoteError> {
    if note.name.is_empty() && note.name.contains(" ") {
        return Err(NoteError::ValidationError);
    }
    Ok(Note {
        id: NoteId::new(note.id),
        name: note.name,
    })
}

fn validate_and_apply(
    note: Note,
    changes: UnvalidatedNoteChanges,
) -> Result<UnvalidatedNote, NoteError> {
    Ok(UnvalidatedNote {
        id: note.id(),
        name: changes.name.unwrap_or(note.name),
    })
}

// MARK: - Dependency
pub trait SaveNoteFn: Fn(Note) -> Result<Note, NoteError> + Clone {}
impl<T> SaveNoteFn for T where T: Fn(Note) -> Result<Note, NoteError> + 'static + Clone {}

pub trait GetNotesFn: Fn() -> Result<Vec<Note>, NoteError> + Clone {}
impl<T> GetNotesFn for T where T: Fn() -> Result<Vec<Note>, NoteError> + 'static + Clone {}

pub trait GetNoteFn: Fn(NoteId) -> Result<Note, NoteError> + Clone {}
impl<T> GetNoteFn for T where T: Fn(NoteId) -> Result<Note, NoteError> + 'static + Clone {}

pub trait DeleteNoteFn: Fn(NoteId) -> Result<(), NoteError> + Clone {}
impl<T> DeleteNoteFn for T where T: Fn(NoteId) -> Result<(), NoteError> + 'static + Clone {}

// MARK: - Workflows
pub trait CreateNoteWorkflow: Fn(CreateNoteCommand) -> Result<Note, NoteError> {}
impl<T> CreateNoteWorkflow for T where T: Fn(CreateNoteCommand) -> Result<Note, NoteError> {}

pub trait ListNoteWorkflow: Fn(ListNoteCommand) -> Result<Vec<Note>, NoteError> {}
impl<T> ListNoteWorkflow for T where T: Fn(ListNoteCommand) -> Result<Vec<Note>, NoteError> {}

pub trait GetNoteWorkflow: Fn(GetNoteCommand) -> Result<Note, NoteError> {}
impl<T> GetNoteWorkflow for T where T: Fn(GetNoteCommand) -> Result<Note, NoteError> {}

pub trait UpdateNoteWorkflow: Fn(UpdateNoteCommand) -> Result<Note, NoteError> {}
impl<T> UpdateNoteWorkflow for T where T: Fn(UpdateNoteCommand) -> Result<Note, NoteError> {}

pub trait DeleteNoteWorkflow: Fn(DeleteNoteCommand) -> Result<(), NoteError> {}
impl<T> DeleteNoteWorkflow for T where T: Fn(DeleteNoteCommand) -> Result<(), NoteError> {}

pub fn create_note_workflow<F: SaveNoteFn>(save_note: F) -> impl CreateNoteWorkflow {
    move |cmd| Ok(cmd.note).and_then(validate).and_then(save_note.clone())
}

pub fn list_notes_workflow<F: GetNotesFn>(get_notes: F) -> impl ListNoteWorkflow {
    move |cmd| Ok(cmd).and_then(|_| get_notes())
}

pub fn get_note_workflow<F: GetNoteFn>(get_note: F) -> impl GetNoteWorkflow {
    move |cmd| Ok(cmd.id).and_then(note_id).and_then(get_note.clone())
}

pub fn update_note_workflow<F1: GetNoteFn, F2: SaveNoteFn>(
    get_note: F1,
    save_note: F2,
) -> impl UpdateNoteWorkflow {
    move |cmd| {
        let changes = cmd.changes.clone();
        Ok(cmd.id)
            .and_then(note_id)
            .and_then(get_note.clone())
            .and_then(|note| validate_and_apply(note, changes))
            .and_then(validate)
            .and_then(save_note.clone())
    }
}

pub fn delete_note_workflow<F: DeleteNoteFn>(delete_note: F) -> impl DeleteNoteWorkflow {
    move |cmd| Ok(cmd.id).and_then(note_id).and_then(delete_note.clone())
}
