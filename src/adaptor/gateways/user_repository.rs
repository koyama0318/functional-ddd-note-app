use crate::domain::user::core::{DeleteUserFn, GetUserFn, GetUsersFn, SaveUserFn, User};
use rusqlite::Connection;

pub fn save_user_fn() -> impl SaveUserFn {
    move |user| {
        let conn = Connection::open("database.sqlite3").expect("msg");
        let result = conn.execute(
            "INSERT INTO users (id, name) VALUES (?1, ?2)",
            (&user.id(), &user.name),
        );
        println!("result: {:?}", result);
        Ok(user)
    }
}

pub fn get_users_fn() -> impl GetUsersFn {
    move || {
        let conn = Connection::open("database.sqlite3").expect("msg");
        let mut stmt = conn.prepare("SELECT id, name FROM users").unwrap();
        let users = stmt
            .query_map([], |row| Ok(User::new(row.get(0)?, row.get(1)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        Ok(users)
    }
}

pub fn get_user_fn() -> impl GetUserFn {
    move |id| {
        let conn = Connection::open("database.sqlite3").expect("msg");
        let mut stmt = conn
            .prepare("SELECT id, name FROM users WHERE id = ?1")
            .unwrap();
        let user = stmt
            .query_row([id.id()], |row| Ok(User::new(row.get(0)?, row.get(1)?)))
            .unwrap();
        Ok(user)
    }
}

pub fn delete_user_fn() -> impl DeleteUserFn {
    move |id| {
        let conn = Connection::open("database.sqlite3").expect("msg");
        let result = conn.execute("DELETE FROM users WHERE id = ?1", [id.id()]);
        println!("result: {:?}", result);
        Ok(())
    }
}
