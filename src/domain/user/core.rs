use super::{error::UserError, id::*};
use serde::{Deserialize, Serialize};

// MARK: - States
#[derive(Deserialize, Debug)]
pub struct UnvalidatedUser {
    name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UnvalidatedUserChanges {
    name: Option<String>,
}

#[derive(Serialize)]
pub struct User {
    pub(crate) id: String,
    pub(crate) name: String,
}

impl User {
    pub fn new(id: UserId, name: String) -> Self {
        User { id: id.id(), name }
    }
}

// MARK: - Actions
pub fn validate(user: UnvalidatedUser) -> Result<User, UserError> {
    if user.name.is_empty() {
        return Err(UserError::ValidationError);
    }
    Ok(User::new(UserId::default(), user.name))
}

pub fn validate_and_apply(user: User, changes: UnvalidatedUserChanges) -> Result<User, UserError> {
    let applied = UnvalidatedUser {
        name: changes.name.unwrap_or(user.name),
    };
    validate(applied)
}

// MARK: - Dependency
pub trait UpsertUserFn: Fn(User) -> Result<User, UserError> + Clone {}
impl<T> UpsertUserFn for T where T: Fn(User) -> Result<User, UserError> + 'static + Clone {}

pub trait GetUserFn: Fn(UserId) -> Result<User, UserError> + Clone {}
impl<T> GetUserFn for T where T: Fn(UserId) -> Result<User, UserError> + 'static + Clone {}

pub trait DeleteUserFn: Fn(UserId) -> Result<UserId, UserError> + Clone {}
impl<T> DeleteUserFn for T where T: Fn(UserId) -> Result<UserId, UserError> + 'static + Clone {}
