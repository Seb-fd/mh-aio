use rusqlite::{Connection, Result};

const MH2G: i32 = 5;

pub fn seed(conn: &Connection) -> Result<()> {
    seed_games(conn)?;
    seed_monsters(conn)?;
    seed_weapons(conn)?;
    seed_armor(conn)?;
    seed_quests(conn)?;
    seed_items(conn)?;
    seed_skills(conn)?;
    seed_weapon_materials(conn)?;
    seed_armor_materials(conn)?;
    seed_item_sources(conn)?;
    seed_item_combine(conn)?;
    backfill_descriptions(conn)?;
    backfill_costs(conn)?;
    Ok(())
}

fn backfill_costs(conn: &Connection) -> Result<()> {
    let weapons: &[(i32, i32)] = &[
        (1, 3840), (2, 1980), (3, 980), (4, 3120), (5, 2210),
        (6, 2470), (7, 2300), (8, 1440), (9, 1040), (10, 1980),
        (11, 2460), (12, 1400), (13, 25200), (14, 36400), (15, 52800),
        (16, 22400), (17, 32200), (18, 62400), (19, 19200), (20, 28600),
        (21, 2100), (22, 28600), (23, 36400), (24, 2640), (25, 1560),
        (26, 1320), (27, 48000), (28, 24800), (29, 30800), (30, 62400),
    ];
    for (id, cost) in weapons {
        conn.execute(
            "UPDATE weapons SET crafting_cost = ?1 WHERE id = ?2 AND game_id = ?3 AND crafting_cost = 0",
            rusqlite::params![cost, id, MH2G],
        )?;
    }

    let armors: &[(i32, i32)] = &[
        (1, 100), (2, 100), (3, 100), (4, 100), (5, 100),
        (6, 600), (7, 600), (8, 8800), (9, 8800), (10, 8800),
        (11, 8800), (12, 8800), (13, 12400), (14, 12400), (15, 12400),
        (16, 12400), (17, 12400), (18, 18600), (19, 18600), (20, 9600),
        (21, 9600), (22, 13200), (23, 13200), (24, 22400), (25, 22400),
    ];
    for (id, cost) in armors {
        conn.execute(
            "UPDATE armor SET crafting_cost = ?1 WHERE id = ?2 AND game_id = ?3 AND crafting_cost = 0",
            rusqlite::params![cost, id, MH2G],
        )?;
    }

    Ok(())
}

fn backfill_descriptions(conn: &Connection) -> Result<()> {
    let monsters: &[(i32, &str)] = &[
        (1, "The King of the Skies. A fearsome Flying Wyvern that rules the airspace over its territory. Its fiery breath and aerial dominance have ended many a hunter's career. Approach with extreme caution and watch its tail — it carries a deadly poison."),
        (2, "The Queen of the Land. Counterpart to Rathalos, this formidable Flying Wyvern patrols the ground with deadly precision. Her fireballs and tail flips are legendary among hunters. Beware her area control — she does not share her territory."),
        (3, "A savage apex predator of the Sandy Plains. Tigrex charges with terrifying speed, using its powerful forelegs to devastating effect. Its roar is said to shake the very ground. Wounded Tigrex become even more dangerous."),
        (4, "A stealthy nocturnal hunter that strikes from the shadows with blinding speed. Its tail, lined with sharp scales, can cut through armor. Highly territorial and fiercely aggressive when cornered."),
        (5, "An aggressive Bird Wyvern with large ears that act as radar. Its deafening screeches can stun hunters. Often the first real challenge a hunter faces in the Kokoto region. Its flame breath and aerial charges have ended many careers."),
        (6, "A pale, blind wyvern that dwells in dark caves. Detects prey through smell and electric field sense. Its body conducts electricity, and it can unleash paralyzing Thunderballs. A genuinely unsettling foe."),
        (7, "A rubbery-skinned Bird Wyvern known for its unpredictable flash attacks. Its head butts are powerful, and its poisonous tail can inflict multiple status effects simultaneously. Surprisingly resilient."),
        (8, "The Wyvern of the Water. This massive Piscine Wyvern dominates rivers and lakes. Its water beams and hip charges can launch hunters across the map. A true test of patience and positioning."),
        (9, "A smaller Piscine Wyvern that burrows through desert sand. Can summon sandstorms to disorient hunters. Its jaw attacks and sand breath are effective even against well-armored prey."),
        (10, "Alpha of the Genprey pack. Uses coordinated tactics with its pack to bring down prey. Its vicious bites and group attacks require hunters to manage crowd control carefully."),
        (11, "Leader of the Ioprey pack. Hunts with venomous bites and coordinated group attacks. Encountering a full Iodrome pack is a serious threat to unprepared hunters."),
        (12, "Pack leader of the Velociprey. Uses group tactics and can call its pack for coordinated attacks. A good warm-up hunt for those learning to deal with multiple monsters."),
        (13, "A massive crab-like Carapaceon with an extremely hard shell. Its powerful claws can deliver crushing blows. Particularly aggressive during mating season. Its bubble attack disorients hunters at close range."),
        (14, "A heavily armored Carapaceon wielding a stone club. Stalks ancient ruins and uses its environment to devastating effect. Beware its charged attacks — they can launch hunters across the map."),
        (15, "A fanged beast with a colorful crest. Its fecal attacks are surprisingly effective, and it can throw objects at hunters. Sometimes found in pink or green varieties, each with different attack patterns."),
        (16, "A terrifyingly powerful Fanged Beast. Its muscular arms can shatter rock. Enraged Rajang channel lightning through their bodies, dealing massive damage. Considered one of the most dangerous non-Elder Dragon monsters."),
        (17, "An Elder Dragon that takes the form of a unicorn wreathed in lightning. Master of Thunder element. Despite its graceful appearance, Kirin is extremely dangerous and its thunder attacks can paralyze hunters instantly."),
        (18, "The Dragon of Steel. Kushala Daora's metallic scales deflect most attacks and it surrounds itself with a perpetual wind barrier. Few materials are as coveted as its daora shells and dragon scales."),
        (19, "The Emperor of Flame. Teostra's fiery breath and explosive powder attacks create a deadly arena of flame. Its mane ignites when enraged, dealing continuous fire damage to anything nearby."),
        (20, "The Phantom Dragon. Chameleos can turn invisible and release toxic breath. Patient and calculating, it uses its environment to ambush unwary hunters. Its poison attacks stack dangerously."),
        (21, "A colossal Elder Dragon whose very footsteps shake the earth. Lao-Shan Lung is so massive that traditional combat tactics are useless — hunters must focus on driving it away from the fortress. A true siege encounter."),
        (22, "A fearsome Elder Dragon said to be the apex land predator. Its roar alone can paralyze hunters. Its prehensile tail and devastating charges make it one of the most dangerous creatures in the world."),
        (23, "A gargantuan Elder Dragon resembling a whale with wings. Yama Tsukami dwells in the sky and is so large that hunters cling to its body to damage it. Its thunder breath can flatten entire parties."),
        (24, "The Black Dragon. Said to be the destroyer of Castle Schrade. Fatalis is the apex predator of all monsters — its fire breath can melt through armor and its homing fireballs are devastating. Only the most skilled hunters dare challenge it."),
        (25, "The legendary White Dragon, said to be the elder of all Fatalis. Controls thunder and ice alongside its already devastating fire breath. Its very presence warps reality around it. The ultimate test of a hunter's skill."),
        (26, "A colorful Bird Wyvern whose dazzling scales can induce sleep. Uses its hypnotic pattern to stun prey before delivering fatal pecks. Encountered in Tower 3 of the Tower."),
        (27, "A Piscine Wyvern adapted to volcanic environments. Its hardened lava-coated body provides excellent defense. Its molten armor cracks periodically, revealing vulnerable flesh."),
        (28, "A large amphibious creature that uses mud and water to its advantage. Its powerful legs can launch hunters across the map. Native to the Jurassic Frontier."),
    ];

    for (id, desc) in monsters {
        conn.execute(
            "UPDATE monsters SET description = ?1 WHERE id = ?2 AND game_id = ?3 AND description IS NULL",
            rusqlite::params![desc, id, MH2G],
        )?;
    }

    let weapons: &[(i32, &str)] = &[
        (1, "Standard-issue Great Sword of the guild. Reliable and well-balanced. The starting point for many a hunter's career."),
        (2, "A Long Sword carved from monster bone. The bone's natural sharpness makes for a respectable weapon."),
        (3, "The most basic Sword and Shield. Trusted by hunters for generations."),
        (4, "A solid iron hammer. Slow but powerful enough to stun most monsters."),
        (5, "Standard iron lance used by the guild. Combines reach with reliable defense."),
        (6, "A heavy weapon combining a lance with a firearm. Devastating burst damage at close range."),
        (7, "A Switch Axe that transforms between axe and sword modes. Switches allow adaptive combat."),
        (8, "A simple but effective bow. The starting weapon for many a bow hunter."),
        (9, "A reliable Light Bowgun with good rapid-fire capability."),
        (10, "A heavy but powerful Bowgun. Devastating single shots at the cost of mobility."),
        (11, "A horn carved from monster bone. Plays melodies to buff allies."),
        (12, "Twin blades carved from monster bone. Allow rapid demon-mode combos."),
        (13, "A sword and shield forged from Rathalos materials. Its flame affinity is unmistakable."),
        (14, "A Switch Axe forged from Rathalos parts. Channels the Fire King's wrath in axe mode."),
        (15, "A brutal Great Sword carved from Tigrex parts. Negative affinity but exceptional raw power."),
        (16, "Twin blades forged from Nargacuga materials. High affinity and lightning-fast combos."),
        (17, "A heavy lance tipped with Tigrex claw materials. Sacrifices affinity for raw power."),
        (18, "A hammer forged from Rajang parts. Pulses with the beast's lightning aura."),
        (19, "A bow infused with Kirin's horn. Arrows crackle with divine thunder."),
        (20, "A hunting horn fashioned from Khezu's horn. Its melodies paralyze foes."),
        (21, "An upgraded bone Long Sword. Sharp and reliable."),
        (22, "A lance forged from Plesioth materials. The Water element soaks through armor."),
        (23, "A hammer made from Congalala parts. Its presence is... aromatic."),
        (24, "A Gunlance with bone construction. Wide shells for crowd control."),
        (25, "An upgraded iron bow. Slightly better range and power."),
        (26, "A bow crafted from monster bone. Lightweight and effective."),
        (27, "A brutal Great Sword favored by barbarian hunters. Slow but crushing."),
        (28, "A flexible lance made from Gypceros parts. Lightweight but durable."),
        (29, "A Long Sword forged from Fatalis's very essence. Said to whisper dark promises to its wielder."),
        (30, "A hammer forged from Fatalis materials. Pulses with the Black Dragon's malice."),
    ];

    for (id, desc) in weapons {
        conn.execute(
            "UPDATE weapons SET description = ?1 WHERE id = ?2 AND game_id = ?3 AND description IS NULL",
            rusqlite::params![desc, id, MH2G],
        )?;
    }

    let armors: &[(i32, &str)] = &[
        (1, "Basic leather helm. Worn by trainee hunters."),
        (2, "Basic leather chest armor. Functional and affordable."),
        (3, "Leather arm guards. Protects the forearms from glancing blows."),
        (4, "A wide leather belt. Carries basic supplies."),
        (5, "Sturdy leather pants. Standard issue for new hunters."),
        (6, "Chainmail helm. Better protection than leather, at the cost of weight."),
        (7, "Chainmail chest piece. Reliable mid-tier armor."),
        (8, "A helm crafted from Rathalos scales. The crest recalls the Fire King's crown."),
        (9, "Chest armor forged from Rathalos parts. Heat-resistant and intimidating."),
        (10, "Arm guards crafted from Rathalos scales. Light and fire-resistant."),
        (11, "Waist armor forged from Rathalos parts. Protects the lower back."),
        (12, "Leg armor crafted from Rathalos scales. The complete Rathalos aesthetic."),
        (13, "A fearsome helm that mimics Tigrex's head. The roar alone can stun monsters."),
        (14, "Chest armor carved from Tigrex scales. Encourages aggression."),
        (15, "Tigrex arm guards. The claws remain sharp even after forging."),
        (16, "Tigrex waist armor. Reinforced with thick hide."),
        (17, "Tigrex leg armor. Built for charging."),
        (18, "A stealthy helm from Nargacuga parts. Reduces detection range."),
        (19, "Light chest armor from Nargacuga scales. Excellent mobility."),
        (20, "A helm made from Khezu hide. Said to conduct electricity."),
        (21, "Khezu chest piece. Pale and unsettling."),
        (22, "A helm crowned with Kirin's horn. Pulses with residual thunder."),
        (23, "Chest piece blessed by Kirin's essence. Worn by true Thunder masters."),
        (24, "A helm forged from Fatalis scales. Said to grant its wearer the Black Dragon's favor."),
        (25, "The pinnacle of armor crafting. Crafted only by hunters who have slain a Black Dragon."),
    ];

    for (id, desc) in armors {
        conn.execute(
            "UPDATE armor SET description = ?1 WHERE id = ?2 AND game_id = ?3 AND description IS NULL",
            rusqlite::params![desc, id, MH2G],
        )?;
    }

    let quests: &[(i32, &str)] = &[
        (1, "Gather mushrooms and herbs for the village."),
        (2, "Hunt your first large monster."),
        (3, "Slay the Fire King that terrorizes the region."),
        (4, "Drive off the Tigrex threatening the Sandy Plains."),
        (5, "Slay the elusive Nargacuga."),
        (6, "Defeat the dreaded Rajang."),
        (7, "Slay the Elder Dragon Kirin."),
        (8, "Investigate Castle Schrade and slay the Black Dragon."),
        (9, "Slay the legendary White Dragon."),
        (10, "Gather resources from the Forest and Hills."),
        (11, "Gather resources from the Desert region."),
        (12, "Hunt the Plesioth in the river."),
    ];

    for (id, desc) in quests {
        conn.execute(
            "UPDATE quests SET description = ?1 WHERE id = ?2 AND game_id = ?3 AND description IS NULL",
            rusqlite::params![desc, id, MH2G],
        )?;
    }

    let items: &[(i32, &str)] = &[
        (1, "Restores a small amount of health. A hunter's best friend."),
        (2, "Restores a moderate amount of health. Combine Potion + Honey."),
        (3, "Cures poison. Essential for many hunts."),
        (4, "Temporarily boosts maximum stamina. Useful for long hunts."),
        (5, "Fully restores health. Combine Nutrients + Mega Potion."),
        (6, "A scale from the Fire King. Shimmers with reddish luster."),
        (7, "Flexible webbing from Rathalos's wings. Surprisingly tough."),
        (8, "A rare, gleaming plate from Rathalos. Highly prized by smiths."),
        (9, "A scale from a Tigrex. Hard as steel."),
        (10, "A razor-sharp claw from a Tigrex. Can shred armor."),
        (11, "A massive fang from a Tigrex. Used in powerful weapons."),
        (12, "A smooth, dark scale from a Nargacuga. Light and sharp."),
        (13, "Supple pelt from a Nargacuga. Used for light armor."),
        (14, "A pale, bloodless scale from a Khezu. Conducts electricity."),
        (15, "Thick, rubbery hide from a Khezu. Surprisingly durable."),
        (16, "Dense black fur from a Rajang. Hums with latent Thunder."),
        (17, "A claw from the dreaded Rajang. Said to contain pure rage."),
        (18, "Mystical hide from Kirin. Worn by only the most devoted Thunder hunters."),
        (19, "A horn from the Elder Dragon Kirin. Channels thunder."),
        (20, "A scale from the Black Dragon itself. Said to be cursed."),
        (21, "A piercing eye from Fatalis. Witnesses the end of civilizations."),
        (22, "Generic monster carving. Often retrieved as a byproduct."),
        (23, "A common but useful ore. Smelts into Machalite ingots."),
        (24, "A common earth crystal. Used in many recipes."),
        (25, "Rare crystals that pulse with life energy. Used in high-grade potions."),
        (26, "Sweet honey gathered from nature. Combine ingredient for potions."),
        (27, "Boosts maximum stamina for a short time."),
        (28, "An ore infused with dragon energy. Used in Elder Dragon gear."),
        (29, "A stone that radiates heat. Found in volcanic regions."),
        (30, "A scale from a Cephalos. Light and slightly oily."),
        (31, "A scale from a Genprey. Has a faint poisonous residue."),
    ];

    for (id, desc) in items {
        conn.execute(
            "UPDATE items SET description = ?1 WHERE id = ?2 AND game_id = ?3 AND description IS NULL",
            rusqlite::params![desc, id, MH2G],
        )?;
    }

    Ok(())
}

fn seed_games(conn: &Connection) -> Result<()> {
    let games = [
        (1, "Monster Hunter World", "MHW", 2018, "PS4 / XB1 / PC"),
        (2, "Monster Hunter Rise", "MHR", 2021, "Switch / PC"),
        (3, "Monster Hunter Wilds", "MHWilds", 2025, "PS5 / XB / PC"),
        (4, "MH Portable 3rd", "MHP3rd", 2010, "PSP / PS3"),
        (5, "MH 2ndG (Freedom Unite)", "MH2G", 2008, "PSP"),
    ];

    for (id, name, abbr, year, platform) in games {
        conn.execute(
            "INSERT OR IGNORE INTO games (id, name, abbreviation, release_year, platform) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![id, name, abbr, year, platform],
        )?;
    }

    Ok(())
}

fn seed_monsters(conn: &Connection) -> Result<()> {
    let monsters: &[(i32, &str, &str, &str)] = &[
        (1, "Rathalos", "Flying Wyvern", "Large"),
        (2, "Rathian", "Flying Wyvern", "Large"),
        (3, "Tigrex", "Flying Wyvern", "Large"),
        (4, "Nargacuga", "Flying Wyvern", "Large"),
        (5, "Yian Kut-Ku", "Bird Wyvern", "Large"),
        (6, "Khezu", "Flying Wyvern", "Large"),
        (7, "Gypceros", "Bird Wyvern", "Large"),
        (8, "Plesioth", "Leviathan", "Large"),
        (9, "Cephadrome", "Leviathan", "Large"),
        (10, "Gendrome", "Bird Wyvern", "Large"),
        (11, "Iodrome", "Bird Wyvern", "Large"),
        (12, "Velocidrome", "Bird Wyvern", "Large"),
        (13, "Daimyo Hermitaur", "Carapaceon", "Large"),
        (14, "Shogun Ceanataur", "Carapaceon", "Large"),
        (15, "Congalala", "Bird Wyvern", "Large"),
        (16, "Rajang", "Fanged Beast", "Large"),
        (17, "Kirin", "Elder Dragon", "Large"),
        (18, "Kushala Daora", "Elder Dragon", "Large"),
        (19, "Teostra", "Elder Dragon", "Large"),
        (20, "Chameleos", "Elder Dragon", "Large"),
        (21, "Lao-Shan Lung", "Elder Dragon", "Giant"),
        (22, "Akantor", "Elder Dragon", "Large"),
        (23, "Yama Tsukami", "Elder Dragon", "Giant"),
        (24, "Fatalis", "Elder Dragon", "Large"),
        (25, "White Fatalis", "Elder Dragon", "Large"),
        (26, "Hypnocatrice", "Bird Wyvern", "Large"),
        (27, "Lavasioth", "Leviathan", "Large"),
        (28, "Tetsucabra", "Amphibian", "Large"),
    ];

    for (id, name, species, size) in monsters {
        conn.execute(
            "INSERT OR IGNORE INTO monsters (id, game_id, name, species, size, language) VALUES (?1, ?2, ?3, ?4, ?5, 'en')",
            rusqlite::params![id, MH2G, name, species, size],
        )?;
    }

    Ok(())
}

fn seed_weapons(conn: &Connection) -> Result<()> {
    let weapons: &[(i32, &str, &str, i32, i32, i32, &str, i32)] = &[
        (1, "Buster Sword", "Great Sword", 1, 384, 0, "", 0),
        (2, "Bone Blade", "Long Sword", 1, 198, 0, "", 0),
        (3, "Hunter's Knife", "Sword and Shield", 1, 98, 0, "", 0),
        (4, "Iron Hammer", "Hammer", 1, 312, 0, "", 0),
        (5, "Iron Lance", "Lance", 1, 221, 0, "", 0),
        (6, "Iron Gunlance", "Gunlance", 1, 247, 0, "", 0),
        (7, "Bone Axe", "Switch Axe", 6, 230, 0, "", 0),
        (8, "Iron Bow", "Bow", 1, 144, 0, "", 0),
        (9, "Crossbow", "Light Bowgun", 1, 104, 0, "", 0),
        (10, "Iron Barrel", "Heavy Bowgun", 1, 198, 0, "", 0),
        (11, "Bone Horn", "Hunting Horn", 1, 246, 0, "", 0),
        (12, "Bone Tonfa", "Dual Blades", 5, 140, 0, "", 0),
        (13, "Rathalos Sword", "Sword and Shield", 7, 252, 0, "Fire", 20),
        (14, "Rathalos Flare", "Switch Axe", 9, 364, 0, "Fire", 28),
        (15, "Tigrex Sword", "Great Sword", 7, 528, -10, "", 0),
        (16, "Nargacuga Blade", "Dual Blades", 9, 224, 30, "", 0),
        (17, "Tigrex Claw", "Lance", 8, 322, -10, "", 0),
        (18, "Rajang Club", "Hammer", 8, 624, 0, "Thunder", 30),
        (19, "Kirin Bolts", "Bow", 7, 192, 0, "Thunder", 30),
        (20, "Khezu Horn", "Hunting Horn", 6, 286, 0, "Thunder", 20),
        (21, "Bone Katana", "Long Sword", 2, 210, 0, "", 0),
        (22, "Plesioth Lance", "Lance", 6, 286, 0, "Water", 18),
        (23, "Congalala Hammer", "Hammer", 5, 364, 0, "", 0),
        (24, "Bone Gunlance", "Gunlance", 2, 264, 0, "", 0),
        (25, "Iron Bow+", "Bow", 2, 156, 0, "", 0),
        (26, "Bone Bow", "Bow", 1, 132, 0, "", 0),
        (27, "Barbaroi Blade", "Great Sword", 6, 480, 0, "", 0),
        (28, "Gypceros Stave", "Lance", 6, 248, 0, "", 0),
        (29, "Fatalis Blade", "Long Sword", 9, 308, 0, "Dragon", 38),
        (30, "Fatalis Crusher", "Hammer", 9, 624, 0, "Dragon", 40),
    ];

    for (id, name, wtype, rarity, atk, aff, elem, eval) in weapons {
        conn.execute(
            "INSERT OR IGNORE INTO weapons (id, game_id, name, weapon_type, rarity, attack, affinity, element_type, element_value, language)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 'en')",
            rusqlite::params![id, MH2G, name, wtype, rarity, atk, aff, elem, eval],
        )?;
    }

    Ok(())
}

fn seed_armor(conn: &Connection) -> Result<()> {
    let armors: &[(i32, &str, &str, &str, i32, i32, i32, i32, i32, i32, i32, i32)] = &[
        (1, "Leather Helm", "head", "Low", 1, 4, 16, 2, 0, 0, 0, 0),
        (2, "Leather Mail", "chest", "Low", 1, 4, 16, 2, 0, 0, 0, 0),
        (3, "Leather Vambraces", "arms", "Low", 1, 2, 12, 1, 0, 0, 0, 0),
        (4, "Leather Belt", "waist", "Low", 1, 2, 12, 1, 0, 0, 0, 0),
        (5, "Leather Trousers", "legs", "Low", 1, 2, 14, 1, 0, 0, 0, 0),
        (6, "Chainmail Helm", "head", "Low", 2, 6, 22, 3, 1, -1, 0, 0),
        (7, "Chainmail Vest", "chest", "Low", 2, 6, 22, 3, 1, -1, 0, 0),
        (8, "Rathalos Helm", "head", "High", 6, 22, 56, 4, 2, 1, -1, 3),
        (9, "Rathalos Mail", "chest", "High", 6, 22, 56, 4, 2, 1, -1, 3),
        (10, "Rathalos Vambraces", "arms", "High", 6, 22, 52, 4, 2, 1, -1, 3),
        (11, "Rathalos Coil", "waist", "High", 6, 22, 52, 4, 2, 1, -1, 3),
        (12, "Rathalos Greaves", "legs", "High", 6, 22, 56, 4, 2, 1, -1, 3),
        (13, "Tigrex Helm", "head", "High", 7, 26, 62, 3, -1, 3, 0, 0),
        (14, "Tigrex Mail", "chest", "High", 7, 26, 62, 3, -1, 3, 0, 0),
        (15, "Tigrex Vambraces", "arms", "High", 7, 26, 58, 3, -1, 3, 0, 0),
        (16, "Tigrex Coil", "waist", "High", 7, 26, 58, 3, -1, 3, 0, 0),
        (17, "Tigrex Greaves", "legs", "High", 7, 26, 62, 3, -1, 3, 0, 0),
        (18, "Nargacuga Helm", "head", "High", 9, 30, 68, 5, 2, -1, -1, 1),
        (19, "Nargacuga Mail", "chest", "High", 9, 30, 68, 5, 2, -1, -1, 1),
        (20, "Khezu Helm", "head", "High", 6, 24, 58, 4, -2, 3, 2, 0),
        (21, "Khezu Mail", "chest", "High", 6, 24, 58, 4, -2, 3, 2, 0),
        (22, "Kirin Horn", "head", "G", 7, 26, 64, 6, -3, 0, 4, 0),
        (23, "Kirin Vest", "chest", "G", 7, 26, 64, 6, -3, 0, 4, 0),
        (24, "Fatalis Helm", "head", "G", 9, 32, 76, 7, 2, 0, 0, 3),
        (25, "Fatalis Mail", "chest", "G", 9, 32, 76, 7, 2, 0, 0, 3),
    ];

    for (id, name, slot, rank, rarity, def_base, def_max, fire, water, thunder, ice, dragon) in armors {
        conn.execute(
            "INSERT OR IGNORE INTO armor (id, game_id, name, slot_type, rank, rarity, defense_base, defense_max,
                resistance_fire, resistance_water, resistance_thunder, resistance_ice, resistance_dragon, language)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, 'en')",
            rusqlite::params![id, MH2G, name, slot, rank, rarity, def_base, def_max, fire, water, thunder, ice, dragon],
        )?;
    }

    Ok(())
}

fn seed_quests(conn: &Connection) -> Result<()> {
    let quests: &[(i32, &str, &str, &str, &str, i32, i32, bool)] = &[
        (1, "Gathering Road", "Gathering", "Low", "Mezeporta", 50, 3, false),
        (2, "The Birth of a Hunter", "Hunting", "Low", "Mezeporta", 50, 3, true),
        (3, "Rathalos, King of the Sky", "Hunting", "High", "Mezeporta", 50, 2, true),
        (4, "Tigrex of the Sand Sea", "Hunting", "High", "Mezeporta", 50, 2, true),
        (5, "Nargacuga, the Shadow", "Hunting", "High", "Mezeporta", 50, 2, true),
        (6, "Slay Rajang!", "Hunting", "G", "Mezeporta", 50, 1, true),
        (7, "Kirin, the Lightning", "Hunting", "G", "Mezeporta", 50, 1, true),
        (8, "Fatalis", "Hunting", "G", "Castle Schrade", 50, 1, true),
        (9, "White Fatalis", "Hunting", "G", "Castle Schrade", 50, 1, true),
        (10, "A Gathering in the Forest", "Gathering", "Low", "Forest and Hills", 50, 3, false),
        (11, "Swimmin' in the Desert", "Gathering", "Low", "Desert", 50, 3, false),
        (12, "The Piscine Wyvern", "Hunting", "Low", "Desert", 50, 3, true),
    ];

    for (id, name, qtype, rank, location, time_limit, faints, is_key) in quests {
        conn.execute(
            "INSERT OR IGNORE INTO quests (id, game_id, name, type, rank, objective, location, time_limit, faints_allowed, is_key_quest, language)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 'en')",
            rusqlite::params![id, MH2G, name, qtype, rank, qtype, location, time_limit, faints, is_key],
        )?;
    }

    Ok(())
}

fn seed_items(conn: &Connection) -> Result<()> {
    let items: &[(i32, &str, &str, i32, i32)] = &[
        (1, "Potion", "Consumable", 1, 8),
        (2, "Mega Potion", "Consumable", 2, 32),
        (3, "Antidote", "Consumable", 1, 6),
        (4, "Mega Nutrients", "Consumable", 3, 110),
        (5, "Max Potion", "Consumable", 4, 220),
        (6, "Rathalos Scale", "Material", 3, 120),
        (7, "Rathalos Webbing", "Material", 4, 220),
        (8, "Rathalos Plate", "Material", 6, 1660),
        (9, "Tigrex Scale", "Material", 4, 230),
        (10, "Tigrex Claw", "Material", 4, 240),
        (11, "Tigrex Fang", "Material", 4, 230),
        (12, "Nargacuga Scale", "Material", 5, 310),
        (13, "Nargacuga Pelt", "Material", 5, 320),
        (14, "Khezu Scale", "Material", 3, 110),
        (15, "Khezu Hide", "Material", 3, 120),
        (16, "Rajang Blackfur", "Material", 7, 740),
        (17, "Rajang Claw", "Material", 7, 760),
        (18, "Kirin Hide", "Material", 6, 620),
        (19, "Kirin Horn", "Material", 7, 980),
        (20, "Fatalis Scale", "Material", 8, 1200),
        (21, "Fatalis Eye", "Material", 9, 1800),
        (22, "Carvings", "Material", 1, 5),
        (23, "Machalite Ore", "Material", 2, 40),
        (24, "Earth Crystal", "Material", 2, 60),
        (25, "Lifecrystals", "Material", 4, 220),
        (26, "Honey", "Material", 1, 30),
        (27, "Nutrients", "Consumable", 2, 50),
        (28, "Dragonvein Ore", "Material", 5, 580),
        (29, "Firecell Stone", "Material", 4, 320),
        (30, "Cephalos Scale", "Material", 2, 80),
        (31, "Genprey Scale", "Material", 1, 50),
    ];

    for (id, name, category, rarity, price) in items {
        conn.execute(
            "INSERT OR IGNORE INTO items (id, game_id, name, category, rarity, sell_price, language)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'en')",
            rusqlite::params![id, MH2G, name, category, rarity, price],
        )?;
    }

    Ok(())
}

fn seed_skills(conn: &Connection) -> Result<()> {
    let skills: &[(i32, &str, &str, i32)] = &[
        (1, "Attack Up", "Increases attack power", 7),
        (2, "Defense Up", "Increases defense", 7),
        (3, "Health +1", "Increases max health by 10", 1),
        (4, "Health +2", "Increases max health by 20", 1),
        (5, "Elemental Attack", "Increases elemental damage", 5),
        (6, "Sharpness +1", "Weapon sharpness extended", 1),
        (7, "Sharpness +2", "Weapon sharpness greatly extended", 1),
        (8, "Recoil Down", "Reduces bowgun recoil", 3),
        (9, "Reload Speed", "Increases bowgun reload speed", 3),
        (10, "Evasion +1", "Increases evasion distance", 1),
        (11, "Evasion +2", "Greatly increases evasion distance", 1),
        (12, "Stamina Recov", "Recovers stamina faster", 2),
        (13, "Wind Pressure", "Reduces wind pressure from monsters", 1),
        (14, "Earplugs", "Nullifies small monster roars", 3),
        (15, "Poison Res", "Reduces poison damage", 1),
        (16, "Fire Res +1", "Increases fire resistance", 1),
        (17, "Ice Res +1", "Increases ice resistance", 1),
        (18, "Thunder Res +1", "Increases thunder resistance", 1),
        (19, "Dragon Res +1", "Increases dragon resistance", 1),
        (20, "Razor Sharp", "Reduces sharpness loss", 3),
    ];

    for (id, name, desc, max_lvl) in skills {
        conn.execute(
            "INSERT OR IGNORE INTO skills (id, game_id, name, description, max_level, language)
             VALUES (?1, ?2, ?3, ?4, ?5, 'en')",
            rusqlite::params![id, MH2G, name, desc, max_lvl],
        )?;
    }

    Ok(())
}

fn seed_weapon_materials(conn: &Connection) -> Result<()> {
    let materials: &[(i32, i32, i32)] = &[
        (13, 6, 3),
        (13, 7, 2),
        (13, 24, 5),
        (13, 22, 1),
        (14, 6, 4),
        (14, 8, 1),
        (14, 29, 3),
        (14, 24, 8),
        (15, 9, 3),
        (15, 11, 2),
        (15, 10, 1),
        (15, 23, 6),
        (16, 12, 3),
        (16, 13, 2),
        (16, 22, 1),
        (17, 9, 4),
        (17, 10, 3),
        (17, 11, 2),
        (17, 24, 10),
        (18, 16, 3),
        (18, 17, 2),
        (18, 25, 1),
        (18, 28, 4),
        (19, 18, 2),
        (19, 19, 1),
        (19, 24, 6),
        (20, 14, 4),
        (20, 15, 3),
        (20, 24, 5),
        (22, 30, 3),
        (22, 22, 2),
        (22, 24, 6),
        (29, 20, 2),
        (29, 21, 1),
        (29, 28, 6),
        (29, 25, 3),
        (30, 20, 3),
        (30, 21, 2),
        (30, 28, 8),
        (30, 19, 2),
    ];

    for (weapon_id, item_id, qty) in materials {
        conn.execute(
            "INSERT OR IGNORE INTO weapon_materials (weapon_id, item_id, quantity) VALUES (?1, ?2, ?3)",
            rusqlite::params![weapon_id, item_id, qty],
        )?;
    }

    Ok(())
}

fn seed_armor_materials(conn: &Connection) -> Result<()> {
    let materials: &[(i32, i32, i32)] = &[
        (8, 6, 4),
        (8, 7, 2),
        (8, 22, 1),
        (9, 6, 5),
        (9, 7, 3),
        (9, 23, 3),
        (10, 6, 3),
        (10, 7, 2),
        (11, 6, 4),
        (11, 7, 2),
        (12, 6, 4),
        (12, 7, 3),
        (13, 9, 4),
        (13, 10, 2),
        (13, 11, 1),
        (14, 9, 5),
        (14, 10, 3),
        (14, 11, 2),
        (15, 9, 3),
        (15, 10, 2),
        (16, 9, 4),
        (16, 11, 2),
        (17, 9, 4),
        (17, 10, 2),
        (18, 12, 3),
        (18, 13, 2),
        (19, 12, 4),
        (19, 13, 3),
        (19, 22, 1),
        (20, 14, 4),
        (20, 15, 3),
        (21, 14, 5),
        (21, 15, 4),
        (21, 23, 2),
        (22, 18, 2),
        (22, 19, 1),
        (23, 18, 3),
        (23, 19, 1),
        (24, 20, 2),
        (24, 21, 1),
        (24, 28, 4),
        (25, 20, 3),
        (25, 21, 1),
        (25, 28, 6),
    ];

    for (armor_id, item_id, qty) in materials {
        conn.execute(
            "INSERT OR IGNORE INTO armor_materials (armor_id, item_id, quantity) VALUES (?1, ?2, ?3)",
            rusqlite::params![armor_id, item_id, qty],
        )?;
    }

    Ok(())
}

fn seed_item_sources(conn: &Connection) -> Result<()> {
    let sources: &[(i32, &str, Option<i32>, i32, i32, f64, &str)] = &[
        (6, "carve", Some(1), 1, 2, 0.40, "Forest and Hills"),
        (6, "carve", Some(1), 1, 1, 0.35, "Volcano"),
        (6, "quest_reward", Some(3), 1, 2, 0.20, "Mezeporta"),
        (6, "shiny", None, 1, 1, 0.15, "Forest and Hills"),
        (7, "carve", Some(1), 1, 1, 0.25, "Volcano"),
        (7, "carve", Some(1), 1, 2, 0.30, "Forest and Hills"),
        (7, "quest_reward", Some(3), 1, 1, 0.18, "Mezeporta"),
        (8, "carve", Some(1), 1, 1, 0.05, "Volcano"),
        (8, "carve", Some(1), 1, 1, 0.03, "Forest and Hills"),
        (8, "quest_reward", Some(3), 1, 1, 0.07, "Mezeporta"),
        (9, "carve", Some(3), 1, 2, 0.42, "Sandy Plains"),
        (9, "carve", Some(3), 1, 1, 0.35, "Desert"),
        (9, "quest_reward", Some(4), 1, 2, 0.20, "Mezeporta"),
        (10, "carve", Some(3), 1, 1, 0.30, "Sandy Plains"),
        (10, "carve", Some(3), 1, 1, 0.25, "Desert"),
        (11, "carve", Some(3), 1, 1, 0.28, "Sandy Plains"),
        (11, "carve", Some(3), 1, 1, 0.22, "Desert"),
        (12, "carve", Some(4), 1, 2, 0.45, "Jungle"),
        (12, "carve", Some(4), 1, 1, 0.30, "Forest and Hills"),
        (12, "quest_reward", Some(5), 1, 1, 0.22, "Mezeporta"),
        (13, "carve", Some(4), 1, 1, 0.32, "Jungle"),
        (13, "carve", Some(4), 1, 2, 0.35, "Forest and Hills"),
        (14, "carve", Some(6), 1, 2, 0.40, "Tundra"),
        (14, "carve", Some(6), 1, 1, 0.30, "Volcano"),
        (14, "shiny", None, 1, 1, 0.18, "Tundra"),
        (15, "carve", Some(6), 1, 1, 0.35, "Tundra"),
        (15, "carve", Some(6), 1, 2, 0.30, "Volcano"),
        (16, "carve", Some(16), 1, 1, 0.38, "Infernal Mountain"),
        (16, "shiny", None, 1, 1, 0.20, "Infernal Mountain"),
        (17, "carve", Some(16), 1, 1, 0.30, "Infernal Mountain"),
        (18, "carve", Some(17), 1, 2, 0.42, "Temple Ruins"),
        (19, "carve", Some(17), 1, 1, 0.25, "Temple Ruins"),
        (19, "quest_reward", Some(7), 1, 1, 0.18, "Mezeporta"),
        (20, "carve", Some(24), 1, 2, 0.40, "Castle Schrade"),
        (20, "quest_reward", Some(8), 1, 1, 0.20, "Castle Schrade"),
        (21, "carve", Some(24), 1, 1, 0.15, "Castle Schrade"),
        (21, "quest_reward", Some(8), 1, 1, 0.12, "Castle Schrade"),
        (22, "carve", Some(1), 1, 3, 0.55, "Any hunt"),
        (22, "carve", Some(2), 1, 3, 0.50, "Any hunt"),
        (22, "carve", Some(3), 1, 3, 0.55, "Any hunt"),
        (23, "mining", None, 1, 2, 0.40, "Volcano"),
        (23, "mining", None, 1, 1, 0.30, "Tundra"),
        (24, "mining", None, 1, 1, 0.45, "Volcano"),
        (24, "mining", None, 1, 1, 0.30, "Desert"),
        (25, "mining", None, 1, 1, 0.15, "Infernal Mountain"),
        (26, "gather", None, 1, 2, 0.65, "Forest and Hills"),
        (26, "gather", None, 1, 1, 0.55, "Moga Woods"),
        (27, "gather", None, 1, 1, 0.40, "Forest and Hills"),
        (28, "mining", None, 1, 2, 0.35, "Infernal Mountain"),
        (28, "mining", None, 1, 1, 0.25, "Temple Ruins"),
        (29, "mining", None, 1, 2, 0.50, "Volcano"),
        (30, "carve", Some(8), 1, 1, 0.40, "Desert"),
        (30, "carve", Some(8), 1, 2, 0.35, "Sandy Plains"),
        (31, "carve", Some(10), 1, 1, 0.45, "Forest and Hills"),
        (31, "carve", Some(10), 1, 2, 0.40, "Jungle"),
    ];

    for (item_id, source_type, source_id, qmin, qmax, prob, location) in sources {
        conn.execute(
            "INSERT OR IGNORE INTO item_sources (item_id, source_type, source_id, quantity_min, quantity_max, probability, location)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![item_id, source_type, source_id, qmin, qmax, prob, location],
        )?;
    }

    Ok(())
}

fn seed_item_combine(conn: &Connection) -> Result<()> {
    let recipes: &[(i32, i32, i32, i32)] = &[
        (2, 1, 1, 1),
        (2, 26, 1, 1),
        (5, 27, 1, 1),
        (5, 2, 1, 1),
    ];

    for (result, component, qty, result_qty) in recipes {
        conn.execute(
            "INSERT OR IGNORE INTO item_combine (result_item_id, component_item_id, quantity, result_quantity)
             VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![result, component, qty, result_qty],
        )?;
    }

    Ok(())
}
