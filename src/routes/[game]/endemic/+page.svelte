<script lang="ts">
  import { selectedGame } from '$lib/stores/game'
  import { api, type EndemicLife } from '$lib/api'
  import Card from '$lib/components/ui/card.svelte'
  import Badge from '$lib/components/ui/badge.svelte'
  import FilterChip from '$lib/components/ui/filter-chip.svelte'
  import Skeleton from '$lib/components/ui/skeleton.svelte'
  import EmptyState from '$lib/components/ui/empty-state.svelte'
  import SearchField from '$lib/components/ui/search-field.svelte'
  import { normKey } from '$lib/utils/norm'
  import { createQuery } from '@tanstack/svelte-query'
  import { dbCache, dbRetry, dbRetryDelay } from '$lib/query'

  const game = $derived($selectedGame)
  const dbId = $derived(game?.dbId)
  const isWilds = $derived(game?.id === 'mhwilds')

  let searchTerm = $state('')
  let kindFilter = $state<string>('all')

  const endemicQuery = createQuery(() => ({
    queryKey: ['endemic-life', dbId ?? 0],
    queryFn: () => api.getEndemicLife(dbId!),
    enabled: dbId != null && isWilds,
    ...dbCache,
    retry: dbRetry,
    retryDelay: dbRetryDelay,
  }))
  const data = $derived<EndemicLife[]>(endemicQuery.data ?? [])
  const loading = $derived(endemicQuery.isPending)

  const kinds = $derived(['all', ...Array.from(new Set(data.map((d) => d.kind ?? 'Other'))).sort()])
  const q = $derived(normKey(searchTerm.trim()))
  const filtered = $derived(
    data.filter((d) => {
      if (kindFilter !== 'all' && (d.kind ?? 'Other') !== kindFilter) return false
      if (q && !normKey(d.name).includes(q) && !normKey(d.description ?? '').includes(q))
        return false
      return true
    }),
  )
</script>

<div class="mx-auto w-full max-w-6xl px-4 py-6">
  <div class="mb-4">
    <h1 class="text-2xl font-bold" style="color: var(--theme-text-accent);">Endemic Life</h1>
    <p class="text-sm text-[var(--theme-text-muted)]">
      Non-monster creatures found in MH Wilds — captured, fished, or collected.
    </p>
  </div>

  <div class="mb-4 flex flex-wrap items-center gap-2">
    {#each kinds as k}
      <FilterChip active={kindFilter === k} onclick={() => (kindFilter = k)}>
        {k === 'all' ? 'All' : k}
      </FilterChip>
    {/each}
    <div class="min-w-48 flex-1">
      <SearchField bind:value={searchTerm} placeholder="Filter by name or description…" />
    </div>
  </div>

  {#if !isWilds}
    <EmptyState
      title="No endemic life data"
      hint="Endemic Life is exclusive to MH Wilds. Switch game to see it."
    />
  {:else if loading}
    <div class="space-y-2">
      {#each Array(6) as _}
        <Skeleton class="h-16 w-full" />
      {/each}
    </div>
  {:else if filtered.length === 0}
    <EmptyState title="No endemic life matches your filter" hint="Try a different search." />
  {:else}
    <p class="mb-2 text-xs text-[var(--theme-text-muted)]">{filtered.length} entries</p>
    <div class="space-y-2">
      {#each filtered as e}
        <Card class="p-3">
          <div class="flex flex-wrap items-center gap-2">
            <span class="font-semibold text-gray-100">{e.name}</span>
            {#if e.kind}<Badge>{e.kind}</Badge>{/if}
          </div>
          {#if e.description}
            <p class="mt-1 text-xs text-gray-400">{e.description}</p>
          {/if}
        </Card>
      {/each}
    </div>
  {/if}
</div>
