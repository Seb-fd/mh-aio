import { writable, derived, type Readable } from 'svelte/store'
import { api, type Favorite, type FavoriteKind } from '$lib/api'

function favKey(kind: FavoriteKind, id: number): string {
  return `${kind}:${id}`
}

/** Per-game favorites cache, backed by app-data via Tauri commands. */
function createFavoritesStore() {
  const inner = writable<Map<number, Map<string, Favorite>>>(new Map())
  const loading = writable(false)

  async function ensure(gameId: number): Promise<void> {
    let has = false
    inner.update((m) => {
      has = m.has(gameId)
      return m
    })
    if (has) return
    loading.set(true)
    try {
      const list = await api.listFavorites(gameId)
      inner.update((m) => {
        const next = new Map(m)
        next.set(gameId, new Map(list.map((f) => [favKey(f.kind, f.id), f])))
        return next
      })
    } finally {
      loading.set(false)
    }
  }

  function refresh(gameId: number): Promise<void> {
    inner.update((m) => {
      const next = new Map(m)
      next.delete(gameId)
      return next
    })
    return ensure(gameId)
  }

  return {
    subscribe: inner.subscribe,
    loading: { subscribe: loading.subscribe } as Readable<boolean>,
    ensure,
    refresh,
    isFav(gameId: number, kind: FavoriteKind, id: number): Readable<boolean> {
      return derived(inner, ($m) => $m.get(gameId)?.has(favKey(kind, id)) ?? false)
    },
    async toggle(gameId: number, kind: FavoriteKind, id: number, name: string): Promise<boolean> {
      const state = await api.toggleFavorite(gameId, kind, id, name)
      await refresh(gameId)
      return state
    },
    async remove(gameId: number, kind: FavoriteKind, id: number): Promise<void> {
      await api.removeFavorite(gameId, kind, id)
      await refresh(gameId)
    },
  }
}

export const favorites = createFavoritesStore()
