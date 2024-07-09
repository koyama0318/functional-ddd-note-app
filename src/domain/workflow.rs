use super::{
    note::{CreatedNote, NoteError, NoteId, UnvalidatedNote, ValidatedNote},
    user::{User, UserError},
};
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

// Action
pub fn validate(note: UnvalidatedNote) -> Result<ValidatedNote, NoteError> {
    Ok(ValidatedNote::new(NoteId::new("".to_string())))
}

pub fn create(note: ValidatedNote) -> Result<CreatedNote, NoteError> {
    Ok(CreatedNote::new(NoteId::new("".to_string())))
}

// Workflow
pub type Workflow = dyn Fn(UnvalidatedNote) -> Result<User, UserError>;

type GetUserById = fn(NoteId) -> Result<User, UserError>;

async fn get_note_by_id(
    context: Arc<Mutex<Connection>>,
) -> impl Fn(NoteId) -> Result<User, NoteError> {
    move |id| {
        let conn = context.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT id, name, email FROM user WHERE id = ?1")
            .unwrap();

        let mut user_iter = stmt
            .query_map(params![id.id()], |row| {
                Ok(User::new(row.get(0)?, row.get(1)?))
            })
            .unwrap();

        user_iter.next().unwrap().ok_or(NoteError::NotFound)
    }
}
