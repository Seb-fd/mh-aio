export interface GameTheme {
  primary: string
  primaryDark: string
  accent: string
  accentSoft: string
  bgBase: string
  bgSurface: string
  bgElevated: string
  border: string
  borderStrong: string
  textAccent: string
  /** Secondary text (mimics gray-400 role, harmonized per game for AA on surfaces). */
  textMuted: string
  textOnPrimary: string
  bannerFrom: string
  bannerTo: string
  glow: string
  /** Focus-ring color (bright variant of primary for visibility). */
  ring: string
  ornament: 'japanese' | 'seigaiha' | 'medieval' | 'tribal' | 'futuristic' | 'hunt' | 'default'
  /** F0 infra: optional per-game display-font override (F2 ornaments may set
      a distinct voice per era; defaults to global --font-display). */
  fontDisplayOverride?: string
  /** Generation numeral rendered as archive backdrop (Guild Archive voice). */
  sigil: string
}

export interface Game {
  id: string
  dbId: number
  name: string
  shortName: string
  year: number
  platform: string
  theme: GameTheme
  iconUrl?: string | null
}

export const GAMES: Game[] = [
  {
    id: 'mhw',
    dbId: 1,
    name: 'Monster Hunter World',
    shortName: 'MHW',
    year: 2018,
    platform: 'PS4 / XB1 / PC',
    theme: {
      primary: '#3b82f6',
      primaryDark: '#1e3a8a',
      accent: '#fbbf24',
      accentSoft: 'rgba(251, 191, 36, 0.2)',
      bgBase: '#070b16',
      bgSurface: '#0d1526',
      bgElevated: '#16233d',
      border: '#1a2740',
      borderStrong: '#3b5a8f',
      textAccent: '#7fb3ff',
      textMuted: '#93a1b8',
      textOnPrimary: '#0a0f1e',
      bannerFrom: '#1e3a8a',
      bannerTo: '#070b16',
      glow: 'rgba(59, 130, 246, 0.4)',
      ring: '#60a5fa',
      ornament: 'tribal',
      sigil: 'V',
    },
  },
  {
    id: 'mhr',
    dbId: 2,
    name: 'Monster Hunter Rise',
    shortName: 'MHR',
    year: 2021,
    platform: 'Switch / PC',
    theme: {
      primary: '#f97316',
      primaryDark: '#7c2d12',
      accent: '#fde047',
      accentSoft: 'rgba(253, 224, 71, 0.2)',
      bgBase: '#0d0705',
      bgSurface: '#1a120c',
      bgElevated: '#2a1e14',
      border: '#2b2015',
      borderStrong: '#5c4632',
      textAccent: '#fdba74',
      textMuted: '#b39a82',
      textOnPrimary: '#1c1917',
      bannerFrom: '#7c2d12',
      bannerTo: '#0d0705',
      glow: 'rgba(249, 115, 22, 0.4)',
      ring: '#fb923c',
      ornament: 'japanese',
      sigil: 'VI',
    },
  },
  {
    id: 'mhwilds',
    dbId: 3,
    name: 'Monster Hunter Wilds',
    shortName: 'MH Wilds',
    year: 2025,
    platform: 'PS5 / XB / PC',
    theme: {
      primary: '#22c55e',
      primaryDark: '#14532d',
      accent: '#facc15',
      accentSoft: 'rgba(250, 204, 21, 0.2)',
      bgBase: '#060c08',
      bgSurface: '#0d1811',
      bgElevated: '#16281d',
      border: '#182a1f',
      borderStrong: '#35684a',
      textAccent: '#6ee7a0',
      textMuted: '#93b09a',
      textOnPrimary: '#0a0e0a',
      bannerFrom: '#14532d',
      bannerTo: '#060c08',
      glow: 'rgba(34, 197, 94, 0.4)',
      ring: '#4ade80',
      ornament: 'futuristic',
      sigil: 'VII',
    },
  },
  {
    id: 'mhp3rd',
    dbId: 4,
    name: 'MH Portable 3rd',
    shortName: 'MHP3rd',
    year: 2010,
    platform: 'PSP / PS3',
    theme: {
      primary: '#a855f7',
      primaryDark: '#581c87',
      accent: '#fbbf24',
      accentSoft: 'rgba(251, 191, 36, 0.2)',
      bgBase: '#090612',
      bgSurface: '#131022',
      bgElevated: '#1f1a36',
      border: '#211b38',
      borderStrong: '#4a3670',
      textAccent: '#d0a9ff',
      textMuted: '#ab9dc9',
      textOnPrimary: '#150826',
      bannerFrom: '#581c87',
      bannerTo: '#090612',
      glow: 'rgba(168, 85, 247, 0.4)',
      ring: '#c084fc',
      ornament: 'seigaiha',
      sigil: 'III',
    },
  },
  {
    id: 'mh2g',
    dbId: 5,
    name: 'MH 2ndG (Freedom Unite)',
    shortName: 'MH2G',
    year: 2008,
    platform: 'PSP',
    iconUrl: '/icons/games/mhp2ndg.png',
    theme: {
      primary: '#b91c1c',
      primaryDark: '#7f1d1d',
      accent: '#d4a017',
      accentSoft: 'rgba(212, 160, 23, 0.2)',
      bgBase: '#120808',
      bgSurface: '#1e0f0f',
      bgElevated: '#301414',
      border: '#3d1e1e',
      borderStrong: '#933838',
      textAccent: '#fca5a5',
      textMuted: '#c09696',
      textOnPrimary: '#fff8e7',
      bannerFrom: '#7f1d1d',
      bannerTo: '#120808',
      glow: 'rgba(185, 28, 28, 0.5)',
      ring: '#f87171',
      ornament: 'medieval',
      sigil: 'II',
    },
  },
]

import { writable } from 'svelte/store'
import { browser } from '$app/environment'

function isGame(value: unknown): value is Game {
  if (!value || typeof value !== 'object') return false
  const v = value as Record<string, unknown>
  return typeof v.id === 'string' && typeof v.dbId === 'number' && typeof v.name === 'string'
}

function parseStoredGame(raw: string | null): Game | null {
  if (!raw) return null
  try {
    const parsed = JSON.parse(raw)
    // Guard against stale/corrupt localStorage that would otherwise break module init.
    if (!isGame(parsed)) return null
    // Ensure the stored game still exists in the current registry.
    return GAMES.find((g) => g.id === parsed.id && g.dbId === parsed.dbId) ?? null
  } catch {
    return null
  }
}

function createGameStore() {
  const stored = browser ? localStorage.getItem('selectedGame') : null
  const initial = parseStoredGame(stored)

  const { subscribe, set } = writable<Game | null>(initial)

  return {
    subscribe,
    select: (game: Game) => {
      if (browser) localStorage.setItem('selectedGame', JSON.stringify(game))
      set(game)
    },
    clear: () => {
      if (browser) localStorage.removeItem('selectedGame')
      set(null)
    },
    getById: (id: string): Game | undefined => {
      return GAMES.find((g) => g.id === id)
    },
  }
}

export const selectedGame = createGameStore()
