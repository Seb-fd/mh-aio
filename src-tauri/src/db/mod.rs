pub mod schema;
pub mod queries;
pub mod seed;

use rusqlite::{Connection, OpenFlags, Result};
use std::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_FULL_MUTEX,
        )?;

        conn.execute_batch("PRAGMA journal_mode=WAL;")?;

        schema::create_tables(&conn)?;

        seed::seed(&conn)?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }
}
