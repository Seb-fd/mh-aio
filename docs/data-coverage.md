# Data Coverage

**Generated file — do not hand-edit.** Regenerate with:

```bash
py scripts/gen_data_coverage.py            # from the installed DB
py scripts/gen_data_coverage.py --check    # CI drift check
```

Every row count quoted in `README.md`, `STATUS.md`, `roadmap.md`, `AGENTS.md`, `docs/fidelity-report.md` and `spec/constitution/*` must match this file. If you add data, regenerate rather than editing counts by hand.

## Game IDs

| `game_id` | slug      | game                     |
| --------- | --------- | ------------------------ |
| 1         | `mhw`     | Monster Hunter: World    |
| 2         | `mhr`     | Monster Hunter Rise      |
| 3         | `mhwilds` | Monster Hunter: Wilds    |
| 4         | `mhp3rd`  | Monster Hunter P3rd      |
| 5         | `mh2g`    | Monster Hunter Freedom Unite |

## Entity counts by game

| Table | mhw | mhr | mhwilds | mhp3rd | mh2g | Total |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `monsters` | 94 | 112 | 68 | 60 | 83 | 417 |
| `items` | 1359 | 1668 | 773 | 1044 | 1244 | 6088 |
| `weapons` | 3544 | 3953 | 1188 | 973 | 1500 | 11158 |
| `armor` | 1595 | 1591 | 714 | 1106 | 2075 | 7081 |
| `armor_sets` | 351 | 410 | 183 | 344 | 348 | 1636 |
| `skills` | 179 | 147 | 202 | 101 | 99 | 728 |
| `decorations` | 404 | 243 | 361 | 164 | 192 | 1364 |
| `quests` | 521 | 946 | 174 | 378 | 610 | 2629 |
| `melder_recipes` | 211 | 0 | 0 | 0 | 0 | 211 |
| `charms` | 0 | 11849 | 76 | 50635 | 0 | 62560 |
| `endemic_life` | 33 | 0 | 70 | 0 | 0 | 103 |

### Junction + derived tables

| Table | mhw | mhr | mhwilds | mhp3rd | mh2g | Total |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `quest_rewards` | 4176 | 7277 | 0 | 1862 | 1273 | 14588 |
| `monster_drops` | 5826 | 6532 | 1677 | 1957 | 3379 | 19371 |
| `monster_weaknesses` | 788 | 624 | 324 | 213 | 420 | 2369 |
| `monster_ailments` | 721 | 1120 | 330 | 120 | 214 | 2505 |
| `monster_tools` | 465 | 270 | 136 | 240 | 354 | 1465 |
| `monster_part_break` | 0 | 468 | 0 | 0 | 0 | 468 |
| `monster_equipment` | 20224 | 21920 | 3772 | 4458 | 9019 | 59393 |
| `weapon_materials` | 9719 | 12303 | 3615 | 2374 | 5137 | 33148 |
| `weapon_craft` | 10056 | 13490 | 3803 | 3829 | 6059 | 37237 |
| `weapon_coatings` | 1428 | 0 | 0 | 0 | 0 | 1428 |
| `armor_materials` | 5887 | 6040 | 2314 | 4130 | 8022 | 26393 |
| `skill_levels` | 419 | 459 | 442 | 211 | 214 | 1745 |
| `item_sources` | 6821 | 7198 | 1537 | 3856 | 15682 | 35094 |
| `item_combine` | 186 | 178 | 120 | 263 | 432 | 1179 |
| `decoration_materials` | 0 | 801 | 0 | 514 | 597 | 1912 |
| `charm_skills` | 0 | 23249 | 76 | 93873 | 0 | 117198 |
| `hunter_charm_materials` | 1247 | 0 | 0 | 0 | 0 | 1247 |
| `weapon_rampage` | 0 | 6822 | 0 | 0 | 0 | 6822 |
| `armor_skill_points` | 2372 | 2834 | 2119 | 4189 | 7124 | 18638 |
| `weapon_skill_points` | 638 | 0 | 1543 | 0 | 0 | 2181 |

### Game-exclusive tables

| Table | Game | Rows |
| --- | --- | ---: |
| `mhw_mantles` | mhw | 20 |
| `palico_gadgets` | mhw | 8 |
| `hunter_charms` | mhw | 317 |
| `buddy_weapons` | mhr | 322 |
| `buddy_armor` | mhr | 362 |
| `rampage_skills` | mhr | 247 |
| `rampage_decorations` | mhr | 26 |
| `comrade_weapons` | mhp3rd | 92 |
| `comrade_armor` | mhp3rd | 154 |
| `comrade_skills` | mhp3rd | 52 |

`palico_gadget_levels` (child of `palico_gadgets`): 38 rows.

`hunter_charm_materials` (child of `hunter_charms`, MHW rank-tree craft mats): 1247 rows.

`weapon_coatings` (child of `weapons`, MHW bow-coating compatibility): 1428 rows.

## `monsters.size` breakdown (exact stored value)

Case-sensitive — `Small` and `small` are stored differently per game.

| Game | size values |
| --- | --- |
| mhw | `Large` = 71, `Small` = 23 |
| mhr | `Large` = 78, `Small` = 34 |
| mhwilds | `small` = 34, `Large` = 34 |
| mhp3rd | `Large` = 40, `Small` = 20 |
| mh2g | `Large` = 54, `Small` = 25, `Giant` = 4 |

## `items.category` split

| Game | Breakdown |
| --- | --- |
| mhw | `Material` = 1082, `Consumable` = 129, `Misc` = 106, `Ammo` = 41, `Hidden` = 1 |
| mhr | `Material` = 1497, `Consumable` = 80, `Ammo` = 65, `Decoration` = 26 |
| mhwilds | `Material` = 773 |
| mhp3rd | `Material` = 943, `Consumable` = 55, `Ammo` = 46 |
| mh2g | `Material` = 1037, `Consumable` = 128, `Ammo` = 79 |

## `item_combine.combine_type` split

| Game | Breakdown |
| --- | --- |
| mhw | `normal` = 186 |
| mhr | `normal` = 178 |
| mhwilds | `normal` = 120 |
| mhp3rd | `normal` = 202, `alchemy` = 61 |
| mh2g | `normal` = 379, `alchemy` = 39, `treasure` = 14 |

## `charms.charm_type` split

| Game | Breakdown |
| --- | --- |
| mhw | _none_ |
| mhr | `talisman` = 11849 |
| mhwilds | `wilds_talisman` = 76 |
| mhp3rd | `dragon` = 10552, `king` = 22439, `queen` = 17644 |
| mh2g | _none_ |

## Quest hub / category split

| Game | Key | Breakdown |
| --- | --- | --- |
| mhw | `hub` | `master` = 260, `low_high` = 258, `siege` = 3 |
| mhw | `category` | `optional` = 253, `event` = 147, `assigned` = 53, `challenge` = 32, `special` = 17, `arena` = 16, `siege` = 3 |
| mhr | `hub` | `hub_high` = 199, `event` = 180, `hub_master` = 171, `village` = 119, `follower` = 104, `hub_low` = 83, `mystery` = 57, `arena` = 25, `challenge` = 7, `training` = 1 |
| mhr | `category` | `Hub` = 614, `Event` = 180, `Village` = 120, `Arena` = 25, `Challenge` = 7 |
| mhwilds | `category` | `Optional` = 81, `Event` = 60, `Assignment` = 33 |
| mhp3rd | `hub` | `guild_high` = 100, `village` = 96, `guild_low` = 88, `event` = 52, `drink` = 16, `training` = 10, `hot_spring` = 7, `challenge` = 6, `nyanta` = 3 |
| mhp3rd | `category` | `None` = 378 |
| mh2g | `hub` | `training` = 140, `elder` = 95, `guild_low` = 89, `guild_g` = 89, `guild_high` = 77, `nekoto` = 62, `event` = 37, `challenge` = 14, `treasure` = 7 |
| mh2g | `category` | `None` = 610 |

## Seed bookkeeping

| Field | Value |
| --- | --- |
| `schema_version` table | `1` |
| `data_patches` rows | `31` |

> **Note:** `src-tauri/src/db/seed.rs` declares `pub const DATA_VERSION: i32 = 1` while the live `schema_version` table holds a higher number. The `seed()` boot gate (`if current >= DATA_VERSION { return Ok(()) }`) is therefore permanently closed for upgraded installs — **all** post-v1 data lands through `apply_data_patches()` (tracked in `data_patches`), never through a version bump. See `AGENTS.md` Gotchas.

### Registered `data_patches` names

- `gaps_100_backfill`
- `item_icons_mhp3rd`
- `item_icons_mhr`
- `item_icons_mhwilds`
- `mh2g_drop_corrections`
- `mh2g_equipment_backfill`
- `mh2g_hunt_info`
- `mh2g_items_backfill`
- `mh2g_monster_drops_backfill`
- `mh2g_quests_fix`
- `mh2g_weaknesses_backfill`
- `mh2g_weapon_craft`
- `mhp3rd_ailments_tools`
- `mhp3rd_charms`
- `mhp3rd_comrades`
- `mhp3rd_drops_sync`
- `mhp3rd_gear_fix`
- `mhp3rd_items_fix`
- `mhp3rd_quests_keys`
- `mhp3rd_roundout`
- `mhp3rd_weaknesses_rewrite`
- `mhr_011_backfill`
- `mhr_buddy_prefix`
- `mhw_010_backfill`
- `mhw_015_endemic`
- `mhw_015_small_size`
- `mhwilds_012_014_backfill`
- `monster_thumbs_rise_wilds`
- `quest_category_backfill`
- `quest_icons_mhr`
- `quest_reward_variants`

## Tables present in the schema

Full list for reference when adding a table (new tables need a `(game_id, id)` UNIQUE index in `add_idempotency_constraints()`):

`armor`, `armor_materials`, `armor_sets`, `armor_skill_points`, `buddy_armor`, `buddy_weapons`, `charm_skills`, `charms`, `comrade_armor`, `comrade_skills`, `comrade_weapons`, `data_patches`, `decoration_materials`, `decorations`, `endemic_life`, `games`, `hunter_charm_materials`, `hunter_charms`, `item_combine`, `item_sources`, `items`, `melder_recipes`, `mhw_mantles`, `monster_ailments`, `monster_drops`, `monster_equipment`, `monster_part_break`, `monster_tools`, `monster_weaknesses`, `monsters`, `palico_gadget_levels`, `palico_gadgets`, `quest_rewards`, `quests`, `rampage_decorations`, `rampage_skills`, `schema_version`, `skill_levels`, `skills`, `weapon_coatings`, `weapon_craft`, `weapon_materials`, `weapon_rampage`, `weapon_skill_points`, `weapons`
