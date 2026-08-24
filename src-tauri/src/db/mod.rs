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

        // Wrap seeding in a transaction to make first run fast on Android (avoid per-row autocommit)
        conn.execute_batch("BEGIN IMMEDIATE;")?;
        let seed_result: Result<()> = (|| {
            schema::create_tables(&conn)?;
            seed::seed(&conn)?;
            Ok(())
        })();
        match seed_result {
            Ok(_) => {
                conn.execute_batch("COMMIT;")?;
            }
            Err(e) => {
                let _ = conn.execute_batch("ROLLBACK;");
                return Err(e);
            }
        }

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }
}
