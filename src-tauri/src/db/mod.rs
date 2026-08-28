pub mod schema;
pub mod queries;
pub mod seed;

use rusqlite::functions::FunctionFlags;
use rusqlite::{Connection, OpenFlags, Result};
use std::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
}

/// Register SQLite scalar functions used by queries (kept separate so tests can
/// set up an in-memory DB identically to `Database::new`).
pub fn register_functions(conn: &Connection) -> Result<()> {
    conn.create_scalar_function(
        "norm_key",
        1,
        FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
        |ctx| {
            let s: Option<String> = ctx.get(0)?;
            Ok(s.map(|v| crate::db::queries::norm_key(&v)).unwrap_or_default())
        },
    )?;
    Ok(())
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
        // Enforce FK constraints (seed inserts parents before children).
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;

        register_functions(&conn)?;

        // Single transaction for the (now idempotent) seed — much faster on
        // Android. If BEGIN fails we fall back to auto-commit per statement; the
        // seed is safe either way because every write is INSERT OR IGNORE.
        let in_txn = conn.execute_batch("BEGIN IMMEDIATE;").is_ok();
        let setup: Result<()> = (|| {
            schema::create_tables(&conn)?;
            seed::seed(&conn)?;
            Ok(())
        })();
        match setup {
            Ok(()) => {
                if in_txn {
                    // Never swallow a failed COMMIT: leave the DB in a consistent state.
                    conn.execute_batch("COMMIT;")?;
                }
            }
            Err(e) => {
                if in_txn {
                    // Best-effort rollback; the connection is dropped right after,
                    // but an explicit ROLLBACK prevents leaving it dangling.
                    let _ = conn.execute_batch("ROLLBACK;");
                }
                return Err(e);
            }
        }

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }
}
