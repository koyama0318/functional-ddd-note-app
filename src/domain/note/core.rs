use super::{error::NoteError, id::*};
use crate::domain::user::id::UserId;
use serde::{Deserialize, Serialize};

// MARK: - States
#[derive(Deserialize, Debug)]
pub struct UnvalidatedNote {
    user_id: String,
    content: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UnvalidatedNoteChanges {
    content: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct Note {
    pub(crate) id: String,
    pub(crate) user_id: String,
    pub(crate) content: String,
}

impl Note {
    pub fn new(id: NoteId, user_id: UserId, content: String) -> Self {
        Note {
            id: id.id(),
            user_id: user_id.id(),
            content,
        }
    }
}

// MARK: - Actions
pub fn validate(note: UnvalidatedNote) -> Result<Note, NoteError> {
    let user_id = UserId::new(note.user_id).map_err(|_| NoteError::ValidationError)?;
    if note.content.len() > 1000 {
        return Err(NoteError::ValidationError);
    }
    Ok(Note::new(
        NoteId::default(),
        user_id,
        note.content.trim().to_string(),
    ))
}

pub fn validate_and_apply(note: Note, changes: UnvalidatedNoteChanges) -> Result<Note, NoteError> {
    let applyed = UnvalidatedNote {
        user_id: note.user_id.clone(),
        content: changes.content.unwrap_or(note.content),
    };
    validate(applyed).and_then(|validated| {
        Ok(Note {
            content: validated.content,
            ..note
        })
    })
}

// MARK: - Dependency
pub trait UpsertNoteFn: Fn(Note) -> Result<Note, NoteError> + Clone {}
impl<T> UpsertNoteFn for T where T: Fn(Note) -> Result<Note, NoteError> + 'static + Clone {}

pub trait ListNoteFn: Fn(UserId) -> Result<Vec<Note>, NoteError> + Clone {}
impl<T> ListNoteFn for T where T: Fn(UserId) -> Result<Vec<Note>, NoteError> + 'static + Clone {}

pub trait GetNoteFn: Fn(NoteId) -> Result<Note, NoteError> + Clone {}
impl<T> GetNoteFn for T where T: Fn(NoteId) -> Result<Note, NoteError> + 'static + Clone {}

pub trait DeleteNoteFn: Fn(NoteId) -> Result<NoteId, NoteError> + Clone {}
impl<T> DeleteNoteFn for T where T: Fn(NoteId) -> Result<NoteId, NoteError> + 'static + Clone {}

pub trait ExistsUserFn: Fn(UserId) -> Result<(), NoteError> + Clone {}
impl<T> ExistsUserFn for T where T: Fn(UserId) -> Result<(), NoteError> + 'static + Clone {}
