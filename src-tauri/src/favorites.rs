//! Favorite entities per game (spec `006-favorites-system`).
//!
//! Storage mirrors `builds.rs`: a single JSON array at
//! `{app_data_dir}/favorites.json` (Tauri app-data, atomic temp+rename
//! writes). Keys are `(game_id, kind, id)`; `name` is a display snapshot
//! taken at toggle time (routes resolve by id, so renames only stale the
//! label, never the link).

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const STORE_FILE: &str = "favorites.json";
const MAX_NAME_LEN: usize = 120;

pub const FAVORITE_KINDS: [&str; 8] = [
    "monster",
    "weapon",
    "armor",
    "armor_set",
    "quest",
    "item",
    "skill",
    "decoration",
];

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Favorite {
    pub game_id: i32,
    pub kind: String,
    pub id: i32,
    pub name: String,
    pub created_at: i64,
}

fn now_secs() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

pub fn store_path(app_data_dir: &std::path::Path) -> PathBuf {
    app_data_dir.join(STORE_FILE)
}

fn read_all(path: &std::path::Path) -> Result<Vec<Favorite>, String> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let raw = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    if raw.trim().is_empty() {
        return Ok(Vec::new());
    }
    serde_json::from_str::<Vec<Favorite>>(&raw)
        .map_err(|e| format!("favorites store corrupted: {e}"))
}

fn write_all(path: &std::path::Path, favs: &[Favorite]) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let raw = serde_json::to_string_pretty(favs).map_err(|e| e.to_string())?;
    let tmp = path.with_extension("json.tmp");
    std::fs::write(&tmp, raw).map_err(|e| e.to_string())?;
    std::fs::rename(&tmp, path).map_err(|e| e.to_string())?;
    Ok(())
}

fn validate(game_id: i32, kind: &str, id: i32, name: &str) -> Result<(), String> {
    if !(1..=5).contains(&game_id) {
        return Err(format!("unknown game_id {game_id}"));
    }
    if !FAVORITE_KINDS.contains(&kind) {
        return Err(format!("unknown favorite kind {kind}"));
    }
    if id <= 0 {
        return Err(format!("invalid favorite id {id}"));
    }
    let name = name.trim();
    if name.is_empty() {
        return Err("favorite name must not be empty".to_string());
    }
    if name.chars().count() > MAX_NAME_LEN {
        return Err(format!("favorite name exceeds {MAX_NAME_LEN} chars"));
    }
    Ok(())
}

pub fn list_favorites(
    app_data_dir: &std::path::Path,
    game_id: Option<i32>,
) -> Result<Vec<Favorite>, String> {
    let mut favs = read_all(&store_path(app_data_dir))?;
    if let Some(g) = game_id {
        favs.retain(|f| f.game_id == g);
    }
    favs.sort_by(|a, b| b.created_at.cmp(&a.created_at).then(a.name.cmp(&b.name)));
    Ok(favs)
}

/// Toggle: returns `true` when the entry is now favorited.
pub fn toggle_favorite(
    app_data_dir: &std::path::Path,
    game_id: i32,
    kind: String,
    id: i32,
    name: String,
) -> Result<bool, String> {
    validate(game_id, &kind, id, &name)?;
    let mut favs = read_all(&store_path(app_data_dir))?;
    if let Some(pos) = favs
        .iter()
        .position(|f| f.game_id == game_id && f.kind == kind && f.id == id)
    {
        favs.remove(pos);
        write_all(&store_path(app_data_dir), &favs)?;
        return Ok(false);
    }
    favs.push(Favorite {
        game_id,
        kind,
        id,
        name: name.trim().to_string(),
        created_at: now_secs(),
    });
    write_all(&store_path(app_data_dir), &favs)?;
    Ok(true)
}

pub fn remove_favorite(
    app_data_dir: &std::path::Path,
    game_id: i32,
    kind: &str,
    id: i32,
) -> Result<bool, String> {
    let mut favs = read_all(&store_path(app_data_dir))?;
    let before = favs.len();
    favs.retain(|f| !(f.game_id == game_id && f.kind == kind && f.id == id));
    if favs.len() == before {
        return Ok(false);
    }
    write_all(&store_path(app_data_dir), &favs)?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmpdir() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!("mh-aio-fav-test-{nanos}"));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn toggle_list_remove_round_trip() {
        let dir = tmpdir();
        assert!(list_favorites(&dir, Some(5)).unwrap().is_empty());
        assert!(toggle_favorite(&dir, 5, "monster".into(), 4, "Rathalos".into()).unwrap());
        assert!(toggle_favorite(&dir, 5, "weapon".into(), 10, "Buster Sword".into()).unwrap());
        assert!(toggle_favorite(&dir, 1, "monster".into(), 1001, "Aptonoth".into()).unwrap());
        // Per-game scoping.
        assert_eq!(list_favorites(&dir, Some(5)).unwrap().len(), 2);
        assert_eq!(list_favorites(&dir, None).unwrap().len(), 3);
        // Toggle off.
        assert!(!toggle_favorite(&dir, 5, "monster".into(), 4, "Rathalos".into()).unwrap());
        assert_eq!(list_favorites(&dir, Some(5)).unwrap().len(), 1);
        assert!(remove_favorite(&dir, 5, "weapon", 10).unwrap());
        assert!(!remove_favorite(&dir, 5, "weapon", 10).unwrap());
        assert!(list_favorites(&dir, Some(5)).unwrap().is_empty());
    }

    #[test]
    fn toggle_validates_input() {
        let dir = tmpdir();
        assert!(toggle_favorite(&dir, 42, "monster".into(), 1, "X".into()).is_err());
        assert!(toggle_favorite(&dir, 5, "boss".into(), 1, "X".into()).is_err());
        assert!(toggle_favorite(&dir, 5, "monster".into(), 0, "X".into()).is_err());
        assert!(toggle_favorite(&dir, 5, "monster".into(), 1, "  ".into()).is_err());
        assert!(list_favorites(&dir, None).unwrap().is_empty());
    }

    #[test]
    fn favorites_survive_reload() {
        let dir = tmpdir();
        toggle_favorite(&dir, 3, "item".into(), 30001, "Potion".into()).unwrap();
        // Fresh read from disk.
        let list = list_favorites(&dir, Some(3)).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].name, "Potion");
    }
}
