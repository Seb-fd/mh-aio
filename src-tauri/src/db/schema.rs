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
            sharpness TEXT,
            slots TEXT,
            skills TEXT,
            status_type TEXT,
            status_value INTEGER,
            defense_bonus INTEGER,
            crafting_cost INTEGER,
            upgrade_path TEXT,
            description TEXT,
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
            slots TEXT,
            skills TEXT,
            set_id INTEGER,
            armor_type TEXT,
            gender TEXT DEFAULT 'both',
            crafting_cost INTEGER,
            materials TEXT,
            description TEXT,
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
            breakable_parts TEXT,
            ailments TEXT,
            description TEXT,
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

        -- Monster material drop table
        CREATE TABLE IF NOT EXISTS monster_drops (
            id INTEGER PRIMARY KEY,
            monster_id INTEGER REFERENCES monsters(id),
            item_id INTEGER REFERENCES items(id),
            method TEXT NOT NULL,
            part TEXT,
            rank TEXT,
            quantity INTEGER NOT NULL DEFAULT 1,
            probability REAL NOT NULL,
            condition TEXT,
            language TEXT DEFAULT 'en'
        );

        -- Monster -> associated gear (weapons/armor crafted from its materials)
        CREATE TABLE IF NOT EXISTS monster_equipment (
            id INTEGER PRIMARY KEY,
            game_id INTEGER REFERENCES games(id),
            monster_id INTEGER REFERENCES monsters(id),
            equipment_kind TEXT NOT NULL,
            equipment_id INTEGER NOT NULL
        );

        -- Quests table
        CREATE TABLE IF NOT EXISTS quests (
            id INTEGER PRIMARY KEY,
            game_id INTEGER REFERENCES games(id),
            name TEXT NOT NULL,
            name_original TEXT,
            type TEXT,
            rank TEXT,
            hub TEXT,
            stars INTEGER,
            objective TEXT,
            location TEXT,
            time_limit INTEGER,
            faints_allowed INTEGER,
            player_limit INTEGER,
            is_key_quest BOOLEAN DEFAULT FALSE,
            unlocks TEXT,
            description TEXT,
            client TEXT,
            requirements TEXT,
            reward_money INTEGER,
            contract_fee INTEGER,
            main_monsters TEXT,
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
            buy_price INTEGER,
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
            effects TEXT,
            language TEXT DEFAULT 'en'
        );

        -- Decorations table (jewels)
        CREATE TABLE IF NOT EXISTS decorations (
            id INTEGER PRIMARY KEY,
            game_id INTEGER REFERENCES games(id),
            name TEXT NOT NULL,
            skill_id INTEGER REFERENCES skills(id),
            skill_level INTEGER,
            skill_points INTEGER,
            secondary_skill_id INTEGER REFERENCES skills(id),
            secondary_points INTEGER,
            slot_size INTEGER,
            rarity INTEGER,
            price INTEGER,
            language TEXT DEFAULT 'en'
        );

        -- Skill ability levels (points -> activated ability, includes negative thresholds)
        CREATE TABLE IF NOT EXISTS skill_levels (
            id INTEGER PRIMARY KEY,
            skill_id INTEGER REFERENCES skills(id),
            points INTEGER NOT NULL,
            ability_name TEXT NOT NULL,
            description TEXT,
            language TEXT DEFAULT 'en'
        );

        -- Normalized armor <-> skill points (parsed from armor.skills string or structured source)
        CREATE TABLE IF NOT EXISTS armor_skill_points (
            armor_id INTEGER REFERENCES armor(id),
            skill_id INTEGER REFERENCES skills(id),
            points INTEGER NOT NULL,
            PRIMARY KEY (armor_id, skill_id)
        );

        -- Normalized weapon <-> skill points
        CREATE TABLE IF NOT EXISTS weapon_skill_points (
            weapon_id INTEGER REFERENCES weapons(id),
            skill_id INTEGER REFERENCES skills(id),
            points INTEGER NOT NULL,
            PRIMARY KEY (weapon_id, skill_id)
        );

        -- Decoration crafting materials (each jewel -> items)
        CREATE TABLE IF NOT EXISTS decoration_materials (
            decoration_id INTEGER REFERENCES decorations(id),
            item_id INTEGER REFERENCES items(id),
            item_name TEXT NOT NULL,
            quantity INTEGER NOT NULL,
            PRIMARY KEY (decoration_id, item_name)
        );

        -- Weapon crafting materials junction table
        CREATE TABLE IF NOT EXISTS weapon_materials (
            id INTEGER PRIMARY KEY,
            weapon_id INTEGER REFERENCES weapons(id),
            item_id INTEGER REFERENCES items(id),
            quantity INTEGER NOT NULL
        );

        -- Weapon FORGE (direct) / UPGRADE recipes
        CREATE TABLE IF NOT EXISTS weapon_craft (
            id INTEGER PRIMARY KEY,
            weapon_id INTEGER REFERENCES weapons(id),
            craft_kind TEXT NOT NULL,
            item_id INTEGER REFERENCES items(id),
            quantity INTEGER NOT NULL
        );

        -- Armor crafting materials junction table
        CREATE TABLE IF NOT EXISTS armor_materials (
            id INTEGER PRIMARY KEY,
            armor_id INTEGER REFERENCES armor(id),
            item_id INTEGER REFERENCES items(id),
            quantity INTEGER NOT NULL
        );

        -- Item combination recipes
        CREATE TABLE IF NOT EXISTS item_combine (
            id INTEGER PRIMARY KEY,
            result_item_id INTEGER REFERENCES items(id),
            component_item_id INTEGER REFERENCES items(id),
            quantity INTEGER NOT NULL,
            result_quantity INTEGER NOT NULL DEFAULT 1
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
        CREATE INDEX IF NOT EXISTS idx_weapon_mats_weapon ON weapon_materials(weapon_id);
        CREATE INDEX IF NOT EXISTS idx_weapon_mats_item ON weapon_materials(item_id);
        CREATE INDEX IF NOT EXISTS idx_weapon_craft_weapon ON weapon_craft(weapon_id);
        CREATE INDEX IF NOT EXISTS idx_weapon_craft_item ON weapon_craft(item_id);
        CREATE INDEX IF NOT EXISTS idx_armor_mats_armor ON armor_materials(armor_id);
        CREATE INDEX IF NOT EXISTS idx_armor_mats_item ON armor_materials(item_id);
        CREATE INDEX IF NOT EXISTS idx_item_combine_result ON item_combine(result_item_id);
        CREATE INDEX IF NOT EXISTS idx_item_combine_component ON item_combine(component_item_id);
        CREATE INDEX IF NOT EXISTS idx_item_sources_item ON item_sources(item_id);
        CREATE INDEX IF NOT EXISTS idx_quest_rewards_quest ON quest_rewards(quest_id);
        CREATE INDEX IF NOT EXISTS idx_monster_drops_monster ON monster_drops(monster_id);
        CREATE INDEX IF NOT EXISTS idx_monster_drops_item ON monster_drops(item_id);
        CREATE INDEX IF NOT EXISTS idx_monster_eq_monster ON monster_equipment(monster_id);
        CREATE INDEX IF NOT EXISTS idx_skill_levels_skill ON skill_levels(skill_id);
        CREATE INDEX IF NOT EXISTS idx_skill_levels_points ON skill_levels(points);
        CREATE INDEX IF NOT EXISTS idx_armor_skill_points_skill ON armor_skill_points(skill_id);
        CREATE INDEX IF NOT EXISTS idx_armor_skill_points_armor ON armor_skill_points(armor_id);
        CREATE INDEX IF NOT EXISTS idx_weapon_skill_points_skill ON weapon_skill_points(skill_id);
        CREATE INDEX IF NOT EXISTS idx_weapon_skill_points_weapon ON weapon_skill_points(weapon_id);
        CREATE INDEX IF NOT EXISTS idx_decorations_skill ON decorations(skill_id);
        CREATE INDEX IF NOT EXISTS idx_decorations_game ON decorations(game_id);
        CREATE INDEX IF NOT EXISTS idx_decoration_mats_deco ON decoration_materials(decoration_id);
        CREATE INDEX IF NOT EXISTS idx_decoration_mats_item ON decoration_materials(item_id);
    ")?;

    apply_migrations(conn)?;

    Ok(())
}

fn apply_migrations(conn: &Connection) -> Result<()> {
    add_column_if_missing(conn, "monsters", "description", "TEXT")?;
    add_column_if_missing(conn, "weapons", "description", "TEXT")?;
    add_column_if_missing(conn, "weapons", "status_type", "TEXT")?;
    add_column_if_missing(conn, "weapons", "status_value", "INTEGER")?;
    add_column_if_missing(conn, "weapons", "defense_bonus", "INTEGER")?;
    add_column_if_missing(conn, "armor", "description", "TEXT")?;
    add_column_if_missing(conn, "armor", "armor_type", "TEXT")?;
    add_column_if_missing(conn, "quests", "description", "TEXT")?;
    add_column_if_missing(conn, "quests", "hub", "TEXT")?;
    add_column_if_missing(conn, "quests", "stars", "INTEGER")?;
    add_column_if_missing(conn, "quests", "client", "TEXT")?;
    add_column_if_missing(conn, "quests", "requirements", "TEXT")?;
    add_column_if_missing(conn, "quests", "reward_money", "INTEGER")?;
    add_column_if_missing(conn, "quests", "contract_fee", "INTEGER")?;
    add_column_if_missing(conn, "quests", "main_monsters", "TEXT")?;
    // Decorations extended columns for jewels with two skills
    add_column_if_missing(conn, "decorations", "skill_points", "INTEGER")?;
    add_column_if_missing(conn, "decorations", "secondary_skill_id", "INTEGER")?;
    add_column_if_missing(conn, "decorations", "secondary_points", "INTEGER")?;
    add_column_if_missing(conn, "decorations", "price", "INTEGER")?;
    // Backfill skill_level -> skill_points for legacy rows
    let _ = conn.execute("UPDATE decorations SET skill_points = skill_level WHERE skill_points IS NULL AND skill_level IS NOT NULL", []);
    add_column_if_missing(conn, "items", "buy_price", "INTEGER")?;
    add_column_if_missing(conn, "armor", "gender", "TEXT")?;
    add_column_if_missing(conn, "quests", "name_original", "TEXT")?;
    Ok(())
}

fn add_column_if_missing(
    conn: &Connection,
    table: &str,
    column: &str,
    column_type: &str,
) -> Result<()> {
    let exists: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM pragma_table_info(?1) WHERE name = ?2",
            rusqlite::params![table, column],
            |r| r.get(0),
        )
        .unwrap_or(false);

    if !exists {
        let stmt = format!("ALTER TABLE {} ADD COLUMN {} {}", table, column, column_type);
        conn.execute(&stmt, [])?;
    }

    Ok(())
}
