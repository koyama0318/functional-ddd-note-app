use super::error::NoteError;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct NoteId {
    id: u64,
}

impl NoteId {
    pub fn new(id: u64) -> Self {
        NoteId { id: id }
    }

    pub fn id(&self) -> u64 {
        self.id.clone()
    }
}

pub fn note_id(id: u64) -> Result<NoteId, NoteError> {
    Ok(NoteId { id: id })
}
