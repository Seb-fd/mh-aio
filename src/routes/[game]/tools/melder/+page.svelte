<script lang="ts">
  import { goto } from '$app/navigation'
  import ErrorState from '$lib/components/ui/error-state.svelte'
  import { selectedGame } from '$lib/stores/game'
  import { api, type MelderRecipe } from '$lib/api'
  import Card from '$lib/components/ui/card.svelte'
  import Badge from '$lib/components/ui/badge.svelte'
  import Skeleton from '$lib/components/ui/skeleton.svelte'
  import EmptyState from '$lib/components/ui/empty-state.svelte'
  import SearchField from '$lib/components/ui/search-field.svelte'
  import { normKey } from '$lib/utils/norm'
  import { toolbarTarget } from '$lib/stores/toolbar'
  import { toolbarPortal } from '$lib/actions/toolbar-portal'
  import { createQuery } from '@tanstack/svelte-query'
  import { dbCache, dbRetry, dbRetryDelay, dbErrorText } from '$lib/query'

  const game = $derived($selectedGame)
  const dbId = $derived(game?.dbId)

  const melderQuery = createQuery(() => ({
    queryKey: ['melder-recipes', dbId ?? 0],
    queryFn: () => api.getMelderRecipes(dbId!),
    enabled: dbId != null,
    ...dbCache,
    retry: dbRetry,
    retryDelay: dbRetryDelay,
  }))

  const recipes = $derived<MelderRecipe[]>(melderQuery.data ?? [])
  const loading = $derived(melderQuery.isPending)
  const error = $derived(
    dbErrorText(melderQuery.isPending, melderQuery.failureCount, melderQuery.error),
  )
  let search = $state('')

  const filtered = $derived(
    recipes.filter(
      (r) =>
        search === '' ||
        normKey(r.result_name).includes(normKey(search)) ||
        normKey(r.unlock_condition ?? '').includes(normKey(search)),
    ),
  )

  function open(id: number) {
    if (game) goto(`/${game.id}/items/${id}`)
  }
</script>

<div class="max-w-6xl mx-auto">
  <div use:toolbarPortal={$toolbarTarget} class="flex flex-wrap gap-2 items-center">
    <SearchField
      bind:value={search}
      placeholder="Search melder recipes..."
      label="Search melder recipes"
      class="sm:w-64"
    />
    <span class="text-xs text-gray-400" role="status"
      >{filtered.length} / {recipes.length} recipes</span
    >
  </div>
  {#if loading}
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3" aria-busy="true">
      {#each Array(6) as _}
        <Skeleton lines={3} />
      {/each}
    </div>
  {:else if error}
    <ErrorState title="Something went wrong" {error} />
  {:else if filtered.length === 0}
    <EmptyState title="No melder recipes match your search." hint="Try another search term." />
  {:else}
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
      {#each filtered as r (r.id)}
        <button
          onclick={() => open(r.result_item_id)}
          aria-label="Open {r.result_name}"
          class="text-left rounded-lg min-h-[44px] focus-visible:outline-none focus-visible:ring-2"
        >
          <Card variant="themed" class="p-4 cursor-pointer">
            <div class="flex items-start gap-3">
              <div class="min-w-0 flex-1">
                <p class="font-semibold text-gray-100 truncate">{r.result_name}</p>
                <p class="text-[11px] text-gray-400 mt-1">
                  Research {r.research_cost} · Melding {r.melding_cost}
                </p>
                {#if r.unlock_condition}
                  <p class="text-[11px] text-gray-400 mt-1 line-clamp-2">{r.unlock_condition}</p>
                {/if}
              </div>
              <Badge tone="neutral">{r.melder_type}</Badge>
            </div>
          </Card>
        </button>
      {/each}
    </div>
  {/if}
</div>
