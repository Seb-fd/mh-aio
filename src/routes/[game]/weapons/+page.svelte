<script lang="ts">
  import { goto } from '$app/navigation'
  import ErrorState from '$lib/components/ui/error-state.svelte'
  import { selectedGame } from '$lib/stores/game'
  import { api, type Weapon } from '$lib/api'
  import ItemIcon from '$lib/components/item-icon.svelte'
  import Button from '$lib/components/ui/button.svelte'
  import Skeleton from '$lib/components/ui/skeleton.svelte'
  import EmptyState from '$lib/components/ui/empty-state.svelte'
  import FavoriteButton from '$lib/components/favorite-button.svelte'
  import { favorites } from '$lib/stores/favorites'
  import { toolbarTarget } from '$lib/stores/toolbar'
  import { toolbarPortal } from '$lib/actions/toolbar-portal'
  import { captureScrollY, restoreScrollY } from '$lib/utils/scroll-restore'
  import type { Snapshot } from './$types.js'

  interface WeaponsSnapshot {
    typeFilter: string
    sortBy: string
    collapsed: number[]
    rootLimit: number
    typeMemory: Record<string, { scrollY: number; rootLimit: number }>
    scrollY: number
  }

  export const snapshot: Snapshot<WeaponsSnapshot> = {
    capture: () => ({
      typeFilter,
      sortBy,
      collapsed: [...collapsed],
      rootLimit,
      typeMemory,
      scrollY: captureScrollY(),
    }),
    restore: (s) => {
      typeFilter = s.typeFilter
      sortBy = s.sortBy
      collapsed = new Set(s.collapsed)
      rootLimit = s.rootLimit
      typeMemory = s.typeMemory
      prevType = s.typeFilter
      prevSort = s.sortBy
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
  import {
    elementColor,
    sharpnessValues,
    SHARP_COLORS_ARR as SHARP_COLORS,
    buildWeaponForest,
  } from '$lib/utils/mh'
  import { ChevronRight, Hammer, Star } from '@lucide/svelte'
  import { tick } from 'svelte'
  import { createQuery } from '@tanstack/svelte-query'
  import { dbCache, dbRetry, dbRetryDelay, dbErrorText } from '$lib/query'

  const game = $derived($selectedGame)
  const dbId = $derived(game?.dbId)

  const weaponsQuery = createQuery(() => ({
    queryKey: ['weapons', dbId ?? 0],
    queryFn: () => api.getWeapons(dbId!),
    enabled: dbId != null,
    ...dbCache,
    retry: dbRetry,
    retryDelay: dbRetryDelay,
  }))

  const weapons = $derived<Weapon[]>(weaponsQuery.data ?? [])
  const loading = $derived(weaponsQuery.isPending)
  const error = $derived(
    dbErrorText(weaponsQuery.isPending, weaponsQuery.failureCount, weaponsQuery.error),
  )
  let typeFilter = $state<string>('Great Sword')
  let sortBy = $state<string>('smith') // smith = armorer tree order (weapon_type -> id) faithful to ISO
  let collapsed = $state<Set<number>>(new Set())
  let rootLimit = $state(30)
  const ROOT_PAGE = 30
  // Per-type memory: depth + scroll position, so switching types restores each
  // type where you left it (also persisted in the snapshot for back navigation).
  let typeMemory = $state<Record<string, { scrollY: number; rootLimit: number }>>({})
  let prevType = $state<string | null>(null)
  let prevSort = $state<string | null>(null)
  let showFavsOnly = $state(false)

  $effect(() => {
    if (game) void favorites.ensure(game.dbId)
  })

  const favKeys = $derived(new Set(game ? [...($favorites.get(game.dbId)?.keys() ?? [])] : []))

  function switchType(next: string) {
    if (next === typeFilter) return
    const main = document.getElementById('main-content')
    typeMemory = { ...typeMemory, [typeFilter]: { scrollY: main?.scrollTop ?? 0, rootLimit } }
    typeFilter = next
  }

  // Ensure filter is always a valid weapon type (default Great Sword)
  $effect(() => {
    if (weaponTypes.length === 0) return
    if (!weaponTypes.includes(typeFilter)) {
      typeFilter = weaponTypes.includes('Great Sword') ? 'Great Sword' : weaponTypes[0]
    }
  })

  // Type switch restores that type's depth + position; sort change resets the
  // current type to the top (new order, new view). Mount/snapshot-restore syncs
  // prev markers without acting (pendingScrollY owns the initial scroll).
  $effect(() => {
    const t = typeFilter
    const s = sortBy
    if (prevType === null || prevSort === null) {
      prevType = t
      prevSort = s
      return
    }
    if (t !== prevType) {
      prevType = t
      prevSort = s
      const mem = typeMemory[t]
      rootLimit = mem?.rootLimit ?? ROOT_PAGE
      const y = mem?.scrollY ?? 0
      const main = document.getElementById('main-content')
      tick().then(() => main?.scrollTo(0, y))
    } else if (s !== prevSort) {
      prevSort = s
      rootLimit = ROOT_PAGE
      typeMemory = { ...typeMemory, [t]: { scrollY: 0, rootLimit: ROOT_PAGE } }
      document.getElementById('main-content')?.scrollTo(0, 0)
    }
  })

  function toggleNode(id: number) {
    const next = new Set(collapsed)
    if (next.has(id)) next.delete(id)
    else next.add(id)
    collapsed = next
  }

  const GAME_WEAPON_ORDER = [
    'Great Sword',
    'Long Sword',
    'Sword & Shield',
    'Dual Blades',
    'Hammer',
    'Hunting Horn',
    'Lance',
    'Gunlance',
    'Switch Axe',
    'Charge Blade',
    'Insect Glaive',
    'Light Bowgun',
    'Heavy Bowgun',
    'Bow',
  ]
  function weaponOrder(t: string): number {
    const i = GAME_WEAPON_ORDER.indexOf(t)
    if (i !== -1) return i
    if (t === 'Sword and Shield') return 2
    return 99
  }
  // Base type icons live in static/icons/mhfu/weapons/ ({slug}.png). ItemIcon
  // falls back to initials if a future type has no file (no visible 404).
  function weaponTypeIcon(type: string): string {
    const slug = type.toLowerCase().replace(/&/g, 'and').replace(/\s+/g, '-').replace(/-+/g, '-')
    return `/icons/mhfu/weapons/${slug}.png`
  }
  const weaponTypes = $derived(
    Array.from(new Set(weapons.map((w) => w.weapon_type))).sort(
      (a, b) => weaponOrder(a) - weaponOrder(b),
    ),
  )
  const filtered = $derived.by(() => {
    let arr = weapons
      .filter((w) => w.weapon_type === typeFilter)
      .filter((w) => !showFavsOnly || favKeys.has(`weapon:${w.id}`))
    if (sortBy === 'name') arr = [...arr].sort((a, b) => a.name.localeCompare(b.name))
    else if (sortBy === 'rarity') arr = [...arr].sort((a, b) => (b.rarity ?? 0) - (a.rarity ?? 0))
    else if (sortBy === 'attack') arr = [...arr].sort((a, b) => (b.attack ?? 0) - (a.attack ?? 0))
    // smith is default already ORDER BY game weapon order, id from queries.rs:699
    return arr
  })

  // Flat render rows — the tree is NEVER rendered recursively (iterative
  // DFS keeps depth unbounded without overflowing the call stack inside
  // Svelte's recursive {@render}).
  interface TreeRow {
    weapon: Weapon
    depth: number
    hasKids: boolean
  }

  interface WeaponTree {
    root: Weapon
    rows: TreeRow[]
  }

  // Max visual nesting: a 300-deep chain at full indent would squeeze content
  // to zero width. DOM order still preserves the parent → child sequence.
  const MAX_INDENT = 8

  function buildTrees(typeWeapons: Weapon[]): WeaponTree[] {
    const sortFn = (a: Weapon, b: Weapon) => {
      if (sortBy === 'name') return a.name.localeCompare(b.name)
      if (sortBy === 'rarity') return (b.rarity ?? 0) - (a.rarity ?? 0)
      if (sortBy === 'attack') return (b.attack ?? 0) - (a.attack ?? 0)
      // smith: use the in-game armor-forge order (sort_order) when present,
      // otherwise fall back to creation order (id) — faithful to the ISO tree.
      return (a.sort_order ?? a.id) - (b.sort_order ?? b.id)
    }
    // ID-keyed forest: names repeat across variants (Wilds Artian x3), so
    // name-keyed maps would merge distinct subtrees (see mh.ts).
    const forest = buildWeaponForest(typeWeapons, sortFn)
    const trees: WeaponTree[] = []
    // emitted: each weapon id renders at most once per type group. A weapon
    // has a single parent so it belongs under exactly one branch — the only
    // way to revisit an id is a stale/duplicate name-cycle, which is dropped.
    const emitted = new Set<number>()
    for (const root of forest.roots) {
      if (emitted.has(root.id)) continue
      const rows: TreeRow[] = []
      const stack: { w: Weapon; depth: number; childIdx: number; kids: Weapon[] }[] = []
      const push = (w: Weapon, depth: number) => {
        const kids = (forest.childrenOf.get(w.id) ?? []).filter((c) => !emitted.has(c.id))
        emitted.add(w.id)
        rows.push({ weapon: w, depth, hasKids: kids.length > 0 })
        stack.push({ w, depth, childIdx: 0, kids: collapsed.has(w.id) ? [] : kids })
      }
      push(root, 0)
      while (stack.length > 0) {
        const top = stack[stack.length - 1]
        if (top.childIdx >= top.kids.length) {
          stack.pop()
          continue
        }
        const child = top.kids[top.childIdx++]
        if (emitted.has(child.id)) continue
        push(child, top.depth + 1)
      }
      trees.push({ root, rows })
    }
    return trees
  }

  const tree = $derived.by<{ type: string; trees: WeaponTree[] }[]>(() => {
    const byType = new Map<string, Weapon[]>()
    for (const w of filtered) {
      const arr = byType.get(w.weapon_type) ?? []
      arr.push(w)
      byType.set(w.weapon_type, arr)
    }
    return [...byType.entries()]
      .sort((a, b) => weaponOrder(a[0]) - weaponOrder(b[0]))
      .map(([type, ws]) => ({ type, trees: buildTrees(ws) }))
  })

  function open(id: number) {
    if (!game) return
    goto(`/${game.id}/weapons/${id}`)
  }

  // Helpers now from $lib/utils/mh (DRY)
  const sharpnessSegments = sharpnessValues
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-4 md:mb-6">
    <h1 class="fluid-h2 font-bold text-gray-100">Weapon Trees</h1>
    <p class="text-sm text-gray-400 mt-1">
      {#if game}
        {game.shortName} · {weapons.length} weapons · craft + upgrade tree
      {:else}
        Select a game first
      {/if}
    </p>
  </div>

  {#if loading}
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-2" aria-busy="true">
      {#each Array(6) as _}
        <Skeleton lines={2} />
      {/each}
    </div>
  {:else if error}
    <ErrorState title="Failed to load weapons" {error} />
  {:else if weapons.length === 0}
    <EmptyState
      title="No weapons found"
      hint={game ? `No weapons for ${game.shortName ?? 'this game'}.` : 'Select a game first.'}
    />
  {:else}
    <div use:toolbarPortal={$toolbarTarget} class="flex flex-col gap-2">
      <div class="flex items-center gap-2">
        <select
          bind:value={sortBy}
          aria-label="Sort weapons"
          class="px-3 rounded-full bg-[var(--theme-bg-surface)] border border-[var(--theme-border)] text-gray-300 focus:outline-none min-h-[44px] sm:min-h-[36px] text-base sm:text-xs"
        >
          <option value="smith">Smith (Game Order)</option>
          <option value="name">Name A-Z</option>
          <option value="rarity">Rarity ↓</option>
          <option value="attack">Attack ↓</option>
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
        <span class="text-xs text-gray-400 ml-auto" role="status">{filtered.length} shown</span>
      </div>
      <div
        class="flex gap-2 overflow-x-auto pb-1 -mb-1"
        role="group"
        aria-label="Filter by weapon type"
      >
        {#each weaponTypes as type}
          {@const typeActive = typeFilter === type}
          <button
            type="button"
            onclick={() => switchType(type)}
            aria-pressed={typeActive}
            aria-label="Show {type} weapons"
            title={type}
            class="shrink-0 flex items-center justify-center rounded-xl border transition-all motion-safe:transition-all motion-reduce:transition-none focus-visible:outline-none focus-visible:ring-2 {typeActive
              ? 'border-[var(--theme-primary)]'
              : 'border-[var(--theme-border)] bg-[var(--theme-bg-surface)] opacity-55 hover:opacity-100 hover:border-[var(--theme-border-strong)]'}"
            style={typeActive
              ? 'background-color: color-mix(in oklab, var(--theme-primary) 15%, transparent); min-width:44px;min-height:44px;'
              : 'min-width:44px;min-height:44px;'}
          >
            <ItemIcon
              iconUrl={weaponTypeIcon(type)}
              iconName={type}
              iconColor="Gray"
              size={28}
              alt=""
            />
          </button>
        {/each}
      </div>
    </div>

    <div class="mt-4 min-w-0">
      {#each tree as group}
        {@const groupIcon = group.trees[0]?.root}
        {@const limited = group.trees.slice(0, rootLimit)}
        <section class="mb-8 min-w-0">
          <h2
            class="text-sm font-semibold uppercase tracking-wider text-gray-400 mb-3 flex items-center gap-2"
          >
            {#if groupIcon}
              <ItemIcon
                iconUrl={groupIcon.icon_url}
                iconName={groupIcon.icon_name}
                iconColor={groupIcon.icon_color}
                size={20}
                alt=""
              />
            {/if}
            {group.type}
            <span class="text-[11px] font-normal normal-case text-gray-400"
              >· {group.trees.length}</span
            >
          </h2>
          {#each limited as t}
            {#each t.rows as row (row.weapon.id)}
              {@render weaponRow(row)}
            {/each}
          {/each}
          {#if group.trees.length > limited.length}
            <div class="mt-3 flex flex-col items-center gap-2">
              <p class="text-xs text-gray-400" role="status">
                Showing {limited.length} of {group.trees.length} trees
              </p>
              <Button
                variant="themedPrimary"
                size="lg"
                class="rounded-full px-6"
                onclick={() => (rootLimit += ROOT_PAGE)}
              >
                Show more trees
              </Button>
            </div>
          {/if}
        </section>
      {/each}
    </div>
  {/if}
</div>

{#snippet weaponRow(row: TreeRow)}
  {@const w = row.weapon}
  {@const isCollapsed = collapsed.has(w.id)}
  <div
    class="mb-1.5 min-w-0 {row.depth > 0
      ? 'border-l border-[var(--theme-border)] ml-5 sm:ml-6 pl-1.5 sm:pl-2'
      : ''}"
    style={row.depth > 1
      ? `margin-left: calc(1.25rem + ${(Math.min(row.depth, MAX_INDENT) - 1) * 0.5}rem);`
      : ''}
  >
    <div class="flex items-stretch gap-1.5 min-w-0">
      {#if row.hasKids}
        <button
          type="button"
          onclick={() => toggleNode(w.id)}
          aria-expanded={!isCollapsed}
          aria-label={isCollapsed
            ? `Expand upgrades of ${w.name}`
            : `Collapse upgrades of ${w.name}`}
          class="shrink-0 self-center flex items-center justify-center rounded-md border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] text-gray-400 hover:text-gray-200 hover:border-[var(--theme-border-strong)] focus-visible:outline-none focus-visible:ring-2"
          style="min-width:44px;min-height:44px;"
        >
          <ChevronRight
            class="h-4 w-4 transition-transform motion-safe:transition-transform motion-reduce:transition-none {isCollapsed
              ? ''
              : 'rotate-90'}"
            aria-hidden="true"
          />
        </button>
      {/if}
      <button
        onclick={() => open(w.id)}
        class="flex-1 min-w-0 text-left px-3 py-2.5 rounded-lg border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] hover:border-[var(--theme-border-strong)] hover:bg-[var(--theme-bg-elevated)] transition-colors motion-safe:transition-colors motion-reduce:transition-none min-h-[44px] focus-visible:outline-none focus-visible:ring-2"
      >
        <div class="flex items-center gap-2 min-w-0">
          <ItemIcon
            iconUrl={w.icon_url}
            iconName={w.icon_name}
            iconColor={w.icon_color}
            size={22}
            alt=""
          />
          {#if w.is_forgeable}
            <Hammer
              class="h-3.5 w-3.5 shrink-0 text-gray-400"
              aria-label="Crafted directly from materials"
            />
          {/if}
          <span
            class="text-[10px] text-gray-400 shrink-0 w-10 text-center rounded bg-[var(--theme-bg-elevated)] py-0.5 border border-[var(--theme-border)]"
            >R{w.rarity ?? 1}</span
          >
          <span class="text-sm text-gray-100 font-medium truncate min-w-0">{w.name}</span>
          <span class="text-[11px] text-gray-400 ml-auto shrink-0 tabular-nums"
            >ATK {w.attack ?? 0}</span
          >
        </div>
        <div class="mt-1 flex items-center gap-2 min-w-0">
          {#if w.element_type}
            <span class="text-[11px] {elementColor(w.element_type)} shrink-0 truncate"
              >{w.element_type} {w.element_value ?? 0}</span
            >
          {/if}
          {#if sharpnessSegments(w.sharpness).length > 0}
            <div
              class="flex items-center gap-[1px] h-1.5 min-w-0 overflow-hidden ml-auto"
              aria-hidden="true"
            >
              {#each sharpnessSegments(w.sharpness) as seg, i}
                {#if seg > 0}
                  <div
                    class="rounded-[1px] shrink-0"
                    style="height: 6px; width: {Math.min(seg, 48)}px; background: {SHARP_COLORS[
                      i
                    ] ?? '#666'};"
                  ></div>
                {/if}
              {/each}
            </div>
          {/if}
        </div>
      </button>
      <div class="self-center shrink-0">
        <FavoriteButton kind="weapon" id={w.id} name={w.name} size="sm" />
      </div>
    </div>
  </div>
{/snippet}
