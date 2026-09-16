<script lang="ts">
  import { goto } from '$app/navigation'
  import ErrorState from '$lib/components/ui/error-state.svelte'
  import { selectedGame } from '$lib/stores/game'
  import { api, type Armor, type ArmorSet } from '$lib/api'
  import { normKey } from '$lib/utils/norm'
  import { rankTone } from '$lib/utils/mh'
  import Card from '$lib/components/ui/card.svelte'
  import Button from '$lib/components/ui/button.svelte'
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
  import type { Snapshot } from './$types.js'
  import { createQuery } from '@tanstack/svelte-query'
  import { dbCache, dbRetry, dbRetryDelay, dbPreparing } from '$lib/query'

  interface ArmorSnapshot {
    searchTerm: string
    rankFilter: string
    genderFilter: string
    typeFilter: string
    sortBy: string
    viewMode: 'sets' | 'pieces'
    visibleCount: number
    scrollY: number
  }

  export const snapshot: Snapshot<ArmorSnapshot> = {
    capture: () => ({
      searchTerm,
      rankFilter,
      genderFilter,
      typeFilter,
      sortBy,
      viewMode,
      visibleCount,
      scrollY: captureScrollY(),
    }),
    restore: (s) => {
      searchTerm = s.searchTerm
      rankFilter = s.rankFilter
      genderFilter = s.genderFilter
      typeFilter = s.typeFilter
      sortBy = s.sortBy
      viewMode = s.viewMode
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

  const game = $derived($selectedGame)
  const dbId = $derived(game?.dbId)

  $effect(() => {
    if (game) void favorites.ensure(game.dbId)
  })

  const favKeys = $derived(new Set(game ? [...($favorites.get(game.dbId)?.keys() ?? [])] : []))

  const PAGE_SIZE = 100

  const armorQuery = createQuery(() => ({
    queryKey: ['armor', dbId ?? 0],
    queryFn: () => api.getArmor(dbId!),
    enabled: dbId != null,
    ...dbCache,
    retry: dbRetry,
    retryDelay: dbRetryDelay,
  }))
  const armorSetsQuery = createQuery(() => ({
    queryKey: ['armor-sets', dbId ?? 0],
    queryFn: () => api.getArmorSets(dbId!),
    enabled: dbId != null,
    ...dbCache,
    retry: dbRetry,
    retryDelay: dbRetryDelay,
  }))

  const armors = $derived<Armor[]>(armorQuery.data ?? [])
  const armorSets = $derived<ArmorSet[]>(armorSetsQuery.data ?? [])
  const loading = $derived(armorQuery.isPending || armorSetsQuery.isPending)
  const error = $derived.by<string | null>(() => {
    const preparing =
      dbPreparing(armorQuery.isPending, armorQuery.failureCount) ||
      dbPreparing(armorSetsQuery.isPending, armorSetsQuery.failureCount)
    if (preparing) return 'Preparing database...'
    const e = armorQuery.error ?? armorSetsQuery.error
    if (loading || e == null) return null
    return e instanceof Error ? e.message : String(e)
  })
  let rankFilter = $state<string>('all')
  let genderFilter = $state<string>('both') // both (show all) | male | female
  let typeFilter = $state<string>('all') // all | blade | gunner
  let sortBy = $state<string>('smith') // smith = armorer list (rank -> slot -> id) faithful to ISO 37652906 string table
  let viewMode = $state<'sets' | 'pieces'>('sets')
  let searchTerm = $state('')
  let showFavsOnly = $state(false)
  let visibleCount = $state(PAGE_SIZE)
  // Skipped once after a snapshot restore (back navigation keeps its depth).
  let skipReset = $state(false)

  // Reset pagination on game change or new criteria (data itself comes from cache)
  $effect(() => {
    void dbId
    void rankFilter
    void genderFilter
    void typeFilter
    void sortBy
    void searchTerm
    void viewMode
    if (skipReset) {
      skipReset = false
      return
    }
    visibleCount = PAGE_SIZE
  })

  const ranks = $derived(['all', ...Array.from(new Set(armors.map((a) => a.rank)))])

  function matchesGender(a: Armor): boolean {
    if (genderFilter === 'both') return true // Both = show all (All+Both redundant)
    // male/female: include Both + specific gender
    const g = a.gender ?? 'both'
    return g === 'both' || g === genderFilter
  }

  // For armor_type both on head: higher defense = blademaster (Helm), lower = gunner (Cap)
  // Precompute blademaster heads (higher defense per set+rank+head)
  const blademasterHeadIds = $derived.by(() => {
    const heads = armors.filter(
      (a) => a.slot_type === 'head' && (a.armor_type ?? 'both') === 'both',
    )
    const byKey = new Map<string, Armor[]>()
    for (const h of heads) {
      const key = `${h.set_id ?? h.name.split(' ').slice(0, -1).join(' ')}|${h.rank}`
      const arr = byKey.get(key) ?? []
      arr.push(h)
      byKey.set(key, arr)
    }
    const ids = new Set<number>()
    for (const [, arr] of byKey) {
      if (arr.length === 1) {
        // single head usable by both -> shown in both filters, treat as blademaster for helper but not filtered
        continue
      }
      if (arr.length >= 2) {
        const sorted = [...arr].sort((a, b) => (b.defense_base ?? 0) - (a.defense_base ?? 0))
        // higher defense is blademaster
        ids.add(sorted[0].id)
      }
    }
    return ids
  })

  function isBlademasterHead(a: Armor): boolean {
    return blademasterHeadIds.has(a.id)
  }

  function matchesType(a: Armor): boolean {
    if (typeFilter === 'all') return true
    const t = (a.armor_type ?? 'both').toLowerCase()
    if (t === 'blade') return typeFilter === 'blade'
    if (t === 'gunner') return typeFilter === 'gunner'
    // both
    if (a.slot_type !== 'head') {
      // both for non-head is usable by both -> show in both filters
      return true
    }
    // head with both: distinguish by defense
    const isBladeHead = isBlademasterHead(a)
    // single head variant (only one per set/rank) -> show in both
    const headsSameKey = armors.filter(
      (x) =>
        x.slot_type === 'head' &&
        (x.armor_type ?? 'both') === 'both' &&
        `${x.set_id ?? x.name.split(' ').slice(0, -1).join(' ')}|${x.rank}` ===
          `${a.set_id ?? a.name.split(' ').slice(0, -1).join(' ')}|${a.rank}`,
    )
    if (headsSameKey.length === 1) return true
    return typeFilter === 'blade' ? isBladeHead : !isBladeHead
  }

  const filtered = $derived.by(() => {
    let arr = armors.filter(
      (a) =>
        (rankFilter === 'all' || a.rank === rankFilter) &&
        matchesGender(a) &&
        matchesType(a) &&
        (searchTerm === '' || normKey(a.name).includes(normKey(searchTerm))) &&
        (!showFavsOnly || favKeys.has(`armor:${a.id}`)),
    )
    if (sortBy === 'name') arr = [...arr].sort((a, b) => a.name.localeCompare(b.name))
    else if (sortBy === 'rarity') arr = [...arr].sort((a, b) => (b.rarity ?? 0) - (a.rarity ?? 0))
    else if (sortBy === 'defense')
      arr = [...arr].sort((a, b) => (b.defense_base ?? 0) - (a.defense_base ?? 0))
    else if (sortBy === 'slots')
      arr = [...arr].sort((a, b) => parseInt(b.slots ?? '0') - parseInt(a.slots ?? '0'))
    return arr
  })

  const filteredSets = $derived.by(() => {
    // A set is included if it has at least one piece matching rank + gender + type
    const setIds = new Set(
      armors
        .filter(
          (a) =>
            (rankFilter === 'all' || a.rank === rankFilter) && matchesGender(a) && matchesType(a),
        )
        .map((a) => a.set_id)
        .filter((x): x is number => x != null),
    )
    let arr = armorSets
      .filter((s) => setIds.has(s.id))
      .filter((s) => searchTerm === '' || normKey(s.name).includes(normKey(searchTerm)))
      .filter((s) => !showFavsOnly || favKeys.has(`armor_set:${s.id}`))
    if (sortBy === 'name') arr = [...arr].sort((a, b) => a.name.localeCompare(b.name))
    else if (sortBy === 'rarity') arr = [...arr].sort((a, b) => (b.rarity ?? 0) - (a.rarity ?? 0))
    else if (sortBy === 'defense') arr = [...arr].sort((a, b) => (b.rarity ?? 0) - (a.rarity ?? 0))
    return arr
  })

  const visibleSets = $derived(filteredSets.slice(0, visibleCount))
  const visiblePieces = $derived(filtered.slice(0, visibleCount))

  function open(id: number) {
    if (!game) return
    goto(`/${game.id}/armor/${id}`)
  }
  function openSet(id: number) {
    if (!game) return
    goto(`/${game.id}/armor/sets/${id}`)
  }

  const slotLabel: Record<string, string> = {
    head: 'Helm',
    chest: 'Mail',
    arms: 'Vambraces',
    waist: 'Coil',
    legs: 'Greaves',
  }

  function setLabel(s: { piece_count: number }): string {
    if (s.piece_count === 1) return 'Singleton — e.g., Black Legs (no full set)'
    if (s.piece_count >= 10) return 'Full set (Blade + Gunner, 10)'
    if (s.piece_count === 5) return 'Full set (5)'
    return `${s.piece_count} pieces`
  }
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-4 md:mb-6">
    <h1 class="fluid-h2 font-bold text-gray-100">Armor</h1>
    <p class="text-sm text-gray-400 mt-1">
      {#if game}
        {game.shortName} · {armors.length} pieces · {armorSets.length} sets
      {:else}
        Select a game first
      {/if}
    </p>
  </div>

  {#if loading}
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-3" aria-busy="true">
      {#each Array(6) as _}
        <Skeleton lines={3} />
      {/each}
    </div>
  {:else if error}
    <ErrorState title="Failed to load armor" {error} />
  {:else if armors.length === 0}
    <EmptyState
      title="No armor found"
      hint={game ? `No armor seeded for ${game.shortName}.` : 'Select a game first.'}
    />
  {:else}
    <div use:toolbarPortal={$toolbarTarget} class="flex flex-col gap-2">
      <div class="flex flex-wrap gap-2 items-center">
        <div
          class="flex rounded-full border border-[var(--theme-border)] overflow-hidden"
          role="group"
          aria-label="Change view"
        >
          <button
            type="button"
            onclick={() => (viewMode = 'sets')}
            aria-pressed={viewMode === 'sets'}
            class="px-4 min-h-[44px] sm:min-h-[36px] text-xs font-medium focus-visible:outline-none focus-visible:ring-2 {viewMode ===
            'sets'
              ? 'text-[var(--theme-text-on-primary)]'
              : 'text-gray-400 hover:text-gray-200'}"
            style={viewMode === 'sets'
              ? 'background-color: var(--theme-primary);'
              : 'background-color: var(--theme-bg-surface);'}>Sets ({armorSets.length})</button
          >
          <button
            type="button"
            onclick={() => (viewMode = 'pieces')}
            aria-pressed={viewMode === 'pieces'}
            class="px-4 min-h-[44px] sm:min-h-[36px] text-xs font-medium focus-visible:outline-none focus-visible:ring-2 {viewMode ===
            'pieces'
              ? 'text-[var(--theme-text-on-primary)]'
              : 'text-gray-400 hover:text-gray-200'}"
            style={viewMode === 'pieces'
              ? 'background-color: var(--theme-primary);'
              : 'background-color: var(--theme-bg-surface);'}>Pieces ({filtered.length})</button
          >
        </div>
        <SearchField
          bind:value={searchTerm}
          placeholder="Search armor..."
          label="Search armor"
          class="sm:w-48"
        />
        <select
          bind:value={sortBy}
          aria-label="Sort armor"
          class="px-3 rounded-full bg-[var(--theme-bg-surface)] border border-[var(--theme-border)] text-gray-300 focus:outline-none min-h-[44px] sm:min-h-[36px] text-base sm:text-xs"
        >
          <option value="smith">Smith (Game Order)</option>
          <option value="name">Name A-Z</option>
          <option value="rarity">Rarity ↓</option>
          <option value="defense">Defense ↓</option>
          <option value="slots">Slots ↓</option>
        </select>
        <span
          class="flex rounded-full border border-[var(--theme-border)] overflow-hidden text-xs"
          role="group"
          aria-label="Filter by gender"
        >
          {#each ['both', 'male', 'female'] as g}
            <button
              type="button"
              onclick={() => (genderFilter = g)}
              aria-pressed={genderFilter === g}
              class="px-3 min-h-[44px] sm:min-h-[36px] transition-colors motion-safe:transition-colors motion-reduce:transition-none focus-visible:outline-none focus-visible:ring-2 {genderFilter ===
              g
                ? 'text-[var(--theme-text-on-primary)]'
                : 'text-gray-400 hover:text-gray-200'}"
              style={genderFilter === g
                ? 'background-color: var(--theme-primary);'
                : 'background-color: var(--theme-bg-surface);'}
            >
              {g === 'both' ? 'Both' : g === 'male' ? 'Male' : 'Female'}
            </button>
          {/each}
        </span>
        <span
          class="flex rounded-full border border-[var(--theme-border)] overflow-hidden text-xs"
          role="group"
          aria-label="Filter by weapon class"
        >
          {#each ['all', 'blade', 'gunner'] as t}
            <button
              type="button"
              onclick={() => (typeFilter = t)}
              aria-pressed={typeFilter === t}
              class="px-3 min-h-[44px] sm:min-h-[36px] transition-colors motion-safe:transition-colors motion-reduce:transition-none focus-visible:outline-none focus-visible:ring-2 {typeFilter ===
              t
                ? 'text-[var(--theme-text-on-primary)]'
                : 'text-gray-400 hover:text-gray-200'}"
              style={typeFilter === t
                ? 'background-color: var(--theme-primary);'
                : 'background-color: var(--theme-bg-surface);'}
            >
              {t === 'all' ? 'All' : t === 'blade' ? 'Blademaster' : 'Gunner'}
            </button>
          {/each}
        </span>
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
      <div class="flex gap-2 overflow-x-auto pb-1 -mb-1" role="group" aria-label="Filter by rank">
        {#each ranks as rank}
          <FilterChip
            active={rankFilter === rank}
            onclick={() => (rankFilter = rank)}
            label={rank === 'all' ? 'All ranks' : `${rank} rank`}
          >
            {rank === 'all' ? 'All' : rank}
          </FilterChip>
        {/each}
      </div>
    </div>

    {#if viewMode === 'sets'}
      {#if filteredSets.length === 0}
        <div class="mt-4">
          <EmptyState
            title="No sets match current filters"
            hint="Try widening rank, gender or class filters."
          />
        </div>
      {:else}
        <div class="mt-4 grid grid-cols-1 lg:grid-cols-2 gap-3">
          {#each visibleSets as set (set.id)}
            {@const pieces = armors
              .filter((a) => a.set_id === set.id && matchesGender(a) && matchesType(a))
              .slice(0, 6)}
            <div class="relative">
              <button
                onclick={() => openSet(set.id)}
                aria-label="Open {set.name} set"
                class="text-left rounded-lg min-h-[44px] w-full focus-visible:outline-none focus-visible:ring-2"
              >
                <Card variant="themed" class="p-4">
                  <div class="flex items-start justify-between gap-2 mb-2">
                    <h3 class="font-semibold text-gray-100 truncate">{set.name}</h3>
                    <Badge tone={rankTone(set.rank)}>
                      {set.rank ?? 'Low'} · {set.piece_count} pcs
                    </Badge>
                  </div>
                  <p class="text-xs text-gray-400 mb-2">
                    {setLabel(set)} · R{set.rarity ?? 1}
                  </p>
                  <div class="flex flex-wrap gap-1">
                    {#each pieces as p}
                      <span
                        class="text-[10px] px-2 py-1 rounded bg-[var(--theme-bg-elevated)] border border-[var(--theme-border)] text-gray-300 inline-flex items-center gap-1"
                        ><ItemIcon
                          iconUrl={p.icon_url}
                          iconName={p.icon_name}
                          iconColor={p.icon_color}
                          size={14}
                          alt=""
                        />{p.name}
                        <span class="text-gray-400">[{slotLabel[p.slot_type] ?? p.slot_type}]</span
                        ></span
                      >
                    {/each}
                    {#if set.piece_count > pieces.length}
                      <span class="text-[10px] px-2 py-1 rounded bg-gray-800 text-gray-400"
                        >+{set.piece_count - pieces.length} more</span
                      >
                    {/if}
                  </div>
                </Card>
              </button>
              <div class="absolute top-2 right-2">
                <FavoriteButton kind="armor_set" id={set.id} name={set.name} size="sm" />
              </div>
            </div>
          {/each}
        </div>
        {#if filteredSets.length > visibleSets.length}
          <div class="mt-6 flex flex-col items-center gap-2">
            <p class="text-xs text-gray-400" role="status">
              Showing {visibleSets.length} of {filteredSets.length}
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
    {:else}
      {#if filtered.length === 0}
        <div class="mt-4">
          <EmptyState
            title="No armor pieces match current filters"
            hint="Try widening your filters."
          />
        </div>
      {:else}
        <div class="mt-4 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
          {#each visiblePieces as piece (piece.id)}
            <div class="relative">
              <button
                onclick={() => open(piece.id)}
                aria-label="Open {piece.name}"
                class="text-left rounded-lg min-h-[44px] w-full focus-visible:outline-none focus-visible:ring-2"
              >
                <Card variant="themed" class="p-4 cursor-pointer">
                  <div class="flex items-center gap-2 mb-2">
                    <ItemIcon
                      iconUrl={piece.icon_url}
                      iconName={piece.icon_name}
                      iconColor={piece.icon_color}
                      size={28}
                      alt=""
                    />
                    <h3 class="font-semibold text-gray-100 truncate flex-1">{piece.name}</h3>
                    <Badge tone={rankTone(piece.rank)}>
                      {piece.rank}
                    </Badge>
                  </div>
                  <p class="text-xs text-gray-400 mb-3">
                    {slotLabel[piece.slot_type] ?? piece.slot_type}
                  </p>
                  <div class="grid grid-cols-2 gap-x-3 gap-y-1 text-xs">
                    <div>
                      <span class="text-gray-400">DEF</span>
                      <span class="text-gray-100 font-medium ml-1 tabular-nums">
                        {piece.defense_base ?? 0}-{piece.defense_max ?? 0}
                      </span>
                    </div>
                    <div>
                      <span class="text-gray-400">Rarity</span>
                      <span class="text-gray-100 font-medium ml-1">{piece.rarity ?? 1}</span>
                    </div>
                    <div>
                      <span class="text-orange-400">Fire</span>
                      <span class="text-gray-100 ml-1 tabular-nums"
                        >{piece.resistance_fire ?? 0}</span
                      >
                    </div>
                    <div>
                      <span class="text-blue-400">Water</span>
                      <span class="text-gray-100 ml-1 tabular-nums"
                        >{piece.resistance_water ?? 0}</span
                      >
                    </div>
                    <div>
                      <span class="text-yellow-400">Thunder</span>
                      <span class="text-gray-100 ml-1 tabular-nums"
                        >{piece.resistance_thunder ?? 0}</span
                      >
                    </div>
                    <div>
                      <span class="text-cyan-400">Ice</span>
                      <span class="text-gray-100 ml-1 tabular-nums"
                        >{piece.resistance_ice ?? 0}</span
                      >
                    </div>
                    <div class="col-span-2">
                      <span class="text-purple-400">Dragon</span>
                      <span class="text-gray-100 ml-1 tabular-nums"
                        >{piece.resistance_dragon ?? 0}</span
                      >
                    </div>
                  </div>
                </Card>
              </button>
              <div class="absolute top-2 right-2">
                <FavoriteButton kind="armor" id={piece.id} name={piece.name} size="sm" />
              </div>
            </div>
          {/each}
        </div>
        {#if filtered.length > visiblePieces.length}
          <div class="mt-6 flex flex-col items-center gap-2">
            <p class="text-xs text-gray-400" role="status">
              Showing {visiblePieces.length} of {filtered.length}
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
  {/if}
</div>
