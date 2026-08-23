pub mod schema;
pub mod queries;
pub mod seed;

use rusqlite::{Connection, Result};
use std::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;

        conn.execute_batch("PRAGMA journal_mode=WAL;")?;

        schema::create_tables(&conn)?;

        seed::seed(&conn)?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }
}
