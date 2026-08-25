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

export interface MonsterDrop {
  id: number;
  monster_id: number;
  item_id: number;
  item_name: string;
  method: string;
  part: string | null;
  rank: string | null;
  quantity: number;
  probability: number;
  condition: string | null;
}

export interface MonsterDetail {
  id: number;
  game_id: number;
  name: string;
  species: string | null;
  size: string | null;
  description: string | null;
  weaknesses: MonsterWeakness[];
  drops: MonsterDrop[];
  armor: Armor[];
  weapons: Weapon[];
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
  sharpness: string | null;
  slots: string | null;
  status_type: string | null;
  status_value: number | null;
  defense_bonus: number | null;
  crafting_cost: number | null;
  upgrade_path: string | null;
  is_forgeable: boolean;
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
  sharpness: string | null;
  slots: string | null;
  skills: string | null;
  status_type: string | null;
  status_value: number | null;
  defense_bonus: number | null;
  crafting_cost: number | null;
  upgrade_path: string | null;
  description: string | null;
  materials: MaterialRef[];
  forge_materials: MaterialRef[];
  upgrade_materials: MaterialRef[];
  is_forgeable: boolean;
  language: string;
}

export interface ArmorSet {
  id: number;
  game_id: number;
  name: string;
  piece_count: number;
  rank: string | null;
  rarity: number | null;
  language: string;
}
export interface ArmorSetDetail {
  id: number;
  game_id: number;
  name: string;
  pieces: Armor[];
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
  slots: string | null;
  skills: string | null;
  armor_type: string | null;
  set_id: number | null;
  gender: string | null;
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
  slots: string | null;
  skills: string | null;
  set_id: number | null;
  armor_type: string | null;
  gender: string | null;
  crafting_cost: number | null;
  description: string | null;
  materials: MaterialRef[];
  language: string;
}

export interface Quest {
  id: number;
  game_id: number;
  name: string;
  name_original: string | null;
  type: string | null;
  rank: string | null;
  hub: string | null;
  stars: number | null;
  objective: string | null;
  location: string | null;
  time_limit: number | null;
  faints_allowed: number | null;
  is_key_quest: boolean;
  client: string | null;
  requirements: string | null;
  reward_money: number | null;
  contract_fee: number | null;
  main_monsters: string | null;
  language: string;
}

export interface QuestReward {
  id: number;
  item_id: number;
  item_name: string;
  quantity: number;
  probability: number;
  condition: string | null;
}

export interface QuestDetail {
  id: number;
  game_id: number;
  name: string;
  name_original: string | null;
  type: string | null;
  rank: string | null;
  hub: string | null;
  stars: number | null;
  objective: string | null;
  location: string | null;
  time_limit: number | null;
  faints_allowed: number | null;
  is_key_quest: boolean;
  description: string | null;
  client: string | null;
  requirements: string | null;
  reward_money: number | null;
  contract_fee: number | null;
  main_monsters: string | null;
  rewards: QuestReward[];
  language: string;
}

export interface Item {
  id: number;
  game_id: number;
  name: string;
  category: string | null;
  rarity: number | null;
  sell_price: number | null;
  buy_price: number | null;
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
  rank: string | null;
  part: string | null;
  condition: string | null;
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
  buy_price: number | null;
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

export interface SkillLevel {
  id: number;
  points: number;
  ability_name: string;
  description: string | null;
}

export interface DecoMaterial {
  item_id: number | null;
  item_name: string;
  quantity: number;
}

export interface SkillDecoration {
  id: number;
  name: string;
  slot_size: number | null;
  skill_points: number;
  secondary_skill_name: string | null;
  secondary_points: number | null;
  price: number | null;
  rarity: number | null;
  materials: DecoMaterial[];
  unlock: string;
  acquisition: string;
}

export interface Decoration {
  id: number;
  game_id: number;
  name: string;
  skill_id: number | null;
  skill_name: string | null;
  skill_points: number | null;
  secondary_skill_id: number | null;
  secondary_skill_name: string | null;
  secondary_points: number | null;
  slot_size: number | null;
  rarity: number | null;
  price: number | null;
  language: string;
}

export interface DecorationDetail {
  id: number;
  game_id: number;
  name: string;
  skill_id: number | null;
  skill_name: string | null;
  skill_points: number | null;
  secondary_skill_id: number | null;
  secondary_skill_name: string | null;
  secondary_points: number | null;
  slot_size: number | null;
  rarity: number | null;
  price: number | null;
  language: string;
  materials: DecoMaterial[];
  unlock: string;
  acquisition: string;
}

export interface SkillArmorRef {
  id: number;
  name: string;
  slot_type: string;
  rank: string;
  rarity: number | null;
  defense_base: number | null;
  defense_max: number | null;
  slots: string | null;
  points: number;
}

export interface SkillWeaponRef {
  id: number;
  name: string;
  weapon_type: string;
  rarity: number | null;
  attack: number | null;
  slots: string | null;
  points: number;
}

export interface SkillDetail {
  id: number;
  game_id: number;
  name: string;
  description: string | null;
  max_level: number | null;
  language: string;
  levels: SkillLevel[];
  decorations: SkillDecoration[];
  armors: SkillArmorRef[];
  weapons: SkillWeaponRef[];
}

export interface SearchResult {
  kind: string;
  id: number;
  name: string;
  subtitle: string;
  route: string;
}

export interface AssSkillReq {
  skill_id: number;
  points_required: number;
}
export interface AssQueryInput {
  game_id: number;
  skills: AssSkillReq[];
  hunter_type: string;
  gender: string;
  hr: number;
  elder_star: number;
  weapon_slots: number;
  include_piercings: boolean;
  allow_bad: boolean;
  allow_torso_inc: boolean;
  allow_dummy: boolean;
  sort_by: string | null;
}
export interface AssArmorRef {
  id: number;
  name: string;
  slot_type: string;
  rarity: number | null;
  defense_base: number | null;
  slots: string | null;
  skills: string | null;
}
export interface AssDecorationRef {
  id: number;
  name: string;
  slot_size: number | null;
  skill_name: string | null;
  skill_points: number | null;
  secondary_skill_name: string | null;
  secondary_points: number | null;
  count: number;
}
export interface AssSolutionView {
  armors: AssArmorRef[];
  decorations: AssDecorationRef[];
  extra_skills: string[];
  defense: number;
  fire_res: number;
  water_res: number;
  thunder_res: number;
  ice_res: number;
  dragon_res: number;
  rarity: number;
  difficulty: number;
  slots_spare: number;
  slots_spare_detail: number[];
}

export const api = {
  getGames: () => invoke<Game[]>('get_games'),
  getMonsters: (gameId: number) => invoke<Monster[]>('get_monsters', { gameId }),
  getWeapons: (gameId: number) => invoke<Weapon[]>('get_weapons', { gameId }),
  getArmor: (gameId: number) => invoke<Armor[]>('get_armor', { gameId }),
  getArmorSets: (gameId: number) => invoke<ArmorSet[]>('get_armor_sets', { gameId }),
  getArmorSetDetail: (id: number) => invoke<ArmorSetDetail | null>('get_armor_set_detail', { id }),
  getQuests: (gameId: number) => invoke<Quest[]>('get_quests', { gameId }),
  getItems: (gameId: number) => invoke<Item[]>('get_items', { gameId }),
  getSkills: (gameId: number) => invoke<Skill[]>('get_skills', { gameId }),
  getDecorations: (gameId: number) => invoke<Decoration[]>('get_decorations', { gameId }),
  getMonsterDetail: (id: number) => invoke<MonsterDetail | null>('get_monster_detail', { id }),
  getMonsterDedicatedSets: (monsterId: number, rank: string | null) => invoke<ArmorSetDetail[]>('get_monster_dedicated_sets', { monsterId, rank }),
  getWeaponDetail: (id: number) => invoke<WeaponDetail | null>('get_weapon_detail', { id }),
  getArmorDetail: (id: number) => invoke<ArmorDetail | null>('get_armor_detail', { id }),
  getQuestDetail: (id: number) => invoke<QuestDetail | null>('get_quest_detail', { id }),
  getItemDetail: (id: number) => invoke<ItemDetail | null>('get_item_detail', { id }),
  getSkillDetail: (id: number) => invoke<SkillDetail | null>('get_skill_detail', { id }),
  getDecorationDetail: (id: number) => invoke<DecorationDetail | null>('get_decoration_detail', { id }),
  searchArmorSets: (query: AssQueryInput) => invoke<AssSolutionView[]>('search_armor_sets', { query }),
  globalSearch: (gameId: number, query: string) => invoke<SearchResult[]>('global_search', { gameId, query }),
};
