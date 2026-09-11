<script lang="ts">
  import { goto } from '$app/navigation'
  import { selectedGame } from '$lib/stores/game'
  import { api, type MhwMantle } from '$lib/api'
  import Card from '$lib/components/ui/card.svelte'
  import ErrorState from '$lib/components/ui/error-state.svelte'
  import Skeleton from '$lib/components/ui/skeleton.svelte'
  import EmptyState from '$lib/components/ui/empty-state.svelte'
  import ItemIcon from '$lib/components/item-icon.svelte'
  import SearchField from '$lib/components/ui/search-field.svelte'
  import { normKey } from '$lib/utils/norm'
  import { toolbarTarget } from '$lib/stores/toolbar'
  import { toolbarPortal } from '$lib/actions/toolbar-portal'
  import { captureScrollY, restoreScrollY } from '$lib/utils/scroll-restore'
  import { createQuery } from '@tanstack/svelte-query'
  import { dbCache, dbRetry, dbRetryDelay, dbErrorText } from '$lib/query'
  import type { Snapshot } from './$types.js'

  interface BoostersSnapshot {
    search: string
    scrollY: number
  }

  export const snapshot: Snapshot<BoostersSnapshot> = {
    capture: () => ({ search, scrollY: captureScrollY() }),
    restore: (s) => {
      search = s.search
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
  import { Timer, RefreshCw, Star } from '@lucide/svelte'
  const game = $derived($selectedGame)
  const dbId = $derived(game?.dbId)
  // Mantles + boosters share one cached query (split client-side by tool_type).
  const boostersQuery = createQuery(() => ({
    queryKey: ['mhw-mantles', dbId ?? 0],
    queryFn: () => api.getMhwMantles(dbId!),
    enabled: dbId != null,
    ...dbCache,
    retry: dbRetry,
    retryDelay: dbRetryDelay,
  }))

  const list = $derived<MhwMantle[]>(
    (boostersQuery.data ?? []).filter((m) => m.tool_type === 'booster'),
  )
  const loading = $derived(boostersQuery.isPending)
  const error = $derived(
    dbErrorText(boostersQuery.isPending, boostersQuery.failureCount, boostersQuery.error),
  )
  let search = $state('')
  const filtered = $derived(
    list.filter((m) => search === '' || normKey(m.name).includes(normKey(search))),
  )
  function open(id: number) {
    if (game) goto(`/${game.id}/tools/boosters/${id}`)
  }
</script>

<div class="max-w-6xl mx-auto">
  <div use:toolbarPortal={$toolbarTarget} class="flex flex-wrap gap-2 items-center">
    <SearchField
      bind:value={search}
      placeholder="Search boosters..."
      label="Search boosters"
      class="sm:w-64"
    />
    <span class="text-xs text-gray-400" role="status"
      >{filtered.length} / {list.length} boosters</span
    >
  </div>
  {#if loading}<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3" aria-busy="true">
      {#each Array(3) as _}
        <Skeleton lines={3} />
      {/each}
    </div>
  {:else if error}
    <ErrorState title="Failed to load boosters" {error} />
  {:else if filtered.length === 0}
    <EmptyState title="No boosters match your search." hint="Try another search term." />
  {:else}
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
      {#each filtered as m (m.id)}
        <button
          onclick={() => open(m.id)}
          aria-label="Open {m.name}"
          class="text-left rounded-lg min-h-[44px] focus-visible:outline-none focus-visible:ring-2"
        >
          <Card variant="themed" class="p-4">
            <div class="flex items-start gap-3">
              <ItemIcon
                iconUrl={m.icon_url}
                iconName={m.icon_name}
                iconColor={m.icon_color}
                size={36}
                alt=""
              />
              <div class="min-w-0 flex-1">
                <p class="font-semibold text-gray-100">{m.name}</p>
                <p class="text-[11px] text-gray-400 mt-1 line-clamp-2">{m.effect}</p>
                <p class="text-[11px] mt-1 text-gray-400 flex flex-wrap gap-x-2 gap-y-0.5">
                  <span class="inline-flex items-center gap-1"
                    ><Timer class="h-3 w-3 shrink-0" aria-hidden="true" />{m.duration_sec}s</span
                  >
                  <span class="inline-flex items-center gap-1"
                    ><RefreshCw class="h-3 w-3 shrink-0" aria-hidden="true" />{m.cooldown_sec}s → {m.cooldown_upgraded_sec}s
                    (+)</span
                  >
                </p>
                {#if m.upgrade_quest}<p
                    class="text-[10px] text-emerald-300/80 mt-1 truncate inline-flex items-center gap-1"
                  >
                    <Star class="h-3 w-3 shrink-0" aria-hidden="true" />{m.upgrade_quest}
                  </p>{/if}
              </div>
            </div>
          </Card>
        </button>
      {/each}
    </div>
  {/if}
</div>
