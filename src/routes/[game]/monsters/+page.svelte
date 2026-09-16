<script lang="ts">
  import { goto } from '$app/navigation'
  import ErrorState from '$lib/components/ui/error-state.svelte'
  import { selectedGame } from '$lib/stores/game'
  import { api, type Monster } from '$lib/api'
  import { normKey } from '$lib/utils/norm'
  import Card from '$lib/components/ui/card.svelte'
  import Badge from '$lib/components/ui/badge.svelte'
  import Skeleton from '$lib/components/ui/skeleton.svelte'
  import EmptyState from '$lib/components/ui/empty-state.svelte'
  import SearchField from '$lib/components/ui/search-field.svelte'
  import ItemIcon from '$lib/components/item-icon.svelte'
  import FavoriteButton from '$lib/components/favorite-button.svelte'
  import { favorites } from '$lib/stores/favorites'
  import { ChevronRight, Star } from '@lucide/svelte'
  import { toolbarTarget } from '$lib/stores/toolbar'
  import { toolbarPortal } from '$lib/actions/toolbar-portal'
  import { captureScrollY, restoreScrollY } from '$lib/utils/scroll-restore'
  import type { Snapshot } from './$types.js'
  import { createQuery } from '@tanstack/svelte-query'
  import { dbCache, dbRetry, dbRetryDelay, dbErrorText } from '$lib/query'

  interface MonstersSnapshot {
    searchTerm: string
    sizeFilter: 'large' | 'all' | 'small'
    scrollY: number
  }

  export const snapshot: Snapshot<MonstersSnapshot> = {
    capture: () => ({ searchTerm, sizeFilter, scrollY: captureScrollY() }),
    restore: (s) => {
      searchTerm = s.searchTerm
      sizeFilter = s.sizeFilter
      pendingScrollY = s.scrollY
    },
  }

  let pendingScrollY = $state<number | null>(null)
  $effect(() => {
    if (!loading && pendingScrollY != null) {
      const y = pendingScrollY
      pendingScrollY = null
      restoreScrollY(y)
    }
  })

  const game = $derived($selectedGame)
  const dbId = $derived(game?.dbId)

  const monstersQuery = createQuery(() => ({
    queryKey: ['monsters', dbId ?? 0],
    queryFn: () => api.getMonsters(dbId!),
    enabled: dbId != null,
    ...dbCache,
    retry: dbRetry,
    retryDelay: dbRetryDelay,
  }))

  const monsters = $derived<Monster[]>(monstersQuery.data ?? [])
  const loading = $derived(monstersQuery.isPending)
  const error = $derived(
    dbErrorText(monstersQuery.isPending, monstersQuery.failureCount, monstersQuery.error),
  )

  function open(id: number) {
    if (!game) return
    goto(`/${game.id}/monsters/${id}`)
  }

  let sizeFilter = $state<'large' | 'all' | 'small'>('large')
  let searchTerm = $state('')
  let showFavsOnly = $state(false)

  $effect(() => {
    if (game) void favorites.ensure(game.dbId)
  })

  const favKeys = $derived(new Set(game ? [...($favorites.get(game.dbId)?.keys() ?? [])] : []))

  const filteredMonsters = $derived(
    monsters
      .filter((m) => {
        const sz = (m.size ?? '').toLowerCase()
        if (sizeFilter === 'all') return true
        if (sizeFilter === 'small') return sz === 'small'
        // large includes Large + Giant (both considered large)
        return sz === 'large' || sz === 'giant'
      })
      .filter((m) => searchTerm === '' || normKey(m.name).includes(normKey(searchTerm)))
      .filter((m) => !showFavsOnly || favKeys.has(`monster:${m.id}`)),
  )

  const sizes = ['large', 'all', 'small'] as const
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-4 md:mb-6">
    <h1 class="fluid-h2 font-bold text-gray-100">Monsters</h1>
    <p class="text-sm text-gray-400 mt-1">
      {#if game}
        {game.shortName} · {filteredMonsters.length} / {monsters.length} monster{monsters.length ===
        1
          ? ''
          : 's'}
      {:else}
        Select a game first
      {/if}
    </p>
  </div>

  {#if loading}
    <div
      class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-3"
      aria-busy="true"
    >
      {#each Array(8) as _}
        <Skeleton lines={2} />
      {/each}
    </div>
  {:else if error}
    <ErrorState title="Failed to load monsters" {error} />
  {:else if monsters.length === 0}
    <EmptyState
      title="No monsters found"
      hint={game ? `No monsters seeded for ${game.shortName}.` : 'Select a game first.'}
    />
  {:else}
    <div use:toolbarPortal={$toolbarTarget} class="flex flex-col gap-2">
      <div class="flex flex-wrap gap-2 items-center">
        <SearchField
          bind:value={searchTerm}
          placeholder="Search monsters..."
          label="Search monsters"
        />
        <div
          class="flex rounded-full border border-[var(--theme-border)] overflow-hidden"
          role="group"
          aria-label="Filter by size"
        >
          {#each sizes as s}
            <button
              type="button"
              onclick={() => (sizeFilter = s)}
              aria-pressed={sizeFilter === s}
              class="px-4 min-h-[44px] sm:min-h-[36px] text-xs font-medium capitalize focus-visible:outline-none focus-visible:ring-2 {sizeFilter ===
              s
                ? 'text-[var(--theme-text-on-primary)]'
                : 'text-gray-400 hover:text-gray-200'}"
              style={sizeFilter === s
                ? 'background-color: var(--theme-primary);'
                : 'background-color: var(--theme-bg-surface);'}
            >
              {s}
            </button>
          {/each}
        </div>
        <button
          type="button"
          onclick={() => (showFavsOnly = !showFavsOnly)}
          aria-pressed={showFavsOnly}
          title="Show favorites only"
          class="inline-flex items-center gap-1.5 px-4 min-h-[44px] sm:min-h-[36px] rounded-full border text-xs font-medium focus-visible:outline-none focus-visible:ring-2 {showFavsOnly
            ? 'border-[var(--theme-accent)]/50 bg-[var(--theme-accent)]/10 text-[var(--theme-accent)]'
            : 'border-[var(--theme-border)] bg-[var(--theme-bg-surface)] text-gray-400 hover:text-gray-200'}"
        >
          <Star
            class="h-3.5 w-3.5 {showFavsOnly ? 'fill-[var(--theme-accent)]' : ''}"
            aria-hidden="true"
          />
          Favorites
        </button>
      </div>
    </div>

    {#if filteredMonsters.length === 0}
      <div class="mt-4">
        <EmptyState title="No matches" hint="Try another search term or size filter.">
          <button
            type="button"
            onclick={() => {
              searchTerm = ''
              sizeFilter = 'all'
            }}
            class="text-xs px-4 min-h-[44px] rounded-full border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] text-gray-200 hover:border-[var(--theme-border-strong)]"
          >
            Clear filters
          </button>
        </EmptyState>
      </div>
    {:else}
      <div class="mt-4 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-3">
        {#each filteredMonsters as monster, i (monster.id)}
          <div class="relative">
            <button
              onclick={() => open(monster.id)}
              aria-label="Open {monster.name}"
              class="group text-left rounded-lg min-h-[44px] w-full focus-visible:outline-none focus-visible:ring-2"
              style="--i: {i};"
            >
              <Card variant="themed" class="p-4 cursor-pointer">
                <div class="flex items-center gap-3">
                  <div
                    class="w-12 h-12 rounded-xl flex items-center justify-center shrink-0 border border-[var(--theme-border-strong)] bg-[var(--theme-bg-elevated)]"
                    aria-hidden="true"
                  >
                    <ItemIcon
                      iconUrl={monster.icon_url}
                      iconName={monster.icon_name}
                      iconColor={monster.icon_color}
                      size={36}
                      alt=""
                    />
                  </div>
                  <div class="min-w-0 flex-1">
                    <h3 class="font-semibold text-gray-100 truncate">
                      {monster.name}
                    </h3>
                    {#if monster.species}
                      <p class="text-xs text-[var(--theme-text-muted)] mt-0.5 truncate">
                        {monster.species}
                      </p>
                    {/if}
                  </div>
                  {#if monster.size}
                    <Badge tone={monster.species === 'Elder Dragon' ? 'accent' : 'neutral'}>
                      {monster.size}
                    </Badge>
                  {/if}
                  <ChevronRight
                    class="h-4 w-4 shrink-0 text-[var(--theme-text-muted)] opacity-0 transition-opacity group-hover:opacity-100 group-focus-visible:opacity-100"
                    aria-hidden="true"
                  />
                </div>
              </Card>
            </button>
            <div class="absolute top-2 right-2">
              <FavoriteButton kind="monster" id={monster.id} name={monster.name} size="sm" />
            </div>
          </div>
        {/each}
      </div>
    {/if}
  {/if}
</div>
