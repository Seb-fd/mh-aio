import { invoke } from '@tauri-apps/api/core';

export interface Game {
  id: number;
  name: string;
  abbreviation: string;
  release_year: number | null;
  platform: string | null;
}

export interface Monster {
  id: number;
  game_id: number;
  name: string;
  species: string | null;
  size: string | null;
  language: string;
}

export interface MonsterWeakness {
  id: number;
  part_name: string;
  sever: number | null;
  blunt: number | null;
  projectile: number | null;
  fire: number | null;
  water: number | null;
  thunder: number | null;
  ice: number | null;
  dragon: number | null;
}

export interface MonsterDetail {
  id: number;
  game_id: number;
  name: string;
  species: string | null;
  size: string | null;
  description: string | null;
  weaknesses: MonsterWeakness[];
  language: string;
}

export interface Weapon {
  id: number;
  game_id: number;
  name: string;
  weapon_type: string;
  rarity: number | null;
  attack: number | null;
  affinity: number | null;
  element_type: string | null;
  element_value: number | null;
  language: string;
}

export interface MaterialRef {
  item_id: number;
  item_name: string;
  quantity: number;
}

export interface WeaponDetail {
  id: number;
  game_id: number;
  name: string;
  weapon_type: string;
  rarity: number | null;
  attack: number | null;
  affinity: number | null;
  element_type: string | null;
  element_value: number | null;
  crafting_cost: number | null;
  description: string | null;
  materials: MaterialRef[];
  language: string;
}

export interface Armor {
  id: number;
  game_id: number;
  name: string;
  slot_type: string;
  rank: string;
  rarity: number | null;
  defense_base: number | null;
  defense_max: number | null;
  resistance_fire: number | null;
  resistance_water: number | null;
  resistance_thunder: number | null;
  resistance_ice: number | null;
  resistance_dragon: number | null;
  language: string;
}

export interface ArmorDetail {
  id: number;
  game_id: number;
  name: string;
  slot_type: string;
  rank: string;
  rarity: number | null;
  defense_base: number | null;
  defense_max: number | null;
  resistance_fire: number | null;
  resistance_water: number | null;
  resistance_thunder: number | null;
  resistance_ice: number | null;
  resistance_dragon: number | null;
  crafting_cost: number | null;
  description: string | null;
  materials: MaterialRef[];
  language: string;
}

export interface Quest {
  id: number;
  game_id: number;
  name: string;
  type: string | null;
  rank: string | null;
  objective: string | null;
  location: string | null;
  time_limit: number | null;
  faints_allowed: number | null;
  is_key_quest: boolean;
  language: string;
}

export interface QuestDetail {
  id: number;
  game_id: number;
  name: string;
  type: string | null;
  rank: string | null;
  objective: string | null;
  location: string | null;
  time_limit: number | null;
  faints_allowed: number | null;
  is_key_quest: boolean;
  description: string | null;
  language: string;
}

export interface Item {
  id: number;
  game_id: number;
  name: string;
  category: string | null;
  rarity: number | null;
  sell_price: number | null;
  description: string | null;
  language: string;
}

export interface ItemSource {
  id: number;
  source_type: string;
  source_id: number | null;
  source_name: string | null;
  quantity_min: number | null;
  quantity_max: number | null;
  probability: number | null;
  location: string | null;
}

export interface CombineRecipe {
  component_item_id: number;
  component_name: string;
  quantity: number;
  result_quantity: number;
}

export interface ItemDetail {
  id: number;
  game_id: number;
  name: string;
  category: string | null;
  rarity: number | null;
  sell_price: number | null;
  description: string | null;
  sources: ItemSource[];
  recipes: CombineRecipe[];
  language: string;
}

export interface Skill {
  id: number;
  game_id: number;
  name: string;
  description: string | null;
  max_level: number | null;
  language: string;
}

export const api = {
  getGames: () => invoke<Game[]>('get_games'),
  getMonsters: (gameId: number) => invoke<Monster[]>('get_monsters', { gameId }),
  getWeapons: (gameId: number) => invoke<Weapon[]>('get_weapons', { gameId }),
  getArmor: (gameId: number) => invoke<Armor[]>('get_armor', { gameId }),
  getQuests: (gameId: number) => invoke<Quest[]>('get_quests', { gameId }),
  getItems: (gameId: number) => invoke<Item[]>('get_items', { gameId }),
  getSkills: (gameId: number) => invoke<Skill[]>('get_skills', { gameId }),
  getMonsterDetail: (id: number) => invoke<MonsterDetail | null>('get_monster_detail', { id }),
  getWeaponDetail: (id: number) => invoke<WeaponDetail | null>('get_weapon_detail', { id }),
  getArmorDetail: (id: number) => invoke<ArmorDetail | null>('get_armor_detail', { id }),
  getQuestDetail: (id: number) => invoke<QuestDetail | null>('get_quest_detail', { id }),
  getItemDetail: (id: number) => invoke<ItemDetail | null>('get_item_detail', { id }),
  getSkillDetail: (id: number) => invoke<Skill | null>('get_skill_detail', { id }),
};
