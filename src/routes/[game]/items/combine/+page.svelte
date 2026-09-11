<script lang="ts">
  import { goto } from '$app/navigation'
  import ErrorState from '$lib/components/ui/error-state.svelte'
  import { api, type CombineView } from '$lib/api'
  import { selectedGame } from '$lib/stores/game'
  import { normKey } from '$lib/utils/norm'
  import FilterChip from '$lib/components/ui/filter-chip.svelte'
  import Button from '$lib/components/ui/button.svelte'
  import Skeleton from '$lib/components/ui/skeleton.svelte'
  import EmptyState from '$lib/components/ui/empty-state.svelte'
  import SearchField from '$lib/components/ui/search-field.svelte'
  import { Lightbulb } from '@lucide/svelte'
  import { toolbarTarget } from '$lib/stores/toolbar'
  import { toolbarPortal } from '$lib/actions/toolbar-portal'
  import { captureScrollY, restoreScrollY } from '$lib/utils/scroll-restore'
  import { createQuery } from '@tanstack/svelte-query'
  import { dbCache, dbRetry, dbRetryDelay, dbErrorText } from '$lib/query'
  import type { Snapshot } from './$types.js'

  interface CombineSnapshot {
    searchTerm: string
    filterType: string
    visibleCount: number
    scrollY: number
  }

  export const snapshot: Snapshot<CombineSnapshot> = {
    capture: () => ({ searchTerm, filterType, visibleCount, scrollY: captureScrollY() }),
    restore: (s) => {
      searchTerm = s.searchTerm
      filterType = s.filterType
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

  const PAGE_SIZE = 100

  const game = $derived($selectedGame)
  const dbId = $derived(game?.dbId)

  const combinesQuery = createQuery(() => ({
    queryKey: ['combinations', dbId ?? 0],
    queryFn: () => api.getCombinations(dbId!),
    enabled: dbId != null,
    ...dbCache,
    retry: dbRetry,
    retryDelay: dbRetryDelay,
  }))

  const combines = $derived<CombineView[]>(combinesQuery.data ?? [])
  const loading = $derived(combinesQuery.isPending)
  const error = $derived(
    dbErrorText(combinesQuery.isPending, combinesQuery.failureCount, combinesQuery.error),
  )
  let filterType = $state<string>('all') // all | normal | alchemy | treasure
  let searchTerm = $state('')
  let visibleCount = $state(PAGE_SIZE)
  // Skipped once after a snapshot restore (back navigation keeps its depth).
  let skipReset = $state(false)

  // Reset pagination on game change or new criteria (data itself comes from cache)
  $effect(() => {
    void dbId
    void filterType
    void searchTerm
    if (skipReset) {
      skipReset = false
      return
    }
    visibleCount = PAGE_SIZE
  })

  const filtered = $derived.by(() => {
    let arr = combines
    if (filterType !== 'all') {
      arr = arr.filter((c) => c.combine_type === filterType)
    }
    if (searchTerm !== '') {
      const term = normKey(searchTerm)
      arr = arr.filter(
        (c) =>
          normKey(c.result_name).includes(term) ||
          c.components.some((comp) => normKey(comp.component_name).includes(term)),
      )
    }
    return arr
  })

  const visible = $derived(filtered.slice(0, visibleCount))

  const counts = $derived.by(() => {
    const c = { all: combines.length, normal: 0, alchemy: 0, treasure: 0 }
    for (const r of combines) {
      if (r.combine_type === 'normal') c.normal++
      else if (r.combine_type === 'alchemy') c.alchemy++
      else if (r.combine_type === 'treasure') c.treasure++
    }
    return c
  })

  function openResult(id: number) {
    if (!game) return
    goto(`/${game.id}/items/${id}`)
  }
  function openComponent(id: number) {
    if (!game) return
    goto(`/${game.id}/items/${id}`)
  }

  const typeBadge: Record<string, { label: string; cls: string }> = {
    normal: { label: 'Normal', cls: 'bg-sky-900/30 text-sky-300 border-sky-800' },
    alchemy: { label: 'Alchemy', cls: 'bg-amber-900/30 text-amber-300 border-amber-800' },
    treasure: { label: 'Treasure', cls: 'bg-purple-900/30 text-purple-300 border-purple-800' },
  }
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-4 md:mb-6">
    <div class="flex items-center gap-2 flex-wrap">
      <button
        onclick={() => {
          if (game) goto(`/${game.id}/items`)
        }}
        aria-label="Back to items"
        class="inline-flex items-center rounded-md text-xs px-3 min-h-[44px] border border-[var(--theme-border)] text-gray-300 hover:text-gray-100 hover:border-[var(--theme-border-strong)] transition-colors motion-safe:transition-colors motion-reduce:transition-none focus-visible:outline-none focus-visible:ring-2"
      >
        <span aria-hidden="true">←&nbsp;</span>Items
      </button>
      <h1 class="fluid-h2 font-bold text-gray-100">Combinations</h1>
      <span
        class="text-xs px-2 py-1 rounded-full border bg-[var(--theme-bg-surface)] text-gray-400 border-[var(--theme-border)] whitespace-nowrap"
        >{combines.length} recipes</span
      >
    </div>
    <p class="text-sm text-gray-400 mt-1">
      {#if game}
        {game.shortName} · Book order (game) · Normal + Alchemy
      {:else}
        Select a game first
      {/if}
    </p>
    {#if filtered.length > 0}
      <p class="text-[11px] text-gray-400 mt-1 inline-flex items-start gap-1.5">
        <Lightbulb class="h-3.5 w-3.5 shrink-0 mt-px" aria-hidden="true" />
        All recipes sorted as in the in-game Combination List (Book of Combos 1-5 + Alchemy Guide)
      </p>
    {/if}
  </div>

  {#if loading}
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-3" aria-busy="true">
      {#each Array(6) as _}
        <Skeleton lines={3} />
      {/each}
    </div>
  {:else if error}
    <ErrorState title="Failed to load combinations" {error} />
  {:else}
    <div use:toolbarPortal={$toolbarTarget} class="flex flex-col gap-2">
      <SearchField
        bind:value={searchTerm}
        placeholder="Search result or ingredient..."
        label="Search combinations"
        class="sm:w-64"
      />
      <div
        class="flex gap-2 overflow-x-auto pb-1 -mb-1"
        role="group"
        aria-label="Filter by combination type"
      >
        {#each [{ key: 'all', label: `All (${counts.all})` }, { key: 'normal', label: `Normal (${counts.normal})` }, { key: 'alchemy', label: `Alchemy (${counts.alchemy})` }, { key: 'treasure', label: `Treasure (${counts.treasure})` }] as f}
          <FilterChip
            active={filterType === f.key}
            onclick={() => (filterType = f.key)}
            label="Show {f.label} recipes"
          >
            {f.label}
          </FilterChip>
        {/each}
      </div>
    </div>

    {#if filtered.length === 0}
      <div class="mt-4">
        <EmptyState
          title="No combinations match your filter."
          hint="Try another search term or type."
        >
          <button
            type="button"
            onclick={() => {
              searchTerm = ''
              filterType = 'all'
            }}
            class="text-xs px-4 min-h-[44px] rounded-full border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] text-gray-200 hover:border-[var(--theme-border-strong)]"
          >
            Clear filters
          </button>
        </EmptyState>
      </div>
    {:else}
      <div class="mt-4 grid grid-cols-1 lg:grid-cols-2 gap-3">
        {#each visible as rec, i (rec.combine_type + '|' + rec.result_item_id + '|' + i)}
          {@const badge = typeBadge[rec.combine_type] ?? {
            label: rec.combine_type,
            cls: 'bg-gray-800 text-gray-300 border-gray-700',
          }}
          <div class="rounded-lg border themed-card p-4 flex flex-col gap-2">
            <div class="flex items-center gap-2 flex-wrap">
              <button
                onclick={() => openResult(rec.result_item_id)}
                aria-label="Open {rec.result_name}"
                class="text-sm font-semibold text-gray-100 hover:text-[var(--theme-accent)] transition-colors motion-safe:transition-colors motion-reduce:transition-none text-left rounded min-h-[36px] focus-visible:outline-none focus-visible:ring-2"
              >
                {rec.result_name}
              </button>
              <span class="text-[10px] px-2 py-0.5 rounded-full border font-semibold {badge.cls}"
                >{badge.label}</span
              >
              {#if rec.chance != null}
                <span
                  class="text-[10px] px-1.5 py-0.5 rounded border bg-[var(--theme-bg-elevated)] text-gray-400 border-[var(--theme-border)] tabular-nums"
                  >{rec.chance}% success</span
                >
              {/if}
              <span class="text-[10px] text-gray-600 ml-auto tabular-nums"
                >#{rec.result_item_id}</span
              >
            </div>
            <div class="flex flex-wrap items-center gap-1.5">
              {#each rec.components as comp, i}
                {#if i > 0}
                  <span class="text-gray-600 text-sm" aria-hidden="true">+</span>
                {/if}
                <button
                  onclick={() => openComponent(comp.component_item_id)}
                  class="px-2.5 min-h-[40px] py-1 rounded-md bg-[var(--theme-bg-elevated)] border border-[var(--theme-border)] flex items-center gap-1.5 hover:border-[var(--theme-border-strong)] hover:bg-[var(--theme-bg-surface)] transition-colors motion-safe:transition-colors motion-reduce:transition-none focus-visible:outline-none focus-visible:ring-2"
                  title="Go to {comp.component_name}"
                >
                  <span class="text-xs text-gray-200">{comp.component_name}</span>
                  <span
                    class="text-[10px] font-semibold tabular-nums"
                    style="color: var(--theme-accent);">x{comp.quantity}</span
                  >
                </button>
              {/each}
              <span class="text-gray-600 text-sm" aria-hidden="true">=</span>
              <button
                onclick={() => openResult(rec.result_item_id)}
                aria-label="Open result {rec.result_name}"
                class="px-2.5 min-h-[40px] py-1 rounded-md flex items-center gap-1.5 hover:opacity-90 transition-opacity focus-visible:outline-none focus-visible:ring-2"
                style="background-color: color-mix(in oklab, var(--theme-accent) 15%, var(--theme-bg-elevated)); border: 1px solid color-mix(in oklab, var(--theme-accent) 40%, transparent);"
              >
                <span class="text-xs text-gray-100">{rec.result_name}</span>
                <span
                  class="text-[10px] font-semibold tabular-nums"
                  style="color: var(--theme-accent);"
                  >x{rec.components[0]?.result_quantity ?? 1}</span
                >
              </button>
            </div>
            {#if rec.combine_type === 'alchemy'}
              <p class="text-[10px] text-amber-400/60">
                ※ Requires Alchemy Guide (progressively unlocks with Books 1-5)
              </p>
            {/if}
          </div>
        {/each}
      </div>
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
            Show more
          </Button>
        </div>
      {/if}
    {/if}
  {/if}
</div>
