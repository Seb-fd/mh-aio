//! Saved hunter builds (spec `005-builds-save-load-export`).
//!
//! Storage is a single JSON array at `{app_data_dir}/builds.json`
//! (Tauri app-data, NOT localStorage — survives frontend cache clears and is
//! trivially backable-up). Writes are atomic (temp file + rename).
//! The solver (`ass.rs`) is untouched; this module only persists its
//! input (`AssQueryInput`) + one chosen output (`AssSolutionView`).

use crate::ass::{AssQueryInput, AssSolutionView};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub const BUILD_SCHEMA_VERSION: u32 = 1;
const STORE_FILE: &str = "builds.json";
const MAX_NAME_LEN: usize = 80;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SavedBuild {
    pub version: u32,
    pub id: String,
    pub name: String,
    pub game_id: i32,
    pub query: AssQueryInput,
    pub solution: AssSolutionView,
    pub created_at: i64,
    pub updated_at: i64,
}

fn now_secs() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

fn new_id() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("b{nanos:x}")
}

pub fn store_path(app_data_dir: &std::path::Path) -> PathBuf {
    app_data_dir.join(STORE_FILE)
}

fn read_all(path: &std::path::Path) -> Result<Vec<SavedBuild>, String> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let raw = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    if raw.trim().is_empty() {
        return Ok(Vec::new());
    }
    serde_json::from_str::<Vec<SavedBuild>>(&raw)
        .map_err(|e| format!("builds store corrupted: {e}"))
}

fn write_all(path: &std::path::Path, builds: &[SavedBuild]) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let raw = serde_json::to_string_pretty(builds).map_err(|e| e.to_string())?;
    // Atomic write: temp + rename so a crash never leaves half a file.
    let tmp = path.with_extension("json.tmp");
    std::fs::write(&tmp, raw).map_err(|e| e.to_string())?;
    std::fs::rename(&tmp, path).map_err(|e| e.to_string())?;
    Ok(())
}

fn validate(b: &SavedBuild) -> Result<(), String> {
    if b.version != BUILD_SCHEMA_VERSION {
        return Err(format!(
            "unsupported build version {} (expected {})",
            b.version, BUILD_SCHEMA_VERSION
        ));
    }
    let name = b.name.trim();
    if name.is_empty() {
        return Err("build name must not be empty".to_string());
    }
    if name.chars().count() > MAX_NAME_LEN {
        return Err(format!("build name exceeds {MAX_NAME_LEN} chars"));
    }
    if !(1..=5).contains(&b.game_id) {
        return Err(format!("unknown game_id {}", b.game_id));
    }
    if b.query.skills.is_empty() || b.query.skills.len() > 5 {
        return Err("build query must request 1..=5 skills".to_string());
    }
    if !["blade", "gunner"].contains(&b.query.hunter_type.as_str()) {
        return Err(format!("unknown hunter_type {}", b.query.hunter_type));
    }
    if !["male", "female"].contains(&b.query.gender.as_str()) {
        return Err(format!("unknown gender {}", b.query.gender));
    }
    if b.solution.armors.len() != 5 {
        return Err(format!(
            "build solution must hold 5 armor pieces, got {}",
            b.solution.armors.len()
        ));
    }
    Ok(())
}

pub fn list_builds(
    app_data_dir: &std::path::Path,
    game_id: Option<i32>,
) -> Result<Vec<SavedBuild>, String> {
    let mut builds = read_all(&store_path(app_data_dir))?;
    if let Some(g) = game_id {
        builds.retain(|b| b.game_id == g);
    }
    builds.sort_by(|a, b| b.updated_at.cmp(&a.updated_at).then(b.id.cmp(&a.id)));
    Ok(builds)
}

pub fn get_build(app_data_dir: &std::path::Path, id: &str) -> Result<Option<SavedBuild>, String> {
    Ok(read_all(&store_path(app_data_dir))?
        .into_iter()
        .find(|b| b.id == id))
}

pub fn save_build(
    app_data_dir: &std::path::Path,
    name: String,
    game_id: i32,
    query: AssQueryInput,
    solution: AssSolutionView,
) -> Result<SavedBuild, String> {
    let mut builds = read_all(&store_path(app_data_dir))?;
    let now = now_secs();
    let mut id = new_id();
    while builds.iter().any(|b| b.id == id) {
        id = new_id();
    }
    let build = SavedBuild {
        version: BUILD_SCHEMA_VERSION,
        id,
        name: name.trim().to_string(),
        game_id,
        query,
        solution,
        created_at: now,
        updated_at: now,
    };
    validate(&build)?;
    builds.push(build.clone());
    write_all(&store_path(app_data_dir), &builds)?;
    Ok(build)
}

pub fn delete_build(app_data_dir: &std::path::Path, id: &str) -> Result<bool, String> {
    let mut builds = read_all(&store_path(app_data_dir))?;
    let before = builds.len();
    builds.retain(|b| b.id != id);
    if builds.len() == before {
        return Ok(false);
    }
    write_all(&store_path(app_data_dir), &builds)?;
    Ok(true)
}

/// Validate an exported JSON blob and persist it as a NEW build (fresh id).
/// Returned build serializes back to identical content apart from
/// `id`/`created_at`/`updated_at` — the export → import round-trip check.
pub fn import_build(app_data_dir: &std::path::Path, json: &str) -> Result<SavedBuild, String> {
    let mut build: SavedBuild =
        serde_json::from_str(json).map_err(|e| format!("invalid build JSON: {e}"))?;
    validate(&build)?;
    let mut builds = read_all(&store_path(app_data_dir))?;
    let now = now_secs();
    let mut id = new_id();
    while builds.iter().any(|b| b.id == id) {
        id = new_id();
    }
    build.id = id;
    build.created_at = now;
    build.updated_at = now;
    builds.push(build.clone());
    write_all(&store_path(app_data_dir), &builds)?;
    Ok(build)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ass::{AssArmorRef, AssDecorationRef, SkillRequirement};

    fn tmpdir() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!("mh-aio-builds-test-{nanos}"));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn sample_query() -> AssQueryInput {
        AssQueryInput {
            game_id: 5,
            skills: vec![SkillRequirement {
                skill_id: 1,
                points_required: 10,
            }],
            hunter_type: "blade".to_string(),
            gender: "male".to_string(),
            hr: 9,
            elder_star: 9,
            weapon_slots: 3,
            include_piercings: true,
            allow_bad: false,
            allow_torso_inc: true,
            sort_by: None,
        }
    }

    fn sample_solution() -> AssSolutionView {
        AssSolutionView {
            armors: (0..5)
                .map(|i| AssArmorRef {
                    id: i,
                    name: format!("Piece {i}"),
                    slot_type: "head".to_string(),
                    rarity: Some(5),
                    defense_base: Some(10),
                    slots: Some("OOO".to_string()),
                    skills: None,
                })
                .collect(),
            decorations: vec![AssDecorationRef {
                id: 1,
                name: "Attack Jewel".to_string(),
                slot_size: Some(1),
                skill_name: Some("Attack".to_string()),
                skill_points: Some(1),
                secondary_skill_name: None,
                secondary_points: None,
                count: 2,
            }],
            extra_skills: vec![],
            defense: 100,
            fire_res: 0,
            water_res: 0,
            thunder_res: 0,
            ice_res: 0,
            dragon_res: 0,
            rarity: 5,
            difficulty: 1,
            slots_spare: 0,
            slots_spare_detail: vec![0, 0, 0],
        }
    }

    #[test]
    fn save_list_get_delete_round_trip() {
        let dir = tmpdir();
        let b = save_build(
            &dir,
            "My Set".to_string(),
            5,
            sample_query(),
            sample_solution(),
        )
        .unwrap();
        assert_eq!(b.version, BUILD_SCHEMA_VERSION);
        let list = list_builds(&dir, Some(5)).unwrap();
        assert_eq!(list.len(), 1);
        assert!(list_builds(&dir, Some(1)).unwrap().is_empty());
        let got = get_build(&dir, &b.id).unwrap().expect("saved build");
        assert_eq!(got.name, "My Set");
        assert!(delete_build(&dir, &b.id).unwrap());
        assert!(!delete_build(&dir, &b.id).unwrap());
        assert!(get_build(&dir, &b.id).unwrap().is_none());
    }

    #[test]
    fn save_is_stable_across_reloads() {
        let dir = tmpdir();
        save_build(&dir, "A".to_string(), 5, sample_query(), sample_solution()).unwrap();
        save_build(&dir, "B".to_string(), 5, sample_query(), sample_solution()).unwrap();
        // Re-read from disk (fresh process simulation): both survive.
        assert_eq!(list_builds(&dir, None).unwrap().len(), 2);
    }

    #[test]
    fn import_validates_and_assigns_fresh_id() {
        let dir = tmpdir();
        let b = save_build(
            &dir,
            "Orig".to_string(),
            5,
            sample_query(),
            sample_solution(),
        )
        .unwrap();
        let exported = serde_json::to_string(&b).unwrap();
        let dir2 = tmpdir();
        let imported = import_build(&dir2, &exported).unwrap();
        assert_ne!(imported.id, b.id, "import must mint a fresh id");
        // Content equality apart from identity/timestamps.
        let mut a = serde_json::to_value(&b).unwrap();
        let mut c = serde_json::to_value(&imported).unwrap();
        for v in [&mut a, &mut c] {
            v.as_object_mut().unwrap().remove("id");
            v.as_object_mut().unwrap().remove("created_at");
            v.as_object_mut().unwrap().remove("updated_at");
        }
        assert_eq!(a, c, "export → import must round-trip content");
    }

    #[test]
    fn import_rejects_garbage() {
        let dir = tmpdir();
        assert!(import_build(&dir, "not json").is_err());
        let mut b = SavedBuild {
            version: 99,
            id: "x".to_string(),
            name: "X".to_string(),
            game_id: 5,
            query: sample_query(),
            solution: sample_solution(),
            created_at: 0,
            updated_at: 0,
        };
        assert!(import_build(&dir, &serde_json::to_string(&b).unwrap()).is_err());
        b.version = 1;
        b.solution.armors.clear();
        assert!(import_build(&dir, &serde_json::to_string(&b).unwrap()).is_err());
        b.solution = sample_solution();
        b.game_id = 42;
        assert!(import_build(&dir, &serde_json::to_string(&b).unwrap()).is_err());
    }
}
