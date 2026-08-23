pub mod schema;
pub mod queries;

use rusqlite::{Connection, Result};
use std::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        
        // Enable WAL mode for better performance
        conn.execute_batch("PRAGMA journal_mode=WAL;")?;
        
        // Create tables
        schema::create_tables(&conn)?;
        
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }
}
