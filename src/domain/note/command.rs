use super::core::*;
use serde::Deserialize;

// MARK: - Commands
#[derive(Deserialize, Debug)]
pub struct CreateNoteCommand {
    pub(crate) note: UnvalidatedNote,
}

#[derive(Deserialize, Debug)]
pub struct ListNoteCommand {
    pub(crate) user_id: String,
}

#[derive(Deserialize, Debug)]
pub struct GetNoteCommand {
    pub(crate) id: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateNoteCommand {
    pub(crate) id: String,
    pub(crate) changes: UnvalidatedNoteChanges,
}

#[derive(Deserialize, Debug)]
pub struct DeleteNoteCommand {
    pub(crate) id: String,
}
