<script lang="ts">
  import { goto } from '$app/navigation'
  import { selectedGame } from '$lib/stores/game'
  import { api, type Item } from '$lib/api'
  import { normKey } from '$lib/utils/norm'
  import Card from '$lib/components/ui/card.svelte'
  import ItemIcon from '$lib/components/item-icon.svelte'

  const game = $derived($selectedGame)
  const dbId = $derived(game?.dbId)

  let items = $state<Item[]>([])
  let loading = $state(true)
  let error = $state<string | null>(null)
  let categoryFilter = $state<string>('all')
  let searchTerm = $state('')
  let sortBy = $state<string>('chest') // chest = game box (id) faithful to ISO DATA.BIN file 15

  async function loadItems(id: number, attempt = 0) {
    try {
      const data = await api.getItems(id)
      items = data
      error = null
    } catch (e) {
      const msg = String(e)
      if (msg.includes('state not managed') && attempt < 6) {
        error = 'Preparing database...'
        setTimeout(() => loadItems(id, attempt + 1), 400 * (attempt + 1))
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
    loadItems(dbId)
  })

  const categories = $derived([
    'all',
    ...Array.from(new Set(items.map((i) => i.category).filter((c): c is string => !!c))),
  ])
  const filtered = $derived.by(() => {
    let arr = items
      .filter((i) => categoryFilter === 'all' || i.category === categoryFilter)
      .filter((i) => searchTerm === '' || normKey(i.name).includes(normKey(searchTerm)))
    // Sorting: chest is already id order from DB, keep stable; other sorts client-side
    if (sortBy === 'name') arr = [...arr].sort((a, b) => a.name.localeCompare(b.name))
    else if (sortBy === 'rarity') arr = [...arr].sort((a, b) => (b.rarity ?? 0) - (a.rarity ?? 0))
    else if (sortBy === 'price')
      arr = [...arr].sort((a, b) => (b.sell_price ?? 0) - (a.sell_price ?? 0))
    else if (sortBy === 'category')
      arr = [...arr].sort(
        (a, b) =>
          (a.category ?? '').localeCompare(b.category ?? '') || a.name.localeCompare(b.name),
      )
    // chest (id) is default, no sort needed - already ORDER BY id from queries.rs:1023
    return arr
  })

  function open(id: number) {
    if (!game) return
    goto(`/${game.id}/items/${id}`)
  }

  const _categoryColor: Record<string, string> = {
    Consumable: 'bg-emerald-900/40 text-emerald-300',
    Material: 'bg-purple-900/40 text-purple-300',
    Ammo: 'bg-orange-900/40 text-orange-300',
  }
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-6">
    <div class="flex items-center gap-3">
      <h1 class="text-2xl font-bold text-gray-100">Items</h1>
      <button
        onclick={() => {
          if (game) goto(`/${game.id}/items/combine`)
        }}
        class="ml-2 text-xs px-3 py-1 rounded-full border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] text-gray-300 hover:border-[var(--theme-accent)] hover:text-[var(--theme-accent)] transition-colors"
      >
        🧪 Combinations →
      </button>
    </div>
    <p class="text-sm text-gray-500 mt-1">
      {#if game}
        {game.shortName} · {items.length} items · Materials, consumables and locations
      {:else}
        Select a game first
      {/if}
    </p>
  </div>

  {#if loading}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Loading items...</p>
    </div>
  {:else if error}
    <div class="bg-red-950/30 border border-red-900 rounded-lg p-8 text-center">
      <p class="text-red-400">Failed to load items</p>
      <p class="text-gray-500 text-sm mt-2">{error}</p>
    </div>
  {:else if items.length === 0}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">No items found for {game?.shortName ?? 'this game'}</p>
    </div>
  {:else}
    <div class="flex flex-wrap gap-2 mb-4 items-center">
      <input
        type="text"
        bind:value={searchTerm}
        placeholder="Search items..."
        class="px-3 py-1.5 text-sm bg-[var(--theme-bg-surface)] border border-[var(--theme-border)] rounded-lg text-gray-100 placeholder-gray-600 focus:outline-none focus:border-[var(--theme-border-strong)]"
      />
      <select
        bind:value={sortBy}
        class="px-3 py-1.5 text-xs bg-[var(--theme-bg-surface)] border border-[var(--theme-border)] rounded-full text-gray-300 focus:outline-none"
      >
        <option value="chest">Chest (Game Order)</option>
        <option value="name">Name A-Z</option>
        <option value="rarity">Rarity ↓</option>
        <option value="price">Sell Price ↓</option>
        <option value="category">Category</option>
      </select>
      {#each categories as cat}
        <button
          onclick={() => (categoryFilter = cat)}
          class="px-3 py-1.5 text-xs rounded-full border transition-colors"
          style={categoryFilter === cat
            ? `background-color: color-mix(in oklab, var(--theme-accent) 12%, transparent); border-color: color-mix(in oklab, var(--theme-accent) 50%, transparent); color: var(--theme-accent);`
            : `background-color: var(--theme-bg-surface); border-color: var(--theme-border); color: rgb(156 163 175);`}
        >
          {cat === 'all' ? 'All' : cat}
        </button>
      {/each}
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2">
      {#each filtered as item}
        <button onclick={() => open(item.id)} class="text-left">
          <Card class="p-3 border transition-all cursor-pointer themed-card">
            <div class="flex items-center gap-3">
              <ItemIcon
                iconUrl={item.icon_url}
                iconName={item.icon_name}
                iconColor={item.icon_color}
                size={36}
                alt={item.name}
              />
              <div class="min-w-0 flex-1">
                <p class="font-medium text-sm text-gray-100 truncate">{item.name}</p>
                <p class="text-[10px] uppercase tracking-wide text-gray-500 mt-0.5">
                  {item.category ?? 'Unknown'}{item.subcategory ? ` • ${item.subcategory}` : ''} · R{item.rarity ??
                    1}
                  {#if item.carry_limit}
                    · x{item.carry_limit}
                  {/if}
                </p>
              </div>
              {#if item.sell_price !== null && item.sell_price !== undefined}
                <span class="text-xs font-medium shrink-0" style="color: var(--theme-accent);"
                  >{item.sell_price}z</span
                >
              {/if}
            </div>
          </Card>
        </button>
      {/each}
    </div>
  {/if}
</div>
