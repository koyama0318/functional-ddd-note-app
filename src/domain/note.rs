#[derive(Debug)]
pub struct NoteId {
    id: String,
}

impl NoteId {
    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn new(id: String) -> Self {
        NoteId { id: id }
    }
}

#[derive(Debug)]
pub struct UnvalidatedNote {
    id: NoteId,
}

impl UnvalidatedNote {
    pub fn new(id: NoteId) -> Self {
        UnvalidatedNote { id: id }
    }
}

#[derive(Debug)]
pub struct ValidatedNote {
    id: NoteId,
}

impl ValidatedNote {
    pub fn new(id: NoteId) -> Self {
        ValidatedNote { id: id }
    }
}

#[derive(Debug)]
pub struct CreatedNote {
    id: NoteId,
}

impl CreatedNote {
    pub fn new(id: NoteId) -> Self {
        CreatedNote { id: id }
    }
}

#[derive(Debug)]
pub enum NoteError {
    NotFound,
}
