use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};

// Dependency
pub struct DBContext {
    db_conn: Arc<Mutex<Connection>>,
}

impl DBContext {
    async fn new() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute(
            "CREATE TABLE note (
                id          INTEGER PRIMARY KEY,
                contents    TEXT NOT NULL,
            )",
            [],
        )?;
        Ok(DBContext {
            db_conn: Arc::new(Mutex::new(conn)),
        })
    }
}
