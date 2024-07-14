use crate::domain::{
    note::{core::*, error::NoteError, id::NoteId},
    user::id::UserId,
};
use rusqlite::Connection;

pub fn upsert_note_fn() -> impl UpsertNoteFn {
    move |note| {
        let conn = Connection::open("database.sqlite3").expect("msg");
        let result = conn.execute(
            "INSERT INTO notes (id, user_id, content) VALUES (?1, ?2, ?3) \
            ON CONFLICT(id) DO UPDATE SET content=excluded.content",
            (&note.id, &note.user_id, &note.content),
        );
        match result {
            Ok(_) => Ok(note),
            Err(e) => Err(NoteError::DatabaseError(e.to_string())),
        }
    }
}

pub fn list_note_fn() -> impl ListNoteFn {
    move |user_id| {
        let conn = Connection::open("database.sqlite3")
            .map_err(|e| NoteError::DatabaseError(e.to_string()))?;
        let mut stmt = conn
            .prepare("SELECT id, user_id, content FROM notes WHERE user_id = ?1")
            .map_err(|e| NoteError::DatabaseError(e.to_string()))?;

        let note_iter = stmt
            .query_map([user_id.id()], |row| {
                Ok(Note::new(
                    NoteId::new(row.get(0)?).unwrap(),
                    UserId::new(row.get(1)?).unwrap(),
                    row.get(2)?,
                ))
            })
            .map_err(|e| NoteError::DatabaseError(e.to_string()))?;

        let mut notes = Vec::new();
        for note in note_iter {
            notes.push(note.map_err(|e| NoteError::DatabaseError(e.to_string()))?);
        }

        Ok(notes)
    }
}

pub fn get_note_fn() -> impl GetNoteFn {
    move |id| {
        let conn = Connection::open("database.sqlite3").expect("msg");
        let mut stmt = conn
            .prepare("SELECT id, user_id, content FROM notes WHERE id = ?1")
            .map_err(|e| NoteError::DatabaseError(e.to_string()))?;

        let result = stmt
            .query_row([id.id()], |row| {
                Ok(Note::new(
                    NoteId::new(row.get(0)?).unwrap(),
                    UserId::new(row.get(1)?).unwrap(),
                    row.get(2)?,
                ))
            })
            .map_err(|e| NoteError::DatabaseError(e.to_string()));

        result
    }
}

pub fn delete_note_fn() -> impl DeleteNoteFn {
    move |id| {
        let conn = Connection::open("database.sqlite3").expect("msg");
        let result = conn.execute("DELETE FROM notes WHERE id = ?1", [id.id()]);
        match result {
            Ok(_) => Ok(id),
            Err(e) => Err(NoteError::DatabaseError(e.to_string())),
        }
    }
}

pub fn exsits_user_fn() -> impl ExistsUserFn {
    move |user_id| {
        let conn = Connection::open("database.sqlite3").expect("msg");
        let mut stmt = conn
            .prepare("SELECT id FROM users WHERE id = ?1 LIMIT 1")
            .map_err(|e| NoteError::DatabaseError(e.to_string()))?;

        let result = stmt
            .query_row([user_id.id()], |row| row.get::<_, String>(0))
            .map(|_| ())
            .map_err(|_| NoteError::UserNotExits);

        result
    }
}
