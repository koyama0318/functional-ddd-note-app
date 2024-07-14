use crate::domain::note::core::{DeleteNoteFn, GetNoteFn, GetNotesFn, Note, SaveNoteFn};
use rusqlite::Connection;

pub fn save_note_fn() -> impl SaveNoteFn {
    move |note| {
        let conn = Connection::open("database.sqlite3").expect("msg");
        let result = conn.execute(
            "INSERT INTO notes (id, name) VALUES (?1, ?2)",
            (&note.id(), &note.name),
        );
        Ok(note)
    }
}

pub fn get_notes_fn() -> impl GetNotesFn {
    move || {
        let conn = Connection::open("database.sqlite3").expect("msg");
        let mut stmt = conn.prepare("SELECT id, name FROM notes").unwrap();
        let notes = stmt
            .query_map([], |row| Ok(Note::new(row.get(0)?, row.get(1)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        Ok(notes)
    }
}

pub fn get_note_fn() -> impl GetNoteFn {
    move |id| {
        let conn = Connection::open("database.sqlite3").expect("msg");
        let mut stmt = conn
            .prepare("SELECT id, name FROM notes WHERE id = ?1")
            .unwrap();
        let note = stmt
            .query_row([id.id()], |row| Ok(Note::new(row.get(0)?, row.get(1)?)))
            .unwrap();
        Ok(note)
    }
}

pub fn delete_note_fn() -> impl DeleteNoteFn {
    move |id| {
        let conn = Connection::open("database.sqlite3").expect("msg");
        let result = conn.execute("DELETE FROM notes WHERE id = ?1", [id.id()]);
        Ok(())
    }
}
