use crate::domain::user::{
    core::{DeleteUserFn, GetUserFn, UpsertUserFn, User},
    error::UserError,
    id::UserId,
};
use rusqlite::Connection;

pub fn upsert_user_fn() -> impl UpsertUserFn {
    move |user| {
        let conn = Connection::open("database.sqlite3").expect("msg");
        let result = conn.execute(
            "INSERT INTO users (id, name) VALUES (?1, ?2) \
            ON CONFLICT(id) DO UPDATE SET name=excluded.name",
            (&user.id, &user.name),
        );
        match result {
            Ok(_) => Ok(user),
            Err(e) => Err(UserError::DatabaseError(e.to_string())),
        }
    }
}

pub fn get_user_fn() -> impl GetUserFn {
    move |id| {
        let conn = Connection::open("database.sqlite3").expect("msg");
        let mut stmt = conn
            .prepare("SELECT id, name FROM users WHERE id = ?1")
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;
        let result = stmt
            .query_row([id.id()], |row| {
                Ok(User::new(UserId::new(row.get(0)?).unwrap(), row.get(1)?))
            })
            .map_err(|e| UserError::DatabaseError(e.to_string()));
        result
    }
}

pub fn delete_user_fn() -> impl DeleteUserFn {
    move |id| {
        let conn = Connection::open("database.sqlite3").expect("msg");
        let result = conn.execute("DELETE FROM users WHERE id = ?1", [id.id()]);
        match result {
            Ok(_) => Ok(id),
            Err(e) => Err(UserError::DatabaseError(e.to_string())),
        }
    }
}
