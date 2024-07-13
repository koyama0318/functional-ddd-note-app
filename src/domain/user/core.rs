use super::{
    error::UserError,
    id::{user_id, UserId},
};
use serde::{Deserialize, Serialize};

// MARK: - Commands
#[derive(Deserialize, Debug)]
pub struct CreateUserCommand {
    pub(crate) user: UnvalidatedUser,
}

#[derive(Deserialize, Debug)]
pub struct ListUserCommand {}

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
fn validate(user: UnvalidatedUser) -> Result<User, UserError> {
    if user.name.is_empty() && user.name.contains(" ") {
        return Err(UserError::ValidationError);
    }
    Ok(User {
        id: UserId::new(user.id),
        name: user.name,
    })
}

fn validate_and_apply(
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

pub trait GetUsersFn: Fn() -> Result<Vec<User>, UserError> + Clone {}
impl<T> GetUsersFn for T where T: Fn() -> Result<Vec<User>, UserError> + 'static + Clone {}

pub trait GetUserFn: Fn(UserId) -> Result<User, UserError> + Clone {}
impl<T> GetUserFn for T where T: Fn(UserId) -> Result<User, UserError> + 'static + Clone {}

pub trait DeleteUserFn: Fn(UserId) -> Result<(), UserError> + Clone {}
impl<T> DeleteUserFn for T where T: Fn(UserId) -> Result<(), UserError> + 'static + Clone {}

// MARK: - Workflows
pub trait CreateUserWorkflow: Fn(CreateUserCommand) -> Result<User, UserError> {}
impl<T> CreateUserWorkflow for T where T: Fn(CreateUserCommand) -> Result<User, UserError> {}

pub trait ListUserWorkflow: Fn(ListUserCommand) -> Result<Vec<User>, UserError> {}
impl<T> ListUserWorkflow for T where T: Fn(ListUserCommand) -> Result<Vec<User>, UserError> {}

pub trait GetUserWorkflow: Fn(GetUserCommand) -> Result<User, UserError> {}
impl<T> GetUserWorkflow for T where T: Fn(GetUserCommand) -> Result<User, UserError> {}

pub trait UpdateUserWorkflow: Fn(UpdateUserCommand) -> Result<User, UserError> {}
impl<T> UpdateUserWorkflow for T where T: Fn(UpdateUserCommand) -> Result<User, UserError> {}

pub trait DeleteUserWorkflow: Fn(DeleteUserCommand) -> Result<(), UserError> {}
impl<T> DeleteUserWorkflow for T where T: Fn(DeleteUserCommand) -> Result<(), UserError> {}

pub fn create_user_workflow<F: SaveUserFn>(save_user: F) -> impl CreateUserWorkflow {
    move |cmd| Ok(cmd.user).and_then(validate).and_then(save_user.clone())
}

pub fn list_users_workflow<F: GetUsersFn>(get_users: F) -> impl ListUserWorkflow {
    move |cmd| Ok(cmd).and_then(|_| get_users())
}

pub fn get_user_workflow<F: GetUserFn>(get_user: F) -> impl GetUserWorkflow {
    move |cmd| Ok(cmd.id).and_then(user_id).and_then(get_user.clone())
}

pub fn update_user_workflow<F1: GetUserFn, F2: SaveUserFn>(
    get_user: F1,
    save_user: F2,
) -> impl UpdateUserWorkflow {
    move |cmd| {
        let changes = cmd.changes.clone();
        Ok(cmd.id)
            .and_then(user_id)
            .and_then(get_user.clone())
            .and_then(|user| validate_and_apply(user, changes))
            .and_then(validate)
            .and_then(save_user.clone())
    }
}

pub fn delete_user_workflow<F: DeleteUserFn>(delete_user: F) -> impl DeleteUserWorkflow {
    move |cmd| Ok(cmd.id).and_then(user_id).and_then(delete_user.clone())
}
