export interface Game {
  id: string;
  name: string;
  shortName: string;
  year: number;
  platform: string;
  color: string;
  borderColor: string;
  bgHover: string;
}

export const GAMES: Game[] = [
  {
    id: 'mhw',
    name: 'Monster Hunter World',
    shortName: 'MHW',
    year: 2018,
    platform: 'PS4 / XB1 / PC',
    color: 'text-blue-400',
    borderColor: 'border-blue-500/50 hover:border-blue-400',
    bgHover: 'hover:bg-blue-500/10'
  },
  {
    id: 'mhr',
    name: 'Monster Hunter Rise',
    shortName: 'MHR',
    year: 2021,
    platform: 'Switch / PC',
    color: 'text-orange-400',
    borderColor: 'border-orange-500/50 hover:border-orange-400',
    bgHover: 'hover:bg-orange-500/10'
  },
  {
    id: 'mhwilds',
    name: 'Monster Hunter Wilds',
    shortName: 'MH Wilds',
    year: 2025,
    platform: 'PS5 / XB / PC',
    color: 'text-green-400',
    borderColor: 'border-green-500/50 hover:border-green-400',
    bgHover: 'hover:bg-green-500/10'
  },
  {
    id: 'mhp3rd',
    name: 'MH Portable 3rd',
    shortName: 'MHP3rd',
    year: 2010,
    platform: 'PSP / PS3',
    color: 'text-purple-400',
    borderColor: 'border-purple-500/50 hover:border-purple-400',
    bgHover: 'hover:bg-purple-500/10'
  },
  {
    id: 'mh2g',
    name: 'MH 2ndG (Freedom Unite)',
    shortName: 'MH2G',
    year: 2008,
    platform: 'PSP',
    color: 'text-red-400',
    borderColor: 'border-red-500/50 hover:border-red-400',
    bgHover: 'hover:bg-red-500/10'
  }
];

import { writable } from 'svelte/store';
import { browser } from '$app/environment';

function createGameStore() {
  const stored = browser ? localStorage.getItem('selectedGame') : null;
  const initial = stored ? JSON.parse(stored) : null;

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
