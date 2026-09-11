<script lang="ts">
  import { goto } from '$app/navigation'
  import ErrorState from '$lib/components/ui/error-state.svelte'
  import { selectedGame } from '$lib/stores/game'
  import { api, type PalicoGadget } from '$lib/api'
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

  interface PalicoSnapshot {
    search: string
    subtab: 'gadget' | 'tailraider' | 'safari'
    scrollY: number
  }

  export const snapshot: Snapshot<PalicoSnapshot> = {
    capture: () => ({ search, subtab, scrollY: captureScrollY() }),
    restore: (s) => {
      search = s.search
      subtab = s.subtab
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
  import { Lock } from '@lucide/svelte'

  const game = $derived($selectedGame)
  const dbId = $derived(game?.dbId)
  const gadgetsQuery = createQuery(() => ({
    queryKey: ['palico-gadgets', dbId ?? 0],
    queryFn: () => api.getPalicoGadgets(dbId!),
    enabled: dbId != null,
    ...dbCache,
    retry: dbRetry,
    retryDelay: dbRetryDelay,
  }))

  const gadgets = $derived<PalicoGadget[]>(gadgetsQuery.data ?? [])
  const loading = $derived(gadgetsQuery.isPending)
  const error = $derived(
    dbErrorText(gadgetsQuery.isPending, gadgetsQuery.failureCount, gadgetsQuery.error),
  )
  let subtab = $state<'gadget' | 'tailraider' | 'safari'>('gadget')
  let search = $state('')

  const filtered = $derived(
    gadgets.filter((g) => {
      const typeOk = subtab === 'gadget' ? g.gadget_type === 'gadget' : g.gadget_type === subtab
      const searchOk =
        search === '' ||
        normKey(g.name).includes(normKey(search)) ||
        normKey(g.tribe ?? '').includes(normKey(search))
      return typeOk && searchOk
    }),
  )

  function open(id: number) {
    if (game) goto(`/${game.id}/tools/palico/${id}`)
  }
</script>

<div class="max-w-6xl mx-auto">
  <div
    use:toolbarPortal={$toolbarTarget}
    class="flex flex-wrap gap-2 items-center"
    role="group"
    aria-label="Palico tool filters"
  >
    <button
      onclick={() => (subtab = 'gadget')}
      aria-pressed={subtab === 'gadget'}
      class="px-3 min-h-[44px] sm:min-h-[36px] rounded-full text-xs font-medium border transition-colors motion-safe:transition-colors motion-reduce:transition-none focus-visible:outline-none focus-visible:ring-2 {subtab ===
      'gadget'
        ? 'bg-[var(--theme-primary)] text-[var(--theme-text-on-primary)] border-transparent'
        : 'bg-[var(--theme-bg-surface)] text-gray-400 border-[var(--theme-border)] hover:text-gray-200'}"
      >Gadgets (6)</button
    >
    <button
      onclick={() => (subtab = 'tailraider')}
      aria-pressed={subtab === 'tailraider'}
      class="px-3 min-h-[44px] sm:min-h-[36px] rounded-full text-xs font-medium border transition-colors motion-safe:transition-colors motion-reduce:transition-none focus-visible:outline-none focus-visible:ring-2 {subtab ===
      'tailraider'
        ? 'bg-[var(--theme-primary)] text-[var(--theme-text-on-primary)] border-transparent'
        : 'bg-[var(--theme-bg-surface)] text-gray-400 border-[var(--theme-border)] hover:text-gray-200'}"
      >Tailraider Signal</button
    >
    <button
      onclick={() => (subtab = 'safari')}
      aria-pressed={subtab === 'safari'}
      class="px-3 min-h-[44px] sm:min-h-[36px] rounded-full text-xs font-medium border transition-colors motion-safe:transition-colors motion-reduce:transition-none focus-visible:outline-none focus-visible:ring-2 {subtab ===
      'safari'
        ? 'bg-[var(--theme-primary)] text-[var(--theme-text-on-primary)] border-transparent'
        : 'bg-[var(--theme-bg-surface)] text-gray-400 border-[var(--theme-border)] hover:text-gray-200'}"
      >Safari</button
    >
    <SearchField
      bind:value={search}
      placeholder="Search..."
      label="Search Palico tools"
      class="sm:w-auto sm:flex-1 sm:min-w-40"
    />
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
    <EmptyState title="No results for {subtab}." hint="Try another search term." />
  {:else}
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
      {#each filtered as g (g.id)}
        <button
          onclick={() => open(g.id)}
          aria-label="Open {g.name}"
          class="text-left rounded-lg min-h-[44px] focus-visible:outline-none focus-visible:ring-2"
        >
          <Card variant="themed" class="p-4">
            <div class="flex items-start gap-3">
              <ItemIcon
                iconUrl={g.icon_url}
                iconName={g.icon_name}
                iconColor={g.icon_color}
                size={36}
                alt=""
              />
              <div class="min-w-0 flex-1">
                <p class="font-semibold text-gray-100">{g.name}</p>
                {#if g.tribe}<p class="text-[11px] text-[var(--theme-accent)] mt-0.5">
                    {g.tribe}
                  </p>{/if}
                <p class="text-[11px] text-gray-400 mt-1 line-clamp-2">
                  {g.effect ?? g.description}
                </p>
                {#if g.acquisition}<p
                    class="text-[10px] text-gray-400 mt-2 line-clamp-2 inline-flex items-start gap-1"
                  >
                    <Lock class="h-3 w-3 shrink-0 mt-px" aria-hidden="true" />{g.acquisition}
                  </p>{/if}
                <Badge tone="neutral" class="mt-2">{g.gadget_type}</Badge>
              </div>
            </div>
          </Card>
        </button>
      {/each}
    </div>
  {/if}
</div>
