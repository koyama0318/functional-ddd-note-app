use super::core::*;
use serde::Deserialize;

// MARK: - Commands
#[derive(Deserialize, Debug)]
pub struct CreateUserCommand {
    pub(crate) user: UnvalidatedUser,
}

#[derive(Deserialize, Debug)]
pub struct GetUserCommand {
    pub(crate) id: u64,
}

#[derive(Deserialize, Debug)]
pub struct UpdateUserCommand {
    pub(crate) id: u64,
    pub(crate) changes: UnvalidatedUserChanges,
}

#[derive(Deserialize, Debug)]
pub struct DeleteUserCommand {
    pub(crate) id: u64,
}
