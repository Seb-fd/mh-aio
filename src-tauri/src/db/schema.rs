use rusqlite::{Connection, Result};

pub fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch("
        -- Games table
        CREATE TABLE IF NOT EXISTS games (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            abbreviation TEXT NOT NULL,
            release_year INTEGER,
            platform TEXT
        );

        -- Weapons table
        CREATE TABLE IF NOT EXISTS weapons (
            id INTEGER PRIMARY KEY,
            game_id INTEGER REFERENCES games(id),
            name TEXT NOT NULL,
            weapon_type TEXT NOT NULL,
            rarity INTEGER,
            attack INTEGER,
            affinity INTEGER,
            element_type TEXT,
            element_value INTEGER,
            sharpness TEXT,  -- JSON
            slots TEXT,      -- JSON
            skills TEXT,     -- JSON
            crafting_cost INTEGER,
            upgrade_path TEXT,  -- JSON
            language TEXT DEFAULT 'en'
        );

        -- Armor table
        CREATE TABLE IF NOT EXISTS armor (
            id INTEGER PRIMARY KEY,
            game_id INTEGER REFERENCES games(id),
            name TEXT NOT NULL,
            slot_type TEXT NOT NULL,
            rank TEXT NOT NULL,
            rarity INTEGER,
            defense_base INTEGER,
            defense_max INTEGER,
            resistance_fire INTEGER,
            resistance_water INTEGER,
            resistance_thunder INTEGER,
            resistance_ice INTEGER,
            resistance_dragon INTEGER,
            slots TEXT,  -- JSON
            skills TEXT,  -- JSON
            set_id INTEGER,
            crafting_cost INTEGER,
            materials TEXT,  -- JSON
            language TEXT DEFAULT 'en'
        );

        -- Armor sets table
        CREATE TABLE IF NOT EXISTS armor_sets (
            id INTEGER PRIMARY KEY,
            game_id INTEGER REFERENCES games(id),
            name TEXT NOT NULL,
            bonus_skill TEXT,
            bonus_required INTEGER,
            language TEXT DEFAULT 'en'
        );

        -- Monsters table
        CREATE TABLE IF NOT EXISTS monsters (
            id INTEGER PRIMARY KEY,
            game_id INTEGER REFERENCES games(id),
            name TEXT NOT NULL,
            species TEXT,
            size TEXT,
            breakable_parts TEXT,  -- JSON
            ailments TEXT,  -- JSON
            language TEXT DEFAULT 'en'
        );

        -- Monster weaknesses table
        CREATE TABLE IF NOT EXISTS monster_weaknesses (
            id INTEGER PRIMARY KEY,
            monster_id INTEGER REFERENCES monsters(id),
            part_name TEXT NOT NULL,
            sever INTEGER,
            blunt INTEGER,
            projectile INTEGER,
            fire INTEGER,
            water INTEGER,
            thunder INTEGER,
            ice INTEGER,
            dragon INTEGER
        );

        -- Quests table
        CREATE TABLE IF NOT EXISTS quests (
            id INTEGER PRIMARY KEY,
            game_id INTEGER REFERENCES games(id),
            name TEXT NOT NULL,
            type TEXT,
            rank TEXT,
            objective TEXT,
            location TEXT,
            time_limit INTEGER,
            faints_allowed INTEGER,
            player_limit INTEGER,
            is_key_quest BOOLEAN DEFAULT FALSE,
            unlocks TEXT,
            language TEXT DEFAULT 'en'
        );

        -- Quest rewards table
        CREATE TABLE IF NOT EXISTS quest_rewards (
            id INTEGER PRIMARY KEY,
            quest_id INTEGER REFERENCES quests(id),
            item_id INTEGER REFERENCES items(id),
            quantity INTEGER,
            probability REAL,
            condition TEXT
        );

        -- Items table
        CREATE TABLE IF NOT EXISTS items (
            id INTEGER PRIMARY KEY,
            game_id INTEGER REFERENCES games(id),
            name TEXT NOT NULL,
            category TEXT,
            rarity INTEGER,
            sell_price INTEGER,
            description TEXT,
            language TEXT DEFAULT 'en'
        );

        -- Item sources table
        CREATE TABLE IF NOT EXISTS item_sources (
            id INTEGER PRIMARY KEY,
            item_id INTEGER REFERENCES items(id),
            source_type TEXT,
            source_id INTEGER,
            quantity_min INTEGER,
            quantity_max INTEGER,
            probability REAL,
            location TEXT,
            conditions TEXT
        );

        -- Skills table
        CREATE TABLE IF NOT EXISTS skills (
            id INTEGER PRIMARY KEY,
            game_id INTEGER REFERENCES games(id),
            name TEXT NOT NULL,
            description TEXT,
            max_level INTEGER,
            effects TEXT,  -- JSON
            language TEXT DEFAULT 'en'
        );

        -- Decorations table
        CREATE TABLE IF NOT EXISTS decorations (
            id INTEGER PRIMARY KEY,
            game_id INTEGER REFERENCES games(id),
            name TEXT NOT NULL,
            skill_id INTEGER REFERENCES skills(id),
            skill_level INTEGER,
            slot_size INTEGER,
            rarity INTEGER,
            language TEXT DEFAULT 'en'
        );

        -- Indexes
        CREATE INDEX IF NOT EXISTS idx_weapons_game ON weapons(game_id);
        CREATE INDEX IF NOT EXISTS idx_weapons_type ON weapons(weapon_type);
        CREATE INDEX IF NOT EXISTS idx_armor_game ON armor(game_id);
        CREATE INDEX IF NOT EXISTS idx_armor_slot ON armor(slot_type);
        CREATE INDEX IF NOT EXISTS idx_monsters_game ON monsters(game_id);
        CREATE INDEX IF NOT EXISTS idx_quests_game ON quests(game_id);
        CREATE INDEX IF NOT EXISTS idx_items_game ON items(game_id);
        CREATE INDEX IF NOT EXISTS idx_skills_game ON skills(game_id);
    ")?;

    Ok(())
}
