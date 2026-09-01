<script lang="ts">
  import { goto } from '$app/navigation'
  import { selectedGame } from '$lib/stores/game'
  import { api, type Armor, type ArmorSet } from '$lib/api'
  import Card from '$lib/components/ui/card.svelte'
  import ItemIcon from '$lib/components/item-icon.svelte'

  const game = $derived($selectedGame)
  const dbId = $derived(game?.dbId)

  let armors = $state<Armor[]>([])
  let armorSets = $state<ArmorSet[]>([])
  let loading = $state(true)
  let error = $state<string | null>(null)
  let rankFilter = $state<string>('all')
  let genderFilter = $state<string>('both') // both (show all) | male | female
  let typeFilter = $state<string>('all') // all | blade | gunner
  let sortBy = $state<string>('smith') // smith = armorer list (rank -> slot -> id) faithful to ISO 37652906 string table
  let viewMode = $state<'sets' | 'pieces'>('sets')

  async function loadAll(id: number, attempt = 0) {
    try {
      const [a, s] = await Promise.all([api.getArmor(id), api.getArmorSets(id)])
      armors = a
      armorSets = s
      error = null
    } catch (e) {
      const msg = String(e)
      if (msg.includes('state not managed') && attempt < 6) {
        error = 'Preparing database...'
        setTimeout(() => loadAll(id, attempt + 1), 400 * (attempt + 1))
        return
      }
      error = msg
    } finally {
      if (error !== 'Preparing database...') loading = false
    }
  }

  $effect(() => {
    if (dbId == null) return
    loading = true
    error = null
    loadAll(dbId)
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
      (a) => (rankFilter === 'all' || a.rank === rankFilter) && matchesGender(a) && matchesType(a),
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
    let arr = armorSets.filter((s) => setIds.has(s.id))
    if (sortBy === 'name') arr = [...arr].sort((a, b) => a.name.localeCompare(b.name))
    else if (sortBy === 'rarity') arr = [...arr].sort((a, b) => (b.rarity ?? 0) - (a.rarity ?? 0))
    else if (sortBy === 'defense') arr = [...arr].sort((a, b) => (b.rarity ?? 0) - (a.rarity ?? 0))
    return arr
  })

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

  const rankColor: Record<string, string> = {
    Low: 'bg-gray-700 text-gray-300',
    High: 'bg-blue-900/40 text-blue-300',
    G: 'bg-yellow-900/40 text-yellow-300',
    Master: 'bg-red-900/40 text-red-300 ring-1 ring-red-700/50',
  }

  function setLabel(s: { piece_count: number }): string {
    if (s.piece_count === 1) return 'Singleton — e.g., Black Legs (no full set)'
    if (s.piece_count >= 10) return 'Full set (Blade + Gunner, 10)'
    if (s.piece_count === 5) return 'Full set (5)'
    return `${s.piece_count} pieces`
  }
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-6">
    <h1 class="text-2xl font-bold text-gray-100">Armor</h1>
    <p class="text-sm text-gray-500 mt-1">
      {#if game}
        {game.shortName} · {armors.length} pieces · {armorSets.length} sets
      {:else}
        Select a game first
      {/if}
    </p>
  </div>

  {#if loading}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Loading armor...</p>
    </div>
  {:else if error}
    <div class="bg-red-950/30 border border-red-900 rounded-lg p-8 text-center">
      <p class="text-red-400">Failed to load armor</p>
      <p class="text-gray-500 text-sm mt-2">{error}</p>
    </div>
  {:else if armors.length === 0}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">No armor found for {game?.shortName ?? 'this game'}</p>
    </div>
  {:else}
    <div class="flex flex-wrap gap-2 mb-4 items-center">
      <div class="flex rounded-full border border-[var(--theme-border)] overflow-hidden">
        <button
          onclick={() => (viewMode = 'sets')}
          class="px-4 py-1.5 text-xs font-medium {viewMode === 'sets'
            ? 'bg-[var(--theme-primary)] text-white'
            : 'bg-[var(--theme-bg-surface)] text-gray-400'}">Sets ({armorSets.length})</button
        >
        <button
          onclick={() => (viewMode = 'pieces')}
          class="px-4 py-1.5 text-xs font-medium {viewMode === 'pieces'
            ? 'bg-[var(--theme-primary)] text-white'
            : 'bg-[var(--theme-bg-surface)] text-gray-400'}">Pieces ({filtered.length})</button
        >
      </div>
      <select
        bind:value={sortBy}
        class="px-3 py-1.5 text-xs bg-[var(--theme-bg-surface)] border border-[var(--theme-border)] rounded-full text-gray-300 focus:outline-none"
      >
        <option value="smith">Smith (Game Order)</option>
        <option value="name">Name A-Z</option>
        <option value="rarity">Rarity ↓</option>
        <option value="defense">Defense ↓</option>
        <option value="slots">Slots ↓</option>
      </select>
      <span class="flex rounded-full border border-[var(--theme-border)] overflow-hidden text-xs">
        {#each ['both', 'male', 'female'] as g}
          <button
            onclick={() => (genderFilter = g)}
            class="px-3 py-1.5 transition-colors {genderFilter === g
              ? 'bg-[var(--theme-primary)] text-white'
              : 'bg-[var(--theme-bg-surface)] text-gray-400'}"
          >
            {g === 'both' ? 'Both' : g === 'male' ? 'Male' : 'Female'}
          </button>
        {/each}
      </span>
      <span class="flex rounded-full border border-[var(--theme-border)] overflow-hidden text-xs">
        {#each ['all', 'blade', 'gunner'] as t}
          <button
            onclick={() => (typeFilter = t)}
            class="px-3 py-1.5 transition-colors {typeFilter === t
              ? 'bg-[var(--theme-primary)] text-white'
              : 'bg-[var(--theme-bg-surface)] text-gray-400'}"
          >
            {t === 'all' ? 'All' : t === 'blade' ? 'Blademaster' : 'Gunner'}
          </button>
        {/each}
      </span>
      {#each ranks as rank}
        <button
          onclick={() => (rankFilter = rank)}
          class="px-3 py-1.5 text-xs rounded-full border transition-colors"
          style={rankFilter === rank
            ? `background-color: color-mix(in oklab, var(--theme-accent) 12%, transparent); border-color: color-mix(in oklab, var(--theme-accent) 50%, transparent); color: var(--theme-accent);`
            : `background-color: var(--theme-bg-surface); border-color: var(--theme-border); color: rgb(156 163 175);`}
        >
          {rank === 'all' ? 'All' : rank}
        </button>
      {/each}
    </div>

    {#if viewMode === 'sets'}
      {#if filteredSets.length === 0}
        <div class="border rounded-lg p-8 text-center themed-card">
          <p class="text-gray-400">No sets match current filters</p>
        </div>
      {:else}
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-3">
          {#each filteredSets as set}
            {@const pieces = armors
              .filter((a) => a.set_id === set.id && matchesGender(a) && matchesType(a))
              .slice(0, 6)}
            <button onclick={() => openSet(set.id)} class="text-left">
              <Card
                class="p-4 border themed-card hover:border-[var(--theme-border-strong)] transition-colors"
              >
                <div class="flex items-start justify-between gap-2 mb-2">
                  <h3 class="font-semibold text-gray-100 truncate">{set.name}</h3>
                  <span
                    class="text-[10px] uppercase tracking-wide px-2 py-0.5 rounded shrink-0 {rankColor[
                      set.rank ?? 'Low'
                    ] ?? 'bg-gray-800 text-gray-400'}"
                  >
                    {set.rank ?? 'Low'} · {set.piece_count} pcs
                  </span>
                </div>
                <p class="text-xs text-gray-500 mb-2">
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
                        alt={p.slot_type}
                      />{p.name}
                      <span class="text-gray-500">[{slotLabel[p.slot_type] ?? p.slot_type}]</span
                      ></span
                    >
                  {/each}
                  {#if set.piece_count > pieces.length}
                    <span class="text-[10px] px-2 py-1 rounded bg-gray-800 text-gray-500"
                      >+{set.piece_count - pieces.length} more</span
                    >
                  {/if}
                </div>
              </Card>
            </button>
          {/each}
        </div>
      {/if}
    {:else}
      {#if filtered.length === 0}
        <div class="border rounded-lg p-8 text-center themed-card">
          <p class="text-gray-400">No armor pieces match current filters</p>
        </div>
      {:else}
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
          {#each filtered as piece}
            <button onclick={() => open(piece.id)} class="text-left">
              <Card class="p-4 border transition-all cursor-pointer hover:scale-[1.02] themed-card">
                <div class="flex items-center gap-2 mb-2">
                  <ItemIcon
                    iconUrl={piece.icon_url}
                    iconName={piece.icon_name}
                    iconColor={piece.icon_color}
                    size={28}
                    alt={piece.slot_type}
                  />
                  <h3 class="font-semibold text-gray-100 truncate flex-1">{piece.name}</h3>
                  <span
                    class="text-[10px] uppercase tracking-wide px-2 py-0.5 rounded shrink-0 {rankColor[
                      piece.rank
                    ] ?? 'bg-gray-800 text-gray-400'}"
                  >
                    {piece.rank}
                  </span>
                </div>
                <p class="text-xs text-gray-500 mb-3">
                  {slotLabel[piece.slot_type] ?? piece.slot_type}
                </p>
                <div class="grid grid-cols-2 gap-x-3 gap-y-1 text-xs">
                  <div>
                    <span class="text-gray-500">DEF</span>
                    <span class="text-gray-100 font-medium ml-1">
                      {piece.defense_base ?? 0}-{piece.defense_max ?? 0}
                    </span>
                  </div>
                  <div>
                    <span class="text-gray-500">Rarity</span>
                    <span class="text-gray-100 font-medium ml-1">{piece.rarity ?? 1}</span>
                  </div>
                  <div>
                    <span class="text-orange-400">Fire</span>
                    <span class="text-gray-100 ml-1">{piece.resistance_fire ?? 0}</span>
                  </div>
                  <div>
                    <span class="text-blue-400">Water</span>
                    <span class="text-gray-100 ml-1">{piece.resistance_water ?? 0}</span>
                  </div>
                  <div>
                    <span class="text-yellow-400">Thunder</span>
                    <span class="text-gray-100 ml-1">{piece.resistance_thunder ?? 0}</span>
                  </div>
                  <div>
                    <span class="text-cyan-400">Ice</span>
                    <span class="text-gray-100 ml-1">{piece.resistance_ice ?? 0}</span>
                  </div>
                  <div class="col-span-2">
                    <span class="text-purple-400">Dragon</span>
                    <span class="text-gray-100 ml-1">{piece.resistance_dragon ?? 0}</span>
                  </div>
                </div>
              </Card>
            </button>
          {/each}
        </div>
      {/if}
    {/if}
  {/if}
</div>
