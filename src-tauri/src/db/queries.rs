use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub abbreviation: String,
    pub release_year: Option<i32>,
    pub platform: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Monster {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub species: Option<String>,
    pub size: Option<String>,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weapon {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub weapon_type: String,
    pub rarity: Option<i32>,
    pub attack: Option<i32>,
    pub affinity: Option<i32>,
    pub element_type: Option<String>,
    pub element_value: Option<i32>,
    pub language: String,
}

pub fn get_games(conn: &Connection) -> Result<Vec<Game>> {
    let mut stmt = conn.prepare("SELECT id, name, abbreviation, release_year, platform FROM games")?;
    
    let games = stmt.query_map([], |row| {
        Ok(Game {
            id: row.get(0)?,
            name: row.get(1)?,
            abbreviation: row.get(2)?,
            release_year: row.get(3)?,
            platform: row.get(4)?,
        })
    })?
    .filter_map(|r| r.ok())
    .collect();
    
    Ok(games)
}

pub fn get_monsters_by_game(conn: &Connection, game_id: i32) -> Result<Vec<Monster>> {
    let mut stmt = conn.prepare(
        "SELECT id, game_id, name, species, size, language FROM monsters WHERE game_id = ?1"
    )?;
    
    let monsters = stmt.query_map(params![game_id], |row| {
        Ok(Monster {
            id: row.get(0)?,
            game_id: row.get(1)?,
            name: row.get(2)?,
            species: row.get(3)?,
            size: row.get(4)?,
            language: row.get(5)?,
        })
    })?
    .filter_map(|r| r.ok())
    .collect();
    
    Ok(monsters)
}

pub fn get_weapons_by_game(conn: &Connection, game_id: i32) -> Result<Vec<Weapon>> {
    let mut stmt = conn.prepare(
        "SELECT id, game_id, name, weapon_type, rarity, attack, affinity, element_type, element_value, language FROM weapons WHERE game_id = ?1"
    )?;
    
    let weapons = stmt.query_map(params![game_id], |row| {
        Ok(Weapon {
            id: row.get(0)?,
            game_id: row.get(1)?,
            name: row.get(2)?,
            weapon_type: row.get(3)?,
            rarity: row.get(4)?,
            attack: row.get(5)?,
            affinity: row.get(6)?,
            element_type: row.get(7)?,
            element_value: row.get(8)?,
            language: row.get(9)?,
        })
    })?
    .filter_map(|r| r.ok())
    .collect();
    
    Ok(weapons)
}

pub fn insert_game(conn: &Connection, game: &Game) -> Result<i32> {
    conn.execute(
        "INSERT INTO games (name, abbreviation, release_year, platform) VALUES (?1, ?2, ?3, ?4)",
        params![game.name, game.abbreviation, game.release_year, game.platform],
    )?;
    
    Ok(conn.last_insert_rowid() as i32)
}
