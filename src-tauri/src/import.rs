//! Curator import panel backend (spec `007-import-panel-json-csv`).
//!
//! Validates arbitrary JSON row arrays against the same shapes `seed.rs`
//! consumes, reports per-row errors without touching the DB (preview), and
//! applies only valid rows inside a single `BEGIN IMMEDIATE` transaction.
//! All inserts are `INSERT OR IGNORE` over the existing UNIQUE indexes, so
//! re-applying the same file is stable (idempotent). FK violations are
//! caught pre-apply as row errors — never as SQLite failures.

use rusqlite::{Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const IMPORT_KINDS: [&str; 6] = [
    "items",
    "monsters",
    "skills",
    "decorations",
    "item_sources",
    "monster_drops",
];

const SOURCE_TYPES: [&str; 19] = [
    "gather",
    "mining",
    "bug",
    "fish",
    "shop",
    "trade",
    "farm",
    "carve",
    "capture",
    "drop",
    "break",
    "quest_reward",
    "reward",
    "melder",
    "steamworks",
    "investigation",
    "palico",
    "plunder",
    "track",
];

const DROP_METHODS: [&str; 14] = [
    "carve",
    "capture",
    "drop",
    "break",
    "reward",
    "quest_reward",
    "investigation",
    "palico",
    "plunder",
    "gather",
    "track",
    "melder",
    "shop",
    "trade",
];

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RowError {
    pub index: usize,
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImportPreview {
    pub kind: String,
    pub total: usize,
    pub valid: usize,
    pub errors: Vec<RowError>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImportResult {
    pub kind: String,
    pub inserted: usize,
    pub skipped: usize,
    pub errors: Vec<RowError>,
}

fn err(index: usize, msg: impl Into<String>) -> RowError {
    RowError {
        index,
        error: msg.into(),
    }
}

fn get_i32(v: &Value, key: &str, index: usize, required: bool) -> Result<Option<i32>, RowError> {
    match v.get(key) {
        None | Some(Value::Null) => {
            if required {
                Err(err(index, format!("missing required field '{key}'")))
            } else {
                Ok(None)
            }
        }
        Some(Value::Number(n)) => n
            .as_i64()
            .and_then(|x| i32::try_from(x).ok())
            .map(Some)
            .ok_or_else(|| err(index, format!("field '{key}' must fit in i32"))),
        Some(_) => Err(err(index, format!("field '{key}' must be an integer"))),
    }
}

fn get_str<'a>(
    v: &'a Value,
    key: &str,
    index: usize,
    required: bool,
) -> Result<Option<&'a str>, RowError> {
    match v.get(key) {
        None | Some(Value::Null) => {
            if required {
                Err(err(index, format!("missing required field '{key}'")))
            } else {
                Ok(None)
            }
        }
        Some(Value::String(s)) => Ok(Some(s.as_str())),
        Some(_) => Err(err(index, format!("field '{key}' must be a string"))),
    }
}

fn get_f64(v: &Value, key: &str, index: usize) -> Result<Option<f64>, RowError> {
    match v.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Number(n)) => n
            .as_f64()
            .map(Some)
            .ok_or_else(|| err(index, format!("field '{key}' must be a number"))),
        Some(_) => Err(err(index, format!("field '{key}' must be a number"))),
    }
}

fn exists(conn: &Connection, sql: &str, params: &[&dyn rusqlite::ToSql]) -> Result<bool, String> {
    conn.query_row(sql, params, |r| r.get::<_, i32>(0))
        .optional()
        .map(|o| o.is_some())
        .map_err(|e| e.to_string())
}

/// Validated, FK-checked row ready to insert. Carries owned params.
enum ParsedRow {
    Item {
        id: i32,
        name: String,
        category: Option<String>,
        subcategory: Option<String>,
        rarity: Option<i32>,
        sell: Option<i32>,
        buy: Option<i32>,
        carry: Option<i32>,
        description: Option<String>,
        sort: Option<i32>,
    },
    Monster {
        id: i32,
        name: String,
        species: Option<String>,
        size: Option<String>,
        description: Option<String>,
        sort: Option<i32>,
    },
    Skill {
        id: i32,
        name: String,
        description: Option<String>,
        max_level: Option<i32>,
    },
    Decoration {
        id: i32,
        name: String,
        slot: i32,
        skill_id: i32,
        skill_pts: Option<i32>,
        secondary_id: Option<i32>,
        secondary_pts: Option<i32>,
        rarity: Option<i32>,
        price: Option<i32>,
    },
    ItemSource {
        item_id: i32,
        source_type: String,
        source_id: Option<i32>,
        qmin: i32,
        qmax: i32,
        prob: Option<f64>,
        location: Option<String>,
        conditions: Option<String>,
    },
    MonsterDrop {
        monster_id: i32,
        item_id: i32,
        method: String,
        part: Option<String>,
        rank: Option<String>,
        quantity: i32,
        prob: Option<f64>,
        condition: Option<String>,
    },
}

fn non_empty(s: &str, index: usize, key: &str) -> Result<String, RowError> {
    let t = s.trim();
    if t.is_empty() {
        Err(err(index, format!("field '{key}' must not be empty")))
    } else {
        Ok(t.to_string())
    }
}

fn parse_row(
    conn: &Connection,
    game_id: i32,
    kind: &str,
    index: usize,
    v: &Value,
) -> Result<ParsedRow, RowError> {
    if !v.is_object() {
        return Err(err(index, "row must be a JSON object"));
    }
    match kind {
        "items" => {
            let id = get_i32(v, "id", index, true)?.unwrap_or(0);
            if id <= 0 {
                return Err(err(index, "field 'id' must be a positive integer"));
            }
            let name = non_empty(
                get_str(v, "name", index, true)?.unwrap_or(""),
                index,
                "name",
            )?;
            Ok(ParsedRow::Item {
                id,
                name,
                category: get_str(v, "category", index, false)?.map(str::to_string),
                subcategory: get_str(v, "subcategory", index, false)?.map(str::to_string),
                rarity: get_i32(v, "rarity", index, false)?,
                sell: get_i32(v, "sell_price", index, false)?,
                buy: get_i32(v, "buy_price", index, false)?,
                carry: get_i32(v, "carry_limit", index, false)?,
                description: get_str(v, "description", index, false)?.map(str::to_string),
                sort: get_i32(v, "sort_order", index, false)?,
            })
        }
        "monsters" => {
            let id = get_i32(v, "id", index, true)?.unwrap_or(0);
            if id <= 0 {
                return Err(err(index, "field 'id' must be a positive integer"));
            }
            let name = non_empty(
                get_str(v, "name", index, true)?.unwrap_or(""),
                index,
                "name",
            )?;
            Ok(ParsedRow::Monster {
                id,
                name,
                species: get_str(v, "species", index, false)?.map(str::to_string),
                size: get_str(v, "size", index, false)?.map(str::to_string),
                description: get_str(v, "description", index, false)?.map(str::to_string),
                sort: get_i32(v, "sort_order", index, false)?,
            })
        }
        "skills" => {
            let id = get_i32(v, "id", index, true)?.unwrap_or(0);
            if id <= 0 {
                return Err(err(index, "field 'id' must be a positive integer"));
            }
            let name = non_empty(
                get_str(v, "name", index, true)?.unwrap_or(""),
                index,
                "name",
            )?;
            Ok(ParsedRow::Skill {
                id,
                name,
                description: get_str(v, "description", index, false)?.map(str::to_string),
                max_level: get_i32(v, "max_level", index, false)?,
            })
        }
        "decorations" => {
            let id = get_i32(v, "id", index, true)?.unwrap_or(0);
            if id <= 0 {
                return Err(err(index, "field 'id' must be a positive integer"));
            }
            let name = non_empty(
                get_str(v, "name", index, true)?.unwrap_or(""),
                index,
                "name",
            )?;
            let slot = get_i32(v, "slot_size", index, true)?.unwrap_or(0);
            if !(1..=3).contains(&slot) {
                return Err(err(index, "field 'slot_size' must be 1, 2 or 3"));
            }
            let skill_id = get_i32(v, "skill_id", index, true)?.unwrap_or(0);
            if !exists(
                conn,
                "SELECT 1 FROM skills WHERE id = ?1 AND game_id = ?2",
                &[&skill_id, &game_id],
            )
            .map_err(|e| err(index, format!("db error: {e}")))?
            {
                return Err(err(
                    index,
                    format!("skill_id {skill_id} does not exist for game {game_id}"),
                ));
            }
            let secondary_id = get_i32(v, "secondary_skill_id", index, false)?;
            if let Some(sid) = secondary_id {
                if !exists(
                    conn,
                    "SELECT 1 FROM skills WHERE id = ?1 AND game_id = ?2",
                    &[&sid, &game_id],
                )
                .map_err(|e| err(index, format!("db error: {e}")))?
                {
                    return Err(err(
                        index,
                        format!("secondary_skill_id {sid} does not exist for game {game_id}"),
                    ));
                }
            }
            Ok(ParsedRow::Decoration {
                id,
                name,
                slot,
                skill_id,
                skill_pts: get_i32(v, "skill_points", index, false)?,
                secondary_id,
                secondary_pts: get_i32(v, "secondary_points", index, false)?,
                rarity: get_i32(v, "rarity", index, false)?,
                price: get_i32(v, "price", index, false)?,
            })
        }
        "item_sources" => {
            let item_id = get_i32(v, "item_id", index, true)?.unwrap_or(0);
            if !exists(
                conn,
                "SELECT 1 FROM items WHERE id = ?1 AND game_id = ?2",
                &[&item_id, &game_id],
            )
            .map_err(|e| err(index, format!("db error: {e}")))?
            {
                return Err(err(
                    index,
                    format!("item_id {item_id} does not exist for game {game_id}"),
                ));
            }
            let st = non_empty(
                get_str(v, "source_type", index, true)?.unwrap_or(""),
                index,
                "source_type",
            )?;
            if !SOURCE_TYPES.contains(&st.as_str()) {
                return Err(err(index, format!("unknown source_type '{st}'")));
            }
            let source_id = get_i32(v, "source_id", index, false)?;
            if let Some(sid) = source_id {
                let ok = match st.as_str() {
                    "carve" | "capture" | "drop" | "break" => exists(
                        conn,
                        "SELECT 1 FROM monsters WHERE id = ?1 AND game_id = ?2",
                        &[&sid, &game_id],
                    ),
                    "quest_reward" => exists(
                        conn,
                        "SELECT 1 FROM quests WHERE id = ?1 AND game_id = ?2",
                        &[&sid, &game_id],
                    ),
                    _ => Ok(true),
                }
                .map_err(|e| err(index, format!("db error: {e}")))?;
                if !ok {
                    return Err(err(
                        index,
                        format!("source_id {sid} does not exist for '{st}' in game {game_id}"),
                    ));
                }
            }
            let prob = get_f64(v, "probability", index)?;
            if let Some(p) = prob {
                if !(0.0..=1.0).contains(&p) {
                    return Err(err(index, "field 'probability' must be within 0..=1"));
                }
            }
            Ok(ParsedRow::ItemSource {
                item_id,
                source_type: st,
                source_id,
                qmin: get_i32(v, "quantity_min", index, false)?.unwrap_or(1),
                qmax: get_i32(v, "quantity_max", index, false)?.unwrap_or(1),
                prob,
                location: get_str(v, "location", index, false)?.map(str::to_string),
                conditions: get_str(v, "conditions", index, false)?.map(str::to_string),
            })
        }
        "monster_drops" => {
            let monster_id = get_i32(v, "monster_id", index, true)?.unwrap_or(0);
            if !exists(
                conn,
                "SELECT 1 FROM monsters WHERE id = ?1 AND game_id = ?2",
                &[&monster_id, &game_id],
            )
            .map_err(|e| err(index, format!("db error: {e}")))?
            {
                return Err(err(
                    index,
                    format!("monster_id {monster_id} does not exist for game {game_id}"),
                ));
            }
            let item_id = get_i32(v, "item_id", index, true)?.unwrap_or(0);
            if !exists(
                conn,
                "SELECT 1 FROM items WHERE id = ?1 AND game_id = ?2",
                &[&item_id, &game_id],
            )
            .map_err(|e| err(index, format!("db error: {e}")))?
            {
                return Err(err(
                    index,
                    format!("item_id {item_id} does not exist for game {game_id}"),
                ));
            }
            let method = non_empty(
                get_str(v, "method", index, true)?.unwrap_or(""),
                index,
                "method",
            )?;
            if !DROP_METHODS.contains(&method.as_str()) {
                return Err(err(index, format!("unknown method '{method}'")));
            }
            let quantity = get_i32(v, "quantity", index, true)?.unwrap_or(0);
            if quantity <= 0 {
                return Err(err(index, "field 'quantity' must be a positive integer"));
            }
            let prob = get_f64(v, "probability", index)?;
            if let Some(p) = prob {
                if !(0.0..=1.0).contains(&p) {
                    return Err(err(index, "field 'probability' must be within 0..=1"));
                }
            }
            Ok(ParsedRow::MonsterDrop {
                monster_id,
                item_id,
                method,
                part: get_str(v, "part", index, false)?.map(str::to_string),
                rank: get_str(v, "rank", index, false)?.map(str::to_string),
                quantity,
                prob,
                condition: get_str(v, "condition", index, false)?.map(str::to_string),
            })
        }
        _ => Err(err(index, format!("unknown import kind '{kind}'"))),
    }
}

fn check_kind(kind: &str) -> Result<(), String> {
    if IMPORT_KINDS.contains(&kind) {
        Ok(())
    } else {
        Err(format!(
            "unknown import kind '{kind}' (expected one of {})",
            IMPORT_KINDS.join(", ")
        ))
    }
}

fn check_game(game_id: i32) -> Result<(), String> {
    if (1..=5).contains(&game_id) {
        Ok(())
    } else {
        Err(format!("unknown game_id {game_id}"))
    }
}

fn parse_all(
    conn: &Connection,
    game_id: i32,
    kind: &str,
    rows: &[Value],
) -> (Vec<ParsedRow>, Vec<RowError>) {
    let mut valid = Vec::new();
    let mut errors = Vec::new();
    for (index, v) in rows.iter().enumerate() {
        match parse_row(conn, game_id, kind, index, v) {
            Ok(r) => valid.push(r),
            Err(e) => errors.push(e),
        }
    }
    (valid, errors)
}

pub fn preview_import(
    conn: &Connection,
    game_id: i32,
    kind: &str,
    rows: &[Value],
) -> Result<ImportPreview, String> {
    check_game(game_id)?;
    check_kind(kind)?;
    let (valid, errors) = parse_all(conn, game_id, kind, rows);
    Ok(ImportPreview {
        kind: kind.to_string(),
        total: rows.len(),
        valid: valid.len(),
        errors,
    })
}

fn insert_row(conn: &Connection, game_id: i32, row: &ParsedRow) -> Result<usize, String> {
    let changed = match row {
        ParsedRow::Item { id, name, category, subcategory, rarity, sell, buy, carry, description, sort } => conn.execute(
            "INSERT OR IGNORE INTO items (id, game_id, name, category, subcategory, rarity, sell_price, buy_price, carry_limit, description, sort_order, language) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, 'en')",
            rusqlite::params![id, game_id, name, category, subcategory, rarity, sell, buy, carry, description, sort],
        ),
        ParsedRow::Monster { id, name, species, size, description, sort } => conn.execute(
            "INSERT OR IGNORE INTO monsters (id, game_id, name, species, size, description, sort_order, language) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'en')",
            rusqlite::params![id, game_id, name, species, size, description, sort],
        ),
        ParsedRow::Skill { id, name, description, max_level } => conn.execute(
            "INSERT OR IGNORE INTO skills (id, game_id, name, description, max_level, language) VALUES (?1, ?2, ?3, ?4, ?5, 'en')",
            rusqlite::params![id, game_id, name, description, max_level],
        ),
        ParsedRow::Decoration { id, name, slot, skill_id, skill_pts, secondary_id, secondary_pts, rarity, price } => conn.execute(
            "INSERT OR IGNORE INTO decorations (id, game_id, name, skill_id, skill_points, secondary_skill_id, secondary_points, slot_size, rarity, price, language) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 'en')",
            rusqlite::params![id, game_id, name, skill_id, skill_pts, secondary_id, secondary_pts, slot, rarity, price],
        ),
        ParsedRow::ItemSource { item_id, source_type, source_id, qmin, qmax, prob, location, conditions } => conn.execute(
            "INSERT OR IGNORE INTO item_sources (item_id, source_type, source_id, quantity_min, quantity_max, probability, location, conditions) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![item_id, source_type, source_id, qmin, qmax, prob, location, conditions],
        ),
        ParsedRow::MonsterDrop { monster_id, item_id, method, part, rank, quantity, prob, condition } => conn.execute(
            "INSERT OR IGNORE INTO monster_drops (monster_id, item_id, method, part, rank, quantity, probability, condition, language) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 'en')",
            rusqlite::params![monster_id, item_id, method, part, rank, quantity, prob, condition],
        ),
    }
    .map_err(|e| e.to_string())?;
    Ok(changed)
}

pub fn apply_import(
    conn: &Connection,
    game_id: i32,
    kind: &str,
    rows: &[Value],
) -> Result<ImportResult, String> {
    check_game(game_id)?;
    check_kind(kind)?;
    // Validate everything first: invalid rows are reported, never applied.
    let (valid, errors) = parse_all(conn, game_id, kind, rows);
    conn.execute_batch("BEGIN IMMEDIATE")
        .map_err(|e| e.to_string())?;
    let mut inserted = 0usize;
    let mut skipped = 0usize;
    let mut result: Result<(), String> = Ok(());
    for row in &valid {
        match insert_row(conn, game_id, row) {
            Ok(0) => skipped += 1,
            Ok(_) => inserted += 1,
            Err(e) => {
                result = Err(e);
                break;
            }
        }
    }
    match result {
        Ok(()) => {
            conn.execute_batch("COMMIT").map_err(|e| e.to_string())?;
            Ok(ImportResult {
                kind: kind.to_string(),
                inserted,
                skipped,
                errors,
            })
        }
        Err(e) => {
            let _ = conn.execute_batch("ROLLBACK");
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn testdb() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        crate::db::schema::create_tables(&conn).unwrap();
        conn.execute(
            "INSERT OR IGNORE INTO games (id, name, abbreviation) VALUES (5, 'MH2G', 'MH2G')",
            [],
        )
        .unwrap();
        conn
    }

    #[test]
    fn preview_accepts_valid_items_and_flags_bad_rows() {
        let c = testdb();
        let rows = vec![
            json!({"id": 9001, "name": "Test Potion", "category": "Consumable", "rarity": 1}),
            json!({"name": "Missing Id"}),
            json!({"id": "xx", "name": "Bad Id"}),
        ];
        let p = preview_import(&c, 5, "items", &rows).unwrap();
        assert_eq!(p.total, 3);
        assert_eq!(p.valid, 1);
        assert_eq!(p.errors.len(), 2);
        // Preview writes nothing.
        let n: i64 = c
            .query_row("SELECT COUNT(*) FROM items", [], |r| r.get(0))
            .unwrap();
        assert_eq!(n, 0);
    }

    #[test]
    fn preview_rejects_unknown_kind_and_game() {
        let c = testdb();
        assert!(preview_import(&c, 5, "weapons", &[]).is_err());
        assert!(preview_import(&c, 42, "items", &[]).is_err());
    }

    #[test]
    fn apply_inserts_and_is_idempotent() {
        let c = testdb();
        let rows = vec![
            json!({"id": 9001, "name": "Test Potion", "category": "Consumable"}),
            json!({"id": 999, "name": "Test Monster", "species": "Test"}),
        ];
        let r1 = apply_import(&c, 5, "items", &rows[..1]).unwrap();
        assert_eq!((r1.inserted, r1.skipped), (1, 0));
        assert!(r1.errors.is_empty());
        // Re-apply: stable counts (INSERT OR IGNORE conflict).
        let r2 = apply_import(&c, 5, "items", &rows[..1]).unwrap();
        assert_eq!((r2.inserted, r2.skipped), (0, 1));
        // Wrong-shape rows are rejected, not applied.
        let not_items = vec![json!({"monster_id": 999, "species": "Test"})];
        let r3 = apply_import(&c, 5, "items", &not_items).unwrap();
        assert_eq!(r3.inserted, 0);
        assert_eq!(r3.errors.len(), 1);
    }

    #[test]
    fn apply_catches_fk_violations() {
        let c = testdb();
        // item_sources pointing at a missing item.
        let bad = vec![json!({"item_id": 4242, "source_type": "shop", "location": "Nowhere"})];
        let p = preview_import(&c, 5, "item_sources", &bad).unwrap();
        assert_eq!(p.valid, 0);
        assert!(p.errors[0].error.contains("does not exist"));
        let r = apply_import(&c, 5, "item_sources", &bad).unwrap();
        assert_eq!(r.inserted, 0);
        // monster_drops with unknown method.
        c.execute(
            "INSERT OR IGNORE INTO items (id, game_id, name, language) VALUES (9001, 5, 'X', 'en')",
            [],
        )
        .unwrap();
        c.execute(
            "INSERT OR IGNORE INTO monsters (id, game_id, name, language) VALUES (901, 5, 'Y', 'en')",
            [],
        )
        .unwrap();
        let bad2 =
            vec![json!({"monster_id": 901, "item_id": 9001, "method": "teleport", "quantity": 1})];
        let p2 = preview_import(&c, 5, "monster_drops", &bad2).unwrap();
        assert_eq!(p2.valid, 0);
        // ...while a valid drop applies.
        let good = vec![
            json!({"monster_id": 901, "item_id": 9001, "method": "carve", "quantity": 1, "probability": 0.5}),
        ];
        let r2 = apply_import(&c, 5, "monster_drops", &good).unwrap();
        assert_eq!((r2.inserted, r2.skipped), (1, 0));
    }

    #[test]
    fn decorations_require_resolving_skills() {
        let c = testdb();
        c.execute(
            "INSERT OR IGNORE INTO skills (id, game_id, name, language) VALUES (700, 5, 'Attack', 'en')",
            [],
        )
        .unwrap();
        let rows = vec![
            json!({"id": 9501, "name": "Attack Jewel", "slot_size": 1, "skill_id": 700, "skill_points": 1}),
            json!({"id": 9502, "name": "Ghost Jewel", "slot_size": 5, "skill_id": 701}),
        ];
        let p = preview_import(&c, 5, "decorations", &rows).unwrap();
        assert_eq!(p.valid, 1);
        assert_eq!(p.errors.len(), 1);
        let r = apply_import(&c, 5, "decorations", &rows).unwrap();
        assert_eq!((r.inserted, r.skipped), (1, 0));
    }
}
