use super::{command::*, core::*, error::UserError, id::*};

// MARK: - Workflow
pub trait CreateUserWorkflow: Fn(CreateUserCommand) -> Result<User, UserError> {}
impl<T> CreateUserWorkflow for T where T: Fn(CreateUserCommand) -> Result<User, UserError> {}

pub trait GetUserWorkflow: Fn(GetUserCommand) -> Result<User, UserError> {}
impl<T> GetUserWorkflow for T where T: Fn(GetUserCommand) -> Result<User, UserError> {}

pub trait UpdateUserWorkflow: Fn(UpdateUserCommand) -> Result<User, UserError> {}
impl<T> UpdateUserWorkflow for T where T: Fn(UpdateUserCommand) -> Result<User, UserError> {}

pub trait DeleteUserWorkflow: Fn(DeleteUserCommand) -> Result<UserId, UserError> {}
impl<T> DeleteUserWorkflow for T where T: Fn(DeleteUserCommand) -> Result<UserId, UserError> {}

// MARK: - Workflow factory method
pub fn create_user_workflow<F: UpsertUserFn>(save_user: F) -> impl CreateUserWorkflow {
    move |cmd| Ok(cmd.user).and_then(validate).and_then(save_user.clone())
}

pub fn get_user_workflow<F: GetUserFn>(get_user: F) -> impl GetUserWorkflow {
    move |cmd| Ok(cmd.id).and_then(UserId::new).and_then(get_user.clone())
}

pub fn update_user_workflow<F1: GetUserFn, F2: UpsertUserFn>(
    get_user: F1,
    save_user: F2,
) -> impl UpdateUserWorkflow {
    move |cmd| {
        let changes = cmd.changes.clone();
        Ok(cmd.id)
            .and_then(UserId::new)
            .and_then(get_user.clone())
            .and_then(|user| validate_and_apply(user, changes))
            .and_then(save_user.clone())
    }
}

pub fn delete_user_workflow<F: DeleteUserFn>(delete_user: F) -> impl DeleteUserWorkflow {
    move |cmd| {
        Ok(cmd.id)
            .and_then(UserId::new)
            .and_then(delete_user.clone())
    }
}
