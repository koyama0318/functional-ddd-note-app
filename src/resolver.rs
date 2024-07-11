use crate::domain::note::{CreatedNote, NoteError, NoteId, UnvalidatedNote};
use crate::domain::workflow::{create, validate, Workflow};

// resolver
pub struct Payload {
    pub message: String,
}

impl Payload {
    pub fn new(message: String) -> Self {
        Payload { message: message }
    }
}

pub fn resolve() -> Payload {
    let note = UnvalidatedNote::new(NoteId::new("id".to_string()));

    let workflow: &Workflow = &|note: UnvalidatedNote| -> Result<CreatedNote, NoteError> {
        validate(note).and_then(create)
    };

    let result = workflow(note);

    match result {
        Ok(item) => Payload::new(format!("{:?}", item)),
        Err(error) => Payload::new(format!("{:?}", error)),
    }
}
