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

        // WAL is fast on desktop but can fail on some Android filesystems; fallback to DELETE
        let _ = conn.execute_batch("PRAGMA journal_mode=WAL;");
        // Use a single transaction for seeding (much faster on Android), but don't fail if BEGIN fails
        let in_txn = conn.execute_batch("BEGIN IMMEDIATE;").is_ok();
        let seed_result: Result<()> = (|| {
            schema::create_tables(&conn)?;
            seed::seed(&conn)?;
            Ok(())
        })();
        match (seed_result, in_txn) {
            (Ok(_), true) => {
                let _ = conn.execute_batch("COMMIT;");
            }
            (Ok(_), false) => {}
            (Err(e), true) => {
                let _ = conn.execute_batch("ROLLBACK;");
                return Err(e);
            }
            (Err(e), false) => return Err(e),
        }

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }
}
