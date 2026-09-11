<script lang="ts">
  import { goto } from '$app/navigation'
  import ErrorState from '$lib/components/ui/error-state.svelte'
  import { selectedGame } from '$lib/stores/game'
  import { api, type MhwMantle } from '$lib/api'
  import Card from '$lib/components/ui/card.svelte'
  import Badge from '$lib/components/ui/badge.svelte'
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

  interface MantlesSnapshot {
    search: string
    scrollY: number
  }

  export const snapshot: Snapshot<MantlesSnapshot> = {
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
  const mantlesQuery = createQuery(() => ({
    queryKey: ['mhw-mantles', dbId ?? 0],
    queryFn: () => api.getMhwMantles(dbId!),
    enabled: dbId != null,
    ...dbCache,
    retry: dbRetry,
    retryDelay: dbRetryDelay,
  }))

  const mantles = $derived<MhwMantle[]>(
    (mantlesQuery.data ?? []).filter((m) => m.tool_type === 'mantle'),
  )
  const loading = $derived(mantlesQuery.isPending)
  const error = $derived(
    dbErrorText(mantlesQuery.isPending, mantlesQuery.failureCount, mantlesQuery.error),
  )
  let search = $state('')

  const filtered = $derived(
    mantles.filter(
      (m) =>
        search === '' ||
        normKey(m.name).includes(normKey(search)) ||
        normKey(m.effect).includes(normKey(search)),
    ),
  )

  function open(id: number) {
    if (game) goto(`/${game.id}/tools/mantles/${id}`)
  }
</script>

<div class="max-w-6xl mx-auto">
  <div use:toolbarPortal={$toolbarTarget} class="flex flex-wrap gap-2 items-center">
    <SearchField
      bind:value={search}
      placeholder="Search mantles..."
      label="Search mantles"
      class="sm:w-64"
    />
    <span class="text-xs text-gray-400" role="status"
      >{filtered.length} / {mantles.length} mantles</span
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
    <EmptyState title="No mantles match your search." hint="Try another search term." />
  {:else}
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
      {#each filtered as m (m.id)}
        <button
          onclick={() => open(m.id)}
          aria-label="Open {m.name}"
          class="text-left rounded-lg min-h-[44px] focus-visible:outline-none focus-visible:ring-2"
        >
          <Card variant="themed" class="p-4 cursor-pointer">
            <div class="flex items-start gap-3">
              <ItemIcon
                iconUrl={m.icon_url}
                iconName={m.icon_name}
                iconColor={m.icon_color}
                size={36}
                alt=""
              />
              <div class="min-w-0 flex-1">
                <p class="font-semibold text-gray-100 truncate">{m.name}</p>
                <p class="text-[11px] text-gray-400 mt-0.5 line-clamp-2 leading-tight">
                  {m.effect}
                </p>
                <p class="text-[11px] mt-1 flex flex-wrap gap-x-2 gap-y-0.5 text-gray-400">
                  <span class="inline-flex items-center gap-1"
                    ><Timer class="h-3 w-3 shrink-0" aria-hidden="true" />{m.duration_sec ??
                      '-'}s</span
                  >
                  <span class="inline-flex items-center gap-1 text-gray-400"
                    ><RefreshCw class="h-3 w-3 shrink-0" aria-hidden="true" />{m.cooldown_sec ??
                      '-'}s → {m.cooldown_upgraded_sec ?? '-'}s (+)</span
                  >
                </p>
                {#if m.slots && m.slots !== '---'}
                  <p class="text-[10px] text-[var(--theme-accent)] mt-1">Slots: {m.slots}</p>
                {/if}
              </div>
              <Badge tone="neutral">Mantle</Badge>
            </div>
            {#if m.upgrade_quest}
              <p
                class="text-[10px] text-emerald-300/80 mt-2 truncate inline-flex items-center gap-1"
              >
                <Star class="h-3 w-3 shrink-0" aria-hidden="true" />{m.upgrade_quest}
              </p>
            {/if}
          </Card>
        </button>
      {/each}
    </div>
  {/if}
</div>
