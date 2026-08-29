export interface GameTheme {
  primary: string;
  primaryDark: string;
  accent: string;
  accentSoft: string;
  bgBase: string;
  bgSurface: string;
  bgElevated: string;
  border: string;
  borderStrong: string;
  textAccent: string;
  textOnPrimary: string;
  bannerFrom: string;
  bannerTo: string;
  glow: string;
  ornament: 'japanese' | 'medieval' | 'tribal' | 'futuristic' | 'hunt' | 'default';
}

export interface Game {
  id: string;
  dbId: number;
  name: string;
  shortName: string;
  year: number;
  platform: string;
  color: string;
  borderColor: string;
  bgHover: string;
  theme: GameTheme;
}

export const GAMES: Game[] = [
  {
    id: 'mhw',
    dbId: 1,
    name: 'Monster Hunter World',
    shortName: 'MHW',
    year: 2018,
    platform: 'PS4 / XB1 / PC',
    color: 'text-blue-400',
    borderColor: 'border-blue-500/50 hover:border-blue-400',
    bgHover: 'hover:bg-blue-500/10',
    theme: {
      primary: '#3b82f6',
      primaryDark: '#1e3a8a',
      accent: '#fbbf24',
      accentSoft: 'rgba(251, 191, 36, 0.2)',
      bgBase: '#0a0e1a',
      bgSurface: '#0f172a',
      bgElevated: '#1e293b',
      border: '#1e293b',
      borderStrong: '#334155',
      textAccent: '#60a5fa',
      textOnPrimary: '#ffffff',
      bannerFrom: '#1e3a8a',
      bannerTo: '#0a0e1a',
      glow: 'rgba(59, 130, 246, 0.4)',
      ornament: 'tribal',
    },
  },
  {
    id: 'mhr',
    dbId: 2,
    name: 'Monster Hunter Rise',
    shortName: 'MHR',
    year: 2021,
    platform: 'Switch / PC',
    color: 'text-orange-400',
    borderColor: 'border-orange-500/50 hover:border-orange-400',
    bgHover: 'hover:bg-orange-500/10',
    theme: {
      primary: '#f97316',
      primaryDark: '#7c2d12',
      accent: '#fde047',
      accentSoft: 'rgba(253, 224, 71, 0.2)',
      bgBase: '#0f0a07',
      bgSurface: '#1c1917',
      bgElevated: '#292524',
      border: '#292524',
      borderStrong: '#44403c',
      textAccent: '#fb923c',
      textOnPrimary: '#1c1917',
      bannerFrom: '#7c2d12',
      bannerTo: '#0f0a07',
      glow: 'rgba(249, 115, 22, 0.4)',
      ornament: 'japanese',
    },
  },
  {
    id: 'mhwilds',
    dbId: 3,
    name: 'Monster Hunter Wilds',
    shortName: 'MH Wilds',
    year: 2025,
    platform: 'PS5 / XB / PC',
    color: 'text-green-400',
    borderColor: 'border-green-500/50 hover:border-green-400',
    bgHover: 'hover:bg-green-500/10',
    theme: {
      primary: '#22c55e',
      primaryDark: '#14532d',
      accent: '#facc15',
      accentSoft: 'rgba(250, 204, 21, 0.2)',
      bgBase: '#070d09',
      bgSurface: '#0f1a14',
      bgElevated: '#1a2e22',
      border: '#1a2e22',
      borderStrong: '#2d5c3e',
      textAccent: '#4ade80',
      textOnPrimary: '#0a0e0a',
      bannerFrom: '#14532d',
      bannerTo: '#070d09',
      glow: 'rgba(34, 197, 94, 0.4)',
      ornament: 'futuristic',
    },
  },
  {
    id: 'mhp3rd',
    dbId: 4,
    name: 'MH Portable 3rd',
    shortName: 'MHP3rd',
    year: 2010,
    platform: 'PSP / PS3',
    color: 'text-purple-400',
    borderColor: 'border-purple-500/50 hover:border-purple-400',
    bgHover: 'hover:bg-purple-500/10',
    theme: {
      primary: '#a855f7',
      primaryDark: '#581c87',
      accent: '#fbbf24',
      accentSoft: 'rgba(251, 191, 36, 0.2)',
      bgBase: '#0a0712',
      bgSurface: '#14101e',
      bgElevated: '#1f1a2e',
      border: '#1f1a2e',
      borderStrong: '#3b2c5c',
      textAccent: '#c084fc',
      textOnPrimary: '#ffffff',
      bannerFrom: '#581c87',
      bannerTo: '#0a0712',
      glow: 'rgba(168, 85, 247, 0.4)',
      ornament: 'japanese',
    },
  },
  {
    id: 'mh2g',
    dbId: 5,
    name: 'MH 2ndG (Freedom Unite)',
    shortName: 'MH2G',
    year: 2008,
    platform: 'PSP',
    color: 'text-red-400',
    borderColor: 'border-red-500/50 hover:border-red-400',
    bgHover: 'hover:bg-red-500/10',
    theme: {
      primary: '#b91c1c',
      primaryDark: '#7f1d1d',
      accent: '#d4a017',
      accentSoft: 'rgba(212, 160, 23, 0.2)',
      bgBase: '#150a0a',
      bgSurface: '#1f1010',
      bgElevated: '#2c1616',
      border: '#3a1c1c',
      borderStrong: '#7f1d1d',
      textAccent: '#f87171',
      textOnPrimary: '#fff8e7',
      bannerFrom: '#7f1d1d',
      bannerTo: '#150a0a',
      glow: 'rgba(185, 28, 28, 0.5)',
      ornament: 'medieval',
    },
  },
];

import { writable } from 'svelte/store';
import { browser } from '$app/environment';

function isGame(value: unknown): value is Game {
  if (!value || typeof value !== 'object') return false;
  const v = value as Record<string, unknown>;
  return typeof v.id === 'string' && typeof v.dbId === 'number' && typeof v.name === 'string';
}

function parseStoredGame(raw: string | null): Game | null {
  if (!raw) return null;
  try {
    const parsed = JSON.parse(raw);
    // Guard against stale/corrupt localStorage that would otherwise break module init.
    if (!isGame(parsed)) return null;
    // Ensure the stored game still exists in the current registry.
    return GAMES.find((g) => g.id === parsed.id && g.dbId === parsed.dbId) ?? null;
  } catch {
    return null;
  }
}

function createGameStore() {
  const stored = browser ? localStorage.getItem('selectedGame') : null;
  const initial = parseStoredGame(stored);

  const { subscribe, set, update } = writable<Game | null>(initial);

  return {
    subscribe,
    select: (game: Game) => {
      if (browser) localStorage.setItem('selectedGame', JSON.stringify(game));
      set(game);
    },
    clear: () => {
      if (browser) localStorage.removeItem('selectedGame');
      set(null);
    },
    getById: (id: string): Game | undefined => {
      return GAMES.find(g => g.id === id);
    }
  };
}

export const selectedGame = createGameStore();
