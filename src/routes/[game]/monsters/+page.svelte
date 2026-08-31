<script lang="ts">
  import { goto } from '$app/navigation'
  import { selectedGame } from '$lib/stores/game'
  import { api, type Monster } from '$lib/api'
  import Card from '$lib/components/ui/card.svelte'
  import ItemIcon from '$lib/components/item-icon.svelte'

  const game = $derived($selectedGame)
  const dbId = $derived(game?.dbId)

  let monsters = $state<Monster[]>([])
  let loading = $state(true)
  let error = $state<string | null>(null)

  async function loadMonsters(id: number, attempt = 0) {
    try {
      const data = await api.getMonsters(id)
      console.log('[monsters] loaded', data.length)
      monsters = data
      error = null
    } catch (e) {
      const msg = String(e)
      console.error('[monsters] failed', msg)
      if (msg.includes('state not managed') && attempt < 6) {
        error = 'Preparing database...'
        setTimeout(() => loadMonsters(id, attempt + 1), 400 * (attempt + 1))
        return
      }
      error = msg
    } finally {
      if (error !== 'Preparing database...') loading = false
    }
  }

  $effect(() => {
    if (dbId == null) return
    console.log('[monsters] loading gameId', dbId)
    loading = true
    error = null
    loadMonsters(dbId)
  })

  function open(id: number) {
    if (!game) return
    goto(`/${game.id}/monsters/${id}`)
  }

  let sizeFilter = $state<'large' | 'all' | 'small'>('large')

  const filteredMonsters = $derived(
    monsters.filter((m) => {
      const sz = (m.size ?? '').toLowerCase()
      if (sizeFilter === 'all') return true
      if (sizeFilter === 'small') return sz === 'small'
      // large includes Large + Giant (both considered large)
      return sz === 'large' || sz === 'giant'
    }),
  )

  function speciesColor(species: string | null): string {
    if (!species) return 'text-gray-400'
    if (species === 'Elder Dragon') return 'text-yellow-400'
    if (species === 'Flying Wyvern') return 'text-red-400'
    if (species === 'Bird Wyvern') return 'text-orange-400'
    if (species === 'Fanged Beast') return 'text-amber-400'
    if (species === 'Leviathan') return 'text-blue-400'
    if (species === 'Carapaceon') return 'text-cyan-400'
    if (species === 'Amphibian') return 'text-emerald-400'
    if (species === 'Herbivore') return 'text-green-400'
    if (species === 'Lynian') return 'text-pink-400'
    if (species === 'Neopteron') return 'text-lime-400'
    if (species === 'Piscine Wyvern') return 'text-blue-300'
    if (species === 'Pelagus') return 'text-amber-300'
    return 'text-gray-400'
  }
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-6">
    <div class="flex items-start justify-between gap-4">
      <div>
        <h1 class="text-2xl font-bold text-gray-100">Monsters</h1>
        <p class="text-sm text-gray-500 mt-1">
          {#if game}
            {game.shortName} · {filteredMonsters.length} / {monsters.length} monster{monsters.length ===
            1
              ? ''
              : 's'} · order: ISO mixed (filtered preserves order)
          {:else}
            Select a game first
          {/if}
        </p>
      </div>
      {#if monsters.length > 0}
        <div class="flex rounded-full border border-[var(--theme-border)] overflow-hidden shrink-0">
          <button
            onclick={() => (sizeFilter = 'large')}
            class="px-3 py-1.5 text-xs font-medium {sizeFilter === 'large'
              ? 'bg-[var(--theme-primary)] text-white'
              : 'bg-[var(--theme-bg-surface)] text-gray-400 hover:text-gray-200'}">Large</button
          >
          <button
            onclick={() => (sizeFilter = 'all')}
            class="px-3 py-1.5 text-xs font-medium {sizeFilter === 'all'
              ? 'bg-[var(--theme-primary)] text-white'
              : 'bg-[var(--theme-bg-surface)] text-gray-400 hover:text-gray-200'}">All</button
          >
          <button
            onclick={() => (sizeFilter = 'small')}
            class="px-3 py-1.5 text-xs font-medium {sizeFilter === 'small'
              ? 'bg-[var(--theme-primary)] text-white'
              : 'bg-[var(--theme-bg-surface)] text-gray-400 hover:text-gray-200'}">Small</button
          >
        </div>
      {/if}
    </div>
  </div>

  {#if loading}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Loading monsters...</p>
    </div>
  {:else if error}
    <div class="bg-red-950/30 border border-red-900 rounded-lg p-8 text-center">
      <p class="text-red-400">Failed to load monsters</p>
      <p class="text-gray-500 text-sm mt-2">{error}</p>
    </div>
  {:else if monsters.length === 0}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">No monsters found for {game?.shortName ?? 'this game'}</p>
    </div>
  {:else if filteredMonsters.length === 0}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">No monsters for filter "{sizeFilter}"</p>
    </div>
  {:else}
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-3">
      {#each filteredMonsters as monster}
        <button onclick={() => open(monster.id)} class="text-left">
          <Card class="p-4 border transition-all cursor-pointer hover:scale-[1.02] themed-card">
            <div class="flex items-center gap-3">
              <ItemIcon
                iconUrl={monster.icon_url}
                iconName={monster.icon_name}
                iconColor={monster.icon_color}
                size={40}
                alt={monster.name}
              />
              <div class="min-w-0 flex-1">
                <h3
                  class="font-semibold text-gray-100 truncate group-hover:text-[var(--theme-text-accent)]"
                >
                  {monster.name}
                </h3>
                {#if monster.species}
                  <p class="text-xs {speciesColor(monster.species)} mt-0.5">{monster.species}</p>
                {/if}
              </div>
              {#if monster.size}
                <span
                  class="text-[10px] uppercase tracking-wide text-gray-500 bg-[var(--theme-bg-elevated)] px-2 py-0.5 rounded shrink-0 border border-[var(--theme-border)]"
                >
                  {monster.size}
                </span>
              {/if}
            </div>
          </Card>
        </button>
      {/each}
    </div>
  {/if}
</div>
