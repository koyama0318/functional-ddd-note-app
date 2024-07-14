use super::error::NoteError;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct NoteId {
    id: String,
}

impl NoteId {
    pub fn default() -> Self {
        NoteId {
            id: Uuid::new_v4().to_string(),
        }
    }

    pub fn new(id: String) -> Result<Self, NoteError> {
        match uuid::Uuid::parse_str(&id) {
            Ok(_) => Ok(NoteId { id }),
            Err(_) => Err(NoteError::ValidationError),
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }
}
