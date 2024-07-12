use super::{error::UserError, id::UserId};
use serde::{Deserialize, Serialize};

// MARK: - States
#[derive(Deserialize, Debug)]
pub struct CreateUserCommand {
    pub(crate) user: UnvalidatedUser,
}

#[derive(Deserialize, Debug)]
pub struct UnvalidatedUser {
    id: u64,
    name: String,
}

#[derive(Serialize)]
pub struct User {
    id: UserId,
    name: String,
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

// MARK: - Dependency
pub trait SaveUserFn: Fn(User) -> Result<User, UserError> {}
impl<T> SaveUserFn for T where T: Fn(User) -> Result<User, UserError> + 'static {}

// MARK: - Workflows
pub trait CreateUserFn: Fn(CreateUserCommand) -> Result<User, UserError> {}
impl<T> CreateUserFn for T where T: Fn(CreateUserCommand) -> Result<User, UserError> {}

pub fn create_user_workflow<F: SaveUserFn>(save_user: F) -> impl CreateUserFn {
    move |cmd| {
        Ok(cmd.user)
            .and_then(validate)
            .and_then(|user| save_user(user))
    }
}
