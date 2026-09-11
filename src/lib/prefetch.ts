import { getQueryClient } from '$lib/query-client'
import { api } from '$lib/api'
import { dbCache } from '$lib/query'

const SECTION_JOBS: Record<string, [key: string, run: (dbId: number) => Promise<unknown>][]> = {
  '/monsters': [['monsters', (id) => api.getMonsters(id)]],
  '/weapons': [['weapons', (id) => api.getWeapons(id)]],
  '/armor': [
    ['armor', (id) => api.getArmor(id)],
    ['armor-sets', (id) => api.getArmorSets(id)],
  ],
  '/quests': [['quests', (id) => api.getQuests(id)]],
  '/items': [['items', (id) => api.getItems(id)]],
  '/skills': [['skills', (id) => api.getSkills(id)]],
  '/decorations': [['decorations', (id) => api.getDecorations(id)]],
  '/tools': [
    ['mhw-mantles', (id) => api.getMhwMantles(id)],
    ['palico-gadgets', (id) => api.getPalicoGadgets(id)],
  ],
}

/**
 * Warm the TanStack cache for a section (hover/focus) so navigation renders
 * instantly. No-op when cached+fresh or when the section has no dataset
 * (home, builds). Local IPC: cheap and deduped while in flight.
 */
export function prefetchSection(href: string, dbId: number | null | undefined): void {
  if (dbId == null) return
  const client = getQueryClient()
  for (const [key, run] of SECTION_JOBS[href] ?? []) {
    void client.prefetchQuery({ queryKey: [key, dbId], queryFn: () => run(dbId), ...dbCache })
  }
}
