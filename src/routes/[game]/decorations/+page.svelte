<script lang="ts">
  import { goto } from '$app/navigation'
  import { selectedGame } from '$lib/stores/game'
  import { api, type Decoration } from '$lib/api'
  import { normKey } from '$lib/utils/norm'
  import Card from '$lib/components/ui/card.svelte'

  const game = $derived($selectedGame)
  const dbId = $derived(game?.dbId)

  let decorations = $state<Decoration[]>([])
  let loading = $state(true)
  let error = $state<string | null>(null)
  let searchTerm = $state('')
  let slotFilter = $state<string>('all')
  let skillFilter = $state<string>('all')

  async function loadDecorationsData(id: number, attempt = 0) {
    try {
      const data = await api.getDecorations(id)
      decorations = data
      error = null
    } catch (e) {
      const msg = String(e)
      if (msg.includes('state not managed') && attempt < 6) {
        error = 'Preparing database...'
        setTimeout(() => loadDecorationsData(id, attempt + 1), 400 * (attempt + 1))
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
    loadDecorationsData(dbId)
  })

  const skills = $derived([
    'all',
    ...Array.from(
      new Set(
        decorations.flatMap((d) =>
          [d.skill_name, d.secondary_skill_name].filter((x): x is string => !!x),
        ),
      ),
    ).sort(),
  ])
  const slots = ['all', '1', '2', '3']

  const filtered = $derived(
    decorations
      .filter((d) => slotFilter === 'all' || String(d.slot_size) === slotFilter)
      .filter(
        (d) =>
          skillFilter === 'all' ||
          d.skill_name === skillFilter ||
          d.secondary_skill_name === skillFilter,
      )
      .filter((d) => searchTerm === '' || normKey(d.name).includes(normKey(searchTerm))),
  )

  function open(id: number) {
    if (!game) return
    goto(`/${game.id}/decorations/${id}`)
  }
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-6">
    <h1 class="text-2xl font-bold text-gray-100">Decorations</h1>
    <p class="text-sm text-gray-500 mt-1">
      {#if game}
        {game.shortName} · {decorations.length} jewels · Crafted at Smith · 100% faithful to MHFU
      {:else}
        Select a game first
      {/if}
    </p>
  </div>

  {#if loading}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Loading decorations...</p>
    </div>
  {:else if error}
    <div class="bg-red-950/30 border border-red-900 rounded-lg p-8 text-center">
      <p class="text-red-400">Failed to load decorations</p>
      <p class="text-gray-500 text-sm mt-2">{error}</p>
    </div>
  {:else if decorations.length === 0}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">No decorations found for {game?.shortName ?? 'this game'}</p>
    </div>
  {:else}
    <div class="flex flex-wrap gap-2 mb-4">
      <input
        type="text"
        bind:value={searchTerm}
        placeholder="Search jewels..."
        class="px-3 py-1.5 text-sm bg-[var(--theme-bg-surface)] border border-[var(--theme-border)] rounded-lg text-gray-100 placeholder-gray-600 focus:outline-none focus:border-[var(--theme-border-strong)]"
      />
      {#each slots as s}
        <button
          onclick={() => (slotFilter = s)}
          class="px-3 py-1.5 text-xs rounded-full border transition-colors"
          style={slotFilter === s
            ? `background-color: color-mix(in oklab, var(--theme-accent) 12%, transparent); border-color: color-mix(in oklab, var(--theme-accent) 50%, transparent); color: var(--theme-accent);`
            : `background-color: var(--theme-bg-surface); border-color: var(--theme-border); color: rgb(156 163 175);`}
        >
          {s === 'all' ? 'All Slots' : `Slot ${s}`}
        </button>
      {/each}
      <select
        bind:value={skillFilter}
        class="px-3 py-1.5 text-xs bg-[var(--theme-bg-surface)] border border-[var(--theme-border)] rounded-full text-gray-300 focus:outline-none focus:border-[var(--theme-border-strong)]"
      >
        {#each skills as sk}
          <option value={sk}>{sk === 'all' ? 'All Skills' : sk}</option>
        {/each}
      </select>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2">
      {#each filtered as deco}
        <button onclick={() => open(deco.id)} class="text-left">
          <Card class="p-3 border transition-all cursor-pointer themed-card h-full">
            <div class="flex items-start justify-between gap-2">
              <div class="min-w-0">
                <p class="font-medium text-sm text-gray-100 truncate">{deco.name}</p>
                <p class="text-[11px] text-gray-500 mt-0.5">
                  {#if deco.skill_name}
                    <span
                      class={(deco.skill_points ?? 0) >= 0 ? 'text-emerald-400' : 'text-red-400'}
                      >{deco.skill_name}
                      {(deco.skill_points ?? 0) > 0 ? '+' : ''}{deco.skill_points}</span
                    >
                  {/if}
                  {#if deco.secondary_skill_name}
                    <span class="text-gray-600"> · </span>
                    <span
                      class={(deco.secondary_points ?? 0) >= 0
                        ? 'text-emerald-400'
                        : 'text-red-400'}
                      >{deco.secondary_skill_name}
                      {(deco.secondary_points ?? 0) > 0 ? '+' : ''}{deco.secondary_points}</span
                    >
                  {/if}
                </p>
              </div>
              <span
                class="inline-flex items-center justify-center w-6 h-6 rounded text-xs font-bold border shrink-0
                {deco.slot_size === 1
                  ? 'bg-gray-800 text-gray-300 border-gray-700'
                  : deco.slot_size === 2
                    ? 'bg-blue-900/30 text-blue-300 border-blue-800'
                    : 'bg-yellow-900/30 text-yellow-300 border-yellow-800'}"
              >
                {deco.slot_size ?? '-'}
              </span>
            </div>
            <div class="flex items-center justify-between mt-2">
              <span
                class="text-[10px] px-1.5 py-0.5 rounded bg-[var(--theme-bg-elevated)] text-gray-400 border border-[var(--theme-border)]"
                >Slot {deco.slot_size}</span
              >
              <span class="text-xs font-medium" style="color: var(--theme-accent);"
                >{deco.price ?? 0}z</span
              >
            </div>
          </Card>
        </button>
      {/each}
    </div>
    {#if filtered.length === 0}
      <p class="text-center text-gray-500 text-sm mt-6">No jewels match filters.</p>
    {/if}
  {/if}
</div>
