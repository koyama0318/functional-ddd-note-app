use super::{error::UserError, id::UserId};
use serde::{Deserialize, Serialize};

// MARK: - States
#[derive(Deserialize, Debug)]
pub struct UnvalidatedUser {
    id: u64,
    name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UnvalidatedUserChanges {
    name: Option<String>,
}

#[derive(Serialize)]
pub struct User {
    id: UserId,
    pub(crate) name: String,
}

impl User {
    pub fn new(id: u64, name: String) -> Self {
        User {
            id: UserId::new(id),
            name: name,
        }
    }

    pub fn id(&self) -> u64 {
        self.id.id()
    }
}

// MARK: - Actions
pub fn validate(user: UnvalidatedUser) -> Result<User, UserError> {
    if user.name.is_empty() && user.name.contains(" ") {
        return Err(UserError::ValidationError);
    }
    Ok(User {
        id: UserId::new(user.id),
        name: user.name,
    })
}

pub fn validate_and_apply(
    user: User,
    changes: UnvalidatedUserChanges,
) -> Result<UnvalidatedUser, UserError> {
    Ok(UnvalidatedUser {
        id: user.id(),
        name: changes.name.unwrap_or(user.name),
    })
}

// MARK: - Dependency
pub trait SaveUserFn: Fn(User) -> Result<User, UserError> + Clone {}
impl<T> SaveUserFn for T where T: Fn(User) -> Result<User, UserError> + 'static + Clone {}

pub trait GetUserFn: Fn(UserId) -> Result<User, UserError> + Clone {}
impl<T> GetUserFn for T where T: Fn(UserId) -> Result<User, UserError> + 'static + Clone {}

pub trait DeleteUserFn: Fn(UserId) -> Result<(), UserError> + Clone {}
impl<T> DeleteUserFn for T where T: Fn(UserId) -> Result<(), UserError> + 'static + Clone {}
