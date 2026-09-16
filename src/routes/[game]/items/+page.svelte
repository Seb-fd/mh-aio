<script lang="ts">
  import { goto } from '$app/navigation'
  import ErrorState from '$lib/components/ui/error-state.svelte'
  import { selectedGame } from '$lib/stores/game'
  import { api, type Item } from '$lib/api'
  import { normKey } from '$lib/utils/norm'
  import { fallbackLabel } from '$lib/utils/mh'
  import Card from '$lib/components/ui/card.svelte'
  import Button from '$lib/components/ui/button.svelte'
  import FilterChip from '$lib/components/ui/filter-chip.svelte'
  import Skeleton from '$lib/components/ui/skeleton.svelte'
  import EmptyState from '$lib/components/ui/empty-state.svelte'
  import SearchField from '$lib/components/ui/search-field.svelte'
  import ItemIcon from '$lib/components/item-icon.svelte'
  import FavoriteButton from '$lib/components/favorite-button.svelte'
  import { favorites } from '$lib/stores/favorites'
  import { browser } from '$app/environment'
  import { toolbarTarget } from '$lib/stores/toolbar'
  import { toolbarPortal } from '$lib/actions/toolbar-portal'
  import { captureScrollY, restoreScrollY } from '$lib/utils/scroll-restore'
  import { createQuery } from '@tanstack/svelte-query'
  import { dbCache, dbRetry, dbRetryDelay, dbErrorText } from '$lib/query'
  import type { Snapshot } from './$types.js'

  interface ItemsSnapshot {
    searchTerm: string
    categoryFilter: string
    sortBy: string
    view: 'cards' | 'table'
    visibleCount: number
    scrollY: number
  }

  export const snapshot: Snapshot<ItemsSnapshot> = {
    capture: () => ({
      searchTerm,
      categoryFilter,
      sortBy,
      view,
      visibleCount,
      scrollY: captureScrollY(),
    }),
    restore: (s) => {
      searchTerm = s.searchTerm
      categoryFilter = s.categoryFilter
      sortBy = s.sortBy
      view = s.view
      visibleCount = s.visibleCount
      skipReset = true
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
  import { LayoutGrid, Table2, FlaskConical, Star } from '@lucide/svelte'

  const game = $derived($selectedGame)
  const dbId = $derived(game?.dbId)

  const PAGE_SIZE = 100

  const itemsQuery = createQuery(() => ({
    queryKey: ['items', dbId ?? 0],
    queryFn: () => api.getItems(dbId!),
    enabled: dbId != null,
    ...dbCache,
    retry: dbRetry,
    retryDelay: dbRetryDelay,
  }))

  const items = $derived<Item[]>(itemsQuery.data ?? [])
  const loading = $derived(itemsQuery.isPending)
  const error = $derived(
    dbErrorText(itemsQuery.isPending, itemsQuery.failureCount, itemsQuery.error),
  )
  let categoryFilter = $state<string>('all')
  let searchTerm = $state('')
  let showFavsOnly = $state(false)

  $effect(() => {
    if (game) void favorites.ensure(game.dbId)
  })

  const favKeys = $derived(new Set(game ? [...($favorites.get(game.dbId)?.keys() ?? [])] : []))
  let sortBy = $state<string>('chest') // chest = game box (id) faithful to ISO DATA.BIN file 15
  // View preference: cards by default, last selection persisted (global, not per-game).
  const ITEMS_VIEW_KEY = 'mh-aio:items-view'
  function initialView(): 'cards' | 'table' {
    if (!browser) return 'cards'
    const stored = localStorage.getItem(ITEMS_VIEW_KEY)
    return stored === 'table' || stored === 'cards' ? stored : 'cards'
  }
  let view = $state<'cards' | 'table'>(initialView())
  $effect(() => {
    if (browser) localStorage.setItem(ITEMS_VIEW_KEY, view)
  })
  let visibleCount = $state(PAGE_SIZE)
  // Skipped once after a snapshot restore (back navigation keeps its depth).
  let skipReset = $state(false)

  // Reset pagination on game change or new criteria (data itself comes from cache)
  $effect(() => {
    void dbId
    void categoryFilter
    void searchTerm
    void sortBy
    void showFavsOnly
    if (skipReset) {
      skipReset = false
      return
    }
    visibleCount = PAGE_SIZE
  })

  const categories = $derived([
    'all',
    ...Array.from(new Set(items.map((i) => i.category).filter((c): c is string => !!c))),
  ])
  const filtered = $derived.by(() => {
    let arr = items
      .filter((i) => categoryFilter === 'all' || i.category === categoryFilter)
      .filter((i) => searchTerm === '' || normKey(i.name).includes(normKey(searchTerm)))
      .filter((i) => !showFavsOnly || favKeys.has(`item:${i.id}`))
    // Sorting: chest is already id order from DB, keep stable; other sorts client-side
    if (sortBy === 'name') arr = [...arr].sort((a, b) => a.name.localeCompare(b.name))
    else if (sortBy === 'rarity') arr = [...arr].sort((a, b) => (b.rarity ?? 0) - (a.rarity ?? 0))
    else if (sortBy === 'price')
      arr = [...arr].sort((a, b) => (b.sell_price ?? 0) - (a.sell_price ?? 0))
    else if (sortBy === 'category')
      arr = [...arr].sort(
        (a, b) =>
          (a.category ?? '').localeCompare(b.category ?? '') || a.name.localeCompare(b.name),
      )
    // chest (id) is default, no sort needed - already ORDER BY id from queries.rs:1023
    return arr
  })

  function open(id: number) {
    if (!game) return
    goto(`/${game.id}/items/${id}`)
  }

  const visible = $derived(filtered.slice(0, visibleCount))

  const _categoryColor: Record<string, string> = {
    Consumable: 'bg-emerald-900/40 text-emerald-300',
    Material: 'bg-purple-900/40 text-purple-300',
    Ammo: 'bg-orange-900/40 text-orange-300',
  }
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-4 md:mb-6">
    <div class="flex items-center gap-3 flex-wrap">
      <h1 class="fluid-h2 font-bold text-gray-100">Items</h1>
      <button
        onclick={() => {
          if (game) goto(`/${game.id}/items/combine`)
        }}
        class="inline-flex items-center gap-1.5 ml-auto sm:ml-2 text-xs px-3 rounded-full border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] text-gray-300 hover:border-[var(--theme-accent)] hover:text-[var(--theme-accent)] transition-colors motion-safe:transition-colors motion-reduce:transition-none min-h-[44px] sm:min-h-[36px]"
      >
        <FlaskConical class="h-3.5 w-3.5" aria-hidden="true" />
        Combinations
        <span aria-hidden="true">→</span>
      </button>
    </div>
    <p class="text-sm text-gray-400 mt-1">
      {#if game}
        {game.shortName} · {filtered.length} / {items.length} items · Materials, consumables and locations
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
    <ErrorState title="Failed to load items" {error} />
  {:else if items.length === 0}
    <EmptyState
      title="No items found"
      hint={game
        ? `No items seeded for ${game.shortName ?? 'this game'} yet.`
        : 'Select a game first.'}
    />
  {:else}
    <div use:toolbarPortal={$toolbarTarget} class="flex flex-col gap-2">
      <div class="flex flex-wrap gap-2 items-center">
        <SearchField bind:value={searchTerm} placeholder="Search items..." label="Search items" />
        <select
          bind:value={sortBy}
          aria-label="Sort items"
          class="px-3 rounded-full bg-[var(--theme-bg-surface)] border border-[var(--theme-border)] text-gray-300 focus:outline-none min-h-[44px] sm:min-h-[36px] text-base sm:text-xs"
        >
          <option value="chest">Chest (Game Order)</option>
          <option value="name">Name A-Z</option>
          <option value="rarity">Rarity ↓</option>
          <option value="price">Sell Price ↓</option>
          <option value="category">Category</option>
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
        <div
          class="hidden md:inline-flex rounded-full border border-[var(--theme-border)] overflow-hidden ml-auto"
          role="group"
          aria-label="Change view"
        >
          <button
            type="button"
            onclick={() => (view = 'cards')}
            aria-pressed={view === 'cards'}
            class="inline-flex items-center gap-1.5 px-3 min-h-[36px] text-xs font-medium {view ===
            'cards'
              ? 'text-[var(--theme-text-on-primary)]'
              : 'text-gray-400 hover:text-gray-200'}"
            style={view === 'cards'
              ? 'background-color: var(--theme-primary);'
              : 'background-color: var(--theme-bg-surface);'}
          >
            <LayoutGrid class="h-3.5 w-3.5" aria-hidden="true" /> Cards
          </button>
          <button
            type="button"
            onclick={() => (view = 'table')}
            aria-pressed={view === 'table'}
            class="inline-flex items-center gap-1.5 px-3 min-h-[36px] text-xs font-medium {view ===
            'table'
              ? 'text-[var(--theme-text-on-primary)]'
              : 'text-gray-400 hover:text-gray-200'}"
            style={view === 'table'
              ? 'background-color: var(--theme-primary);'
              : 'background-color: var(--theme-bg-surface);'}
          >
            <Table2 class="h-3.5 w-3.5" aria-hidden="true" /> Table
          </button>
        </div>
      </div>
      <div
        class="flex gap-2 overflow-x-auto pb-1 -mb-1"
        role="group"
        aria-label="Filter by category"
      >
        {#each categories as cat}
          <FilterChip
            active={categoryFilter === cat}
            onclick={() => (categoryFilter = cat)}
            label={cat === 'all' ? `All (${items.length})` : cat}
          >
            {cat === 'all' ? `All (${items.length})` : cat}
          </FilterChip>
        {/each}
      </div>
    </div>

    {#if filtered.length === 0}
      <div class="mt-4">
        <EmptyState title="No matches" hint="Try another search term or category.">
          <button
            type="button"
            onclick={() => {
              searchTerm = ''
              categoryFilter = 'all'
            }}
            class="text-xs px-4 min-h-[44px] rounded-full border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] text-gray-200 hover:border-[var(--theme-border-strong)]"
          >
            Clear filters
          </button>
        </EmptyState>
      </div>
    {:else if view === 'table'}
      <div class="mt-4 table-responsive">
        <table class="text-sm">
          <caption class="sr-only">
            Items — showing {visible.length} of {filtered.length}
          </caption>
          <thead>
            <tr class="text-left text-xs uppercase tracking-wide text-gray-400">
              <th scope="col" class="px-3 py-3 font-medium">Item</th>
              <th scope="col" class="px-3 py-3 font-medium">Category</th>
              <th scope="col" class="px-3 py-3 font-medium text-right">Rarity</th>
              <th scope="col" class="px-3 py-3 font-medium text-right">Price</th>
            </tr>
          </thead>
          <tbody>
            {#each visible as item (item.id)}
              <tr
                class="border-t border-[var(--theme-border)] hover:bg-[var(--theme-bg-elevated)] transition-colors motion-safe:transition-colors motion-reduce:transition-none"
              >
                <td class="px-3 py-2">
                  <button
                    type="button"
                    onclick={() => open(item.id)}
                    class="flex items-center gap-3 text-left w-full rounded-md min-h-[44px] focus-visible:outline-none focus-visible:ring-2"
                  >
                    <ItemIcon
                      iconUrl={item.icon_url}
                      iconName={item.icon_name}
                      iconColor={item.icon_color}
                      size={28}
                      alt=""
                    />
                    <span class="font-medium text-gray-100 truncate">{item.name}</span>
                  </button>
                </td>
                <td class="px-3 py-2 text-xs text-gray-400 whitespace-nowrap">
                  {fallbackLabel(item.category)}{item.subcategory ? ` • ${item.subcategory}` : ''}
                </td>
                <td class="px-3 py-2 text-xs text-gray-300 text-right tabular-nums">
                  R{item.rarity ?? 1}
                </td>
                <td
                  class="px-3 py-2 text-xs text-right tabular-nums"
                  style="color: var(--theme-accent);"
                >
                  {item.sell_price ?? '—'}{item.sell_price != null ? 'z' : ''}
                </td>
                <td class="px-2 py-2 w-12">
                  <FavoriteButton kind="item" id={item.id} name={item.name} size="sm" />
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {:else}
      <div class="mt-4 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2">
        {#each visible as item (item.id)}
          <div class="relative">
            <button onclick={() => open(item.id)} class="text-left rounded-lg min-h-[44px] w-full">
              <Card variant="themed" class="p-3 cursor-pointer">
                <div class="flex items-center gap-3">
                  <ItemIcon
                    iconUrl={item.icon_url}
                    iconName={item.icon_name}
                    iconColor={item.icon_color}
                    size={36}
                    alt=""
                  />
                  <div class="min-w-0 flex-1">
                    <p class="font-medium text-sm text-gray-100 truncate">{item.name}</p>
                    <p class="text-[10px] uppercase tracking-wide text-gray-400 mt-0.5">
                      {fallbackLabel(item.category)}{item.subcategory
                        ? ` • ${item.subcategory}`
                        : ''} · R{item.rarity ?? 1}
                      {#if item.carry_limit}
                        · x{item.carry_limit}
                      {/if}
                    </p>
                  </div>
                  {#if item.sell_price !== null && item.sell_price !== undefined}
                    <span
                      class="text-xs font-medium tabular-nums shrink-0"
                      style="color: var(--theme-accent);">{item.sell_price}z</span
                    >
                  {/if}
                </div>
              </Card>
            </button>
            <div class="absolute top-2 right-2">
              <FavoriteButton kind="item" id={item.id} name={item.name} size="sm" />
            </div>
          </div>
        {/each}
      </div>
    {/if}

    {#if filtered.length > visible.length}
      <div class="mt-6 flex flex-col items-center gap-2">
        <p class="text-xs text-gray-400" role="status">
          Showing {visible.length} of {filtered.length}
        </p>
        <Button
          variant="themedPrimary"
          size="lg"
          class="rounded-full px-6"
          onclick={() => (visibleCount += PAGE_SIZE)}
        >
          Show more ({Math.min(PAGE_SIZE, filtered.length - visible.length)} more)
        </Button>
      </div>
    {:else if filtered.length > 0}
      <p class="mt-6 text-center text-xs text-gray-600" role="status">
        Showing all {filtered.length} items
      </p>
    {/if}
  {/if}
</div>
