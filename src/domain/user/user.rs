use serde::{Deserialize, Serialize};

use super::{error::UserError, id::UserId};

// MARK: - States

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
        return Err(UserError::InvalidName);
    }
    Ok(User {
        id: UserId::new(user.id),
        name: user.name,
    })
}

// MARK: - Workflows

pub type CreateUser = dyn Fn(UnvalidatedUser) -> Result<User, UserError>;

pub fn create_user_workflow() -> impl Fn(UnvalidatedUser) -> Result<User, UserError> {
    |user| validate(user)
}

// MARK: - Dependencies

// type GetUserById = fn(UserId) -> Result<User, UserError>;

// async fn get_note_by_id(ctx: Arc<Mutex<Connection>>) -> impl Fn(UserId) -> Result<User, UserError> {
//     |id| Ok(User::new(id, "name".to_string()))
// }

// async fn get_user_by_ida(context: Connection, user_id: UserId) -> Result<User, UserError> {
//     task::spawn_blocking(move || {
//         // let conn = context.lock().unwrap();
//         let mut stmt = context
//             .prepare("SELECT id, name FROM user WHERE id = ?1")
//             .unwrap();

//         let mut user_iter = stmt
//             .query_map(params![user_id.id()], |row| {
//                 Ok(User::new(
//                     UserId::new(row.get(0).unwrap_or("".to_string())),
//                     row.get(1).unwrap_or("".to_string()),
//                 ))
//             })
//             .unwrap();

//         user_iter
//             .next()
//             .transpose()
//             .unwrap()
//             .ok_or(UserError::NotFound)
//     })
//     .await
//     .unwrap()
// }
