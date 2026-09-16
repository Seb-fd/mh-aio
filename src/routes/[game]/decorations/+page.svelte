<script lang="ts">
  import { goto } from '$app/navigation'
  import ErrorState from '$lib/components/ui/error-state.svelte'
  import { selectedGame } from '$lib/stores/game'
  import { api, type Decoration } from '$lib/api'
  import { normKey } from '$lib/utils/norm'
  import Card from '$lib/components/ui/card.svelte'
  import Badge from '$lib/components/ui/badge.svelte'
  import FilterChip from '$lib/components/ui/filter-chip.svelte'
  import Skeleton from '$lib/components/ui/skeleton.svelte'
  import EmptyState from '$lib/components/ui/empty-state.svelte'
  import SearchField from '$lib/components/ui/search-field.svelte'
  import ItemIcon from '$lib/components/item-icon.svelte'
  import FavoriteButton from '$lib/components/favorite-button.svelte'
  import { favorites } from '$lib/stores/favorites'
  import { Star } from '@lucide/svelte'
  import { toolbarTarget } from '$lib/stores/toolbar'
  import { toolbarPortal } from '$lib/actions/toolbar-portal'
  import { captureScrollY, restoreScrollY } from '$lib/utils/scroll-restore'
  import { createQuery } from '@tanstack/svelte-query'
  import { dbCache, dbRetry, dbRetryDelay, dbErrorText } from '$lib/query'
  import type { Snapshot } from './$types.js'

  interface DecorationsSnapshot {
    searchTerm: string
    skillFilter: string
    slotFilter: string
    scrollY: number
  }

  export const snapshot: Snapshot<DecorationsSnapshot> = {
    capture: () => ({ searchTerm, skillFilter, slotFilter, scrollY: captureScrollY() }),
    restore: (s) => {
      searchTerm = s.searchTerm
      skillFilter = s.skillFilter
      slotFilter = s.slotFilter
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

  const decorationsQuery = createQuery(() => ({
    queryKey: ['decorations', dbId ?? 0],
    queryFn: () => api.getDecorations(dbId!),
    enabled: dbId != null,
    ...dbCache,
    retry: dbRetry,
    retryDelay: dbRetryDelay,
  }))

  const decorations = $derived<Decoration[]>(decorationsQuery.data ?? [])
  const loading = $derived(decorationsQuery.isPending)
  const error = $derived(
    dbErrorText(decorationsQuery.isPending, decorationsQuery.failureCount, decorationsQuery.error),
  )
  let searchTerm = $state('')
  let slotFilter = $state<string>('all')
  let skillFilter = $state<string>('all')
  let showFavsOnly = $state(false)

  $effect(() => {
    if (game) void favorites.ensure(game.dbId)
  })

  const favKeys = $derived(new Set(game ? [...($favorites.get(game.dbId)?.keys() ?? [])] : []))

  const skills = $derived([
    'all',
    ...Array.from(
      new Set(
        decorations.flatMap((d) =>
          [d.skill_name, d.secondary_skill_name].filter((x): x is string => !!x),
        ),
      ),
    ).sort(),
  ])
  const slots = ['all', '1', '2', '3']

  const filtered = $derived(
    decorations
      .filter((d) => slotFilter === 'all' || String(d.slot_size) === slotFilter)
      .filter(
        (d) =>
          skillFilter === 'all' ||
          d.skill_name === skillFilter ||
          d.secondary_skill_name === skillFilter,
      )
      .filter((d) => searchTerm === '' || normKey(d.name).includes(normKey(searchTerm)))
      .filter((d) => !showFavsOnly || favKeys.has(`decoration:${d.id}`)),
  )

  function open(id: number) {
    if (!game) return
    goto(`/${game.id}/decorations/${id}`)
  }
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-4 md:mb-6">
    <h1 class="fluid-h2 font-bold text-gray-100">Decorations</h1>
    <p class="text-sm text-gray-400 mt-1">
      {#if game}
        {game.shortName} · {filtered.length} / {decorations.length} jewels · Crafted at Smith
      {:else}
        Select a game first
      {/if}
    </p>
  </div>

  {#if loading}
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2" aria-busy="true">
      {#each Array(6) as _}
        <Skeleton lines={2} />
      {/each}
    </div>
  {:else if error}
    <ErrorState title="Failed to load decorations" {error} />
  {:else if decorations.length === 0}
    <EmptyState
      title="No decorations found"
      hint={game ? `No decorations seeded for ${game.shortName}.` : 'Select a game first.'}
    />
  {:else}
    <div use:toolbarPortal={$toolbarTarget} class="flex flex-col gap-2">
      <div class="flex flex-wrap gap-2 items-center">
        <SearchField
          bind:value={searchTerm}
          placeholder="Search jewels..."
          label="Search decorations"
          class="sm:w-48"
        />
        <select
          bind:value={skillFilter}
          aria-label="Filter by skill"
          class="px-3 rounded-full bg-[var(--theme-bg-surface)] border border-[var(--theme-border)] text-gray-300 focus:outline-none focus:border-[var(--theme-border-strong)] min-h-[44px] sm:min-h-[36px] text-base sm:text-xs"
        >
          {#each skills as sk}
            <option value={sk}>{sk === 'all' ? 'All Skills' : sk}</option>
          {/each}
        </select>
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
      <div
        class="flex gap-2 overflow-x-auto pb-1 -mb-1"
        role="group"
        aria-label="Filter by slot size"
      >
        {#each slots as s}
          <FilterChip
            active={slotFilter === s}
            onclick={() => (slotFilter = s)}
            label={s === 'all' ? 'All slots' : `Slot ${s}`}
          >
            {s === 'all' ? 'All Slots' : `Slot ${s}`}
          </FilterChip>
        {/each}
      </div>
    </div>

    {#if filtered.length === 0}
      <div class="mt-4">
        <EmptyState title="No jewels match filters." hint="Try widening your filters.">
          <button
            type="button"
            onclick={() => {
              searchTerm = ''
              slotFilter = 'all'
              skillFilter = 'all'
            }}
            class="text-xs px-4 min-h-[44px] rounded-full border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] text-gray-200 hover:border-[var(--theme-border-strong)]"
          >
            Clear filters
          </button>
        </EmptyState>
      </div>
    {:else}
      <div class="mt-4 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2">
        {#each filtered as deco (deco.id)}
          <div class="relative">
            <button
              onclick={() => open(deco.id)}
              aria-label="Open {deco.name}"
              class="text-left rounded-lg min-h-[44px] w-full focus-visible:outline-none focus-visible:ring-2"
            >
              <Card variant="themed" class="p-3 cursor-pointer h-full">
                <div class="flex items-start gap-2">
                  <ItemIcon
                    iconUrl={deco.icon_url}
                    iconName={deco.icon_name}
                    iconColor={deco.icon_color}
                    size={32}
                    alt=""
                  />
                  <div class="min-w-0 flex-1">
                    <p class="font-medium text-sm text-gray-100 truncate">{deco.name}</p>
                    <p class="text-[11px] text-gray-400 mt-0.5 truncate">
                      {#if deco.skill_name}
                        <span
                          class={(deco.skill_points ?? 0) >= 0
                            ? 'text-emerald-300'
                            : 'text-red-300'}
                          >{deco.skill_name}
                          {(deco.skill_points ?? 0) > 0 ? '+' : ''}{deco.skill_points}</span
                        >
                      {/if}
                      {#if deco.secondary_skill_name}
                        <span class="text-gray-600"> · </span>
                        <span
                          class={(deco.secondary_points ?? 0) >= 0
                            ? 'text-emerald-300'
                            : 'text-red-300'}
                          >{deco.secondary_skill_name}
                          {(deco.secondary_points ?? 0) > 0 ? '+' : ''}{deco.secondary_points}</span
                        >
                      {/if}
                    </p>
                  </div>
                </div>
                <div class="flex items-center justify-between mt-2">
                  <Badge tone="neutral">Slot {deco.slot_size}</Badge>
                  <span class="text-xs font-medium tabular-nums" style="color: var(--theme-accent);"
                    >{deco.price ?? 0}z</span
                  >
                </div>
              </Card>
            </button>
            <div class="absolute top-2 right-2">
              <FavoriteButton kind="decoration" id={deco.id} name={deco.name} size="sm" />
            </div>
          </div>
        {/each}
      </div>
    {/if}
  {/if}
</div>
