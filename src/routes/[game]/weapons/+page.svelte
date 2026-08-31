<script lang="ts">
  import { goto } from '$app/navigation'
  import { selectedGame } from '$lib/stores/game'
  import { api, type Weapon } from '$lib/api'
  import ItemIcon from '$lib/components/item-icon.svelte'
  import { elementColor, sharpnessValues, SHARP_COLORS_ARR as SHARP_COLORS } from '$lib/utils/mh'

  const game = $derived($selectedGame)
  const dbId = $derived(game?.dbId)

  let weapons = $state<Weapon[]>([])
  let loading = $state(true)
  let error = $state<string | null>(null)
  let typeFilter = $state<string>('all')
  let sortBy = $state<string>('smith') // smith = armorer tree order (weapon_type -> id) faithful to ISO

  async function loadWeapons(id: number, attempt = 0) {
    try {
      const data = await api.getWeapons(id)
      console.log('[weapons] loaded', data.length)
      weapons = data
      error = null
    } catch (e) {
      const msg = String(e)
      console.error('[weapons] failed', msg)
      if (msg.includes('state not managed') && attempt < 6) {
        error = 'Preparing database...'
        setTimeout(() => loadWeapons(id, attempt + 1), 400 * (attempt + 1))
        return
      }
      error = msg
    } finally {
      if (error !== 'Preparing database...') loading = false
    }
  }

  $effect(() => {
    if (dbId == null) return
    console.log('[weapons] loading gameId', dbId)
    loading = true
    error = null
    loadWeapons(dbId)
  })

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
  const weaponTypes = $derived([
    'all',
    ...Array.from(new Set(weapons.map((w) => w.weapon_type))).sort(
      (a, b) => weaponOrder(a) - weaponOrder(b),
    ),
  ])
  const filtered = $derived.by(() => {
    let arr = typeFilter === 'all' ? weapons : weapons.filter((w) => w.weapon_type === typeFilter)
    if (sortBy === 'name') arr = [...arr].sort((a, b) => a.name.localeCompare(b.name))
    else if (sortBy === 'rarity') arr = [...arr].sort((a, b) => (b.rarity ?? 0) - (a.rarity ?? 0))
    else if (sortBy === 'attack') arr = [...arr].sort((a, b) => (b.attack ?? 0) - (a.attack ?? 0))
    // smith is default already ORDER BY game weapon order, id from queries.rs:699
    return arr
  })

  interface TreeNode {
    weapon: Weapon
    children: TreeNode[]
  }

  function buildForest(typeWeapons: Weapon[]): TreeNode[] {
    const set = new Set(typeWeapons.map((w) => w.name))
    const childrenOf = new Map<string, Weapon[]>()
    for (const w of typeWeapons) {
      if (!w.upgrade_path) continue
      const arr = childrenOf.get(w.upgrade_path) ?? []
      arr.push(w)
      childrenOf.set(w.upgrade_path, arr)
    }
    const roots = typeWeapons.filter((w) => !w.upgrade_path || !set.has(w.upgrade_path))
    const sortFn = (a: Weapon, b: Weapon) => {
      if (sortBy === 'name') return a.name.localeCompare(b.name)
      if (sortBy === 'rarity') return (b.rarity ?? 0) - (a.rarity ?? 0)
      if (sortBy === 'attack') return (b.attack ?? 0) - (a.attack ?? 0)
      // smith: use the in-game armor-forge order (sort_order) when present,
      // otherwise fall back to creation order (id) — faithful to the ISO tree.
      return (a.sort_order ?? a.id) - (b.sort_order ?? b.id)
    }
    const build = (w: Weapon): TreeNode => ({
      weapon: w,
      children: (childrenOf.get(w.name) ?? []).sort(sortFn).map(build),
    })
    return roots.sort(sortFn).map(build)
  }

  const tree = $derived.by<{ type: string; forests: TreeNode[] }[]>(() => {
    const byType = new Map<string, Weapon[]>()
    for (const w of filtered) {
      const arr = byType.get(w.weapon_type) ?? []
      arr.push(w)
      byType.set(w.weapon_type, arr)
    }
    return [...byType.entries()]
      .sort((a, b) => weaponOrder(a[0]) - weaponOrder(b[0]))
      .map(([type, ws]) => ({ type, forests: buildForest(ws) }))
  })

  const _allCount = $derived(tree.reduce((n, t) => n + _countForests(t.forests), 0))
  function _countForests(f: TreeNode[]): number {
    return f.reduce((n, node) => n + 1 + _countForests(node.children), 0)
  }

  function open(id: number) {
    if (!game) return
    goto(`/${game.id}/weapons/${id}`)
  }

  // Helpers now from $lib/utils/mh (DRY)
  const sharpnessSegments = sharpnessValues
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-6">
    <h1 class="text-2xl font-bold text-gray-100">Weapon Trees</h1>
    <p class="text-sm text-gray-500 mt-1">
      {#if game}
        {game.shortName} · {weapons.length} weapons · craft + upgrade tree
      {:else}
        Select a game first
      {/if}
    </p>
  </div>

  {#if loading}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Loading weapons...</p>
    </div>
  {:else if error}
    <div class="bg-red-950/30 border border-red-900 rounded-lg p-8 text-center">
      <p class="text-red-400">Failed to load weapons</p>
      <p class="text-gray-500 text-sm mt-2">{error}</p>
    </div>
  {:else if weapons.length === 0}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">No weapons found for {game?.shortName ?? 'this game'}</p>
    </div>
  {:else}
    <div class="flex flex-wrap gap-2 mb-6 items-center">
      <select
        bind:value={sortBy}
        class="px-3 py-1.5 text-xs bg-[var(--theme-bg-surface)] border border-[var(--theme-border)] rounded-full text-gray-300 focus:outline-none"
      >
        <option value="smith">Smith (Game Order)</option>
        <option value="name">Name A-Z</option>
        <option value="rarity">Rarity ↓</option>
        <option value="attack">Attack ↓</option>
      </select>
      {#each weaponTypes as type}
        <button
          onclick={() => (typeFilter = type)}
          class="px-3 py-1.5 text-xs rounded-full border transition-colors"
          style={typeFilter === type
            ? `background-color: color-mix(in oklab, var(--theme-accent) 12%, transparent); border-color: color-mix(in oklab, var(--theme-accent) 50%, transparent); color: var(--theme-accent);`
            : `background-color: var(--theme-bg-surface); border-color: var(--theme-border); color: rgb(156 163 175);`}
        >
          {type === 'all' ? 'All' : type}
        </button>
      {/each}
    </div>

    <div class="overflow-x-auto -mx-2 px-2">
      {#each tree as group}
        {@const groupIcon = group.forests[0]?.weapon}
        <section class="mb-10 min-w-[320px]">
          <h2 class="text-sm font-semibold uppercase tracking-wider text-gray-400 mb-4 flex items-center gap-2">
            {#if groupIcon}
              <ItemIcon
                iconUrl={groupIcon.icon_url}
                iconName={groupIcon.icon_name}
                iconColor={groupIcon.icon_color}
                size={20}
                alt={group.type}
              />
            {/if}
            {group.type}
          </h2>
          {#each group.forests as node}
            {@render treeNode(node, 0)}
          {/each}
        </section>
      {/each}
    </div>
  {/if}
</div>

{#snippet treeNode(node: TreeNode, depth: number)}
  <div class="mb-1.5 min-w-0" style="margin-left: {depth * 18}px">
    <div class="flex items-center gap-2">
      <button
        onclick={() => open(node.weapon.id)}
        class="flex-1 text-left px-3 py-2 rounded-lg border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] hover:border-[var(--theme-border-strong)] hover:bg-[var(--theme-bg-elevated)] transition-all"
      >
        <div class="flex items-center gap-2 min-w-0">
          <ItemIcon
            iconUrl={node.weapon.icon_url}
            iconName={node.weapon.icon_name}
            iconColor={node.weapon.icon_color}
            size={22}
            alt={node.weapon.weapon_type}
          />
          {#if node.weapon.is_forgeable}
            <span class="text-[12px] shrink-0" title="Crafted directly from materials">🛠️</span>
          {/if}
          <span
            class="text-[10px] text-gray-500 shrink-0 w-10 text-center rounded bg-[var(--theme-bg-elevated)] py-0.5 border border-[var(--theme-border)]"
            >R{node.weapon.rarity ?? 1}</span
          >
          <span class="text-sm text-gray-100 font-medium truncate">{node.weapon.name}</span>
          {#if node.weapon.element_type}
            <span class="text-[11px] {elementColor(node.weapon.element_type)} shrink-0"
              >{node.weapon.element_type} {node.weapon.element_value ?? 0}</span
            >
          {/if}
          <span class="text-[11px] text-gray-500 ml-auto shrink-0"
            >ATK {node.weapon.attack ?? 0}</span
          >
        </div>
        {#if sharpnessSegments(node.weapon.sharpness).length > 0}
          <div class="flex items-center gap-[1px] mt-1.5 h-1.5">
            {#each sharpnessSegments(node.weapon.sharpness) as seg, i}
              {#if seg > 0}
                <div
                  class="rounded-[1px]"
                  style="height: 6px; width: {seg}px; background: {SHARP_COLORS[i] ?? '#666'};"
                ></div>
              {/if}
            {/each}
          </div>
        {/if}
      </button>
    </div>
    {#if node.children.length > 0}
      <div class="border-l border-[var(--theme-border)] ml-4 mt-1.5 pl-2">
        {#each node.children as child}
          {@render treeNode(child, 0)}
        {/each}
      </div>
    {/if}
  </div>
{/snippet}
