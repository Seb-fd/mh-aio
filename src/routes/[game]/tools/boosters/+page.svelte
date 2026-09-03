<script lang="ts">
  import { goto } from '$app/navigation'
  import { selectedGame } from '$lib/stores/game'
  import { api, type MhwMantle } from '$lib/api'
  import Card from '$lib/components/ui/card.svelte'
  import ItemIcon from '$lib/components/item-icon.svelte'
  import { normKey } from '$lib/utils/norm'
  const game = $derived($selectedGame)
  const dbId = $derived(game?.dbId)
  let list = $state<MhwMantle[]>([])
  let loading = $state(true)
  let error = $state<string | null>(null)
  let search = $state('')
  async function load(id: number) {
    try {
      const all = await api.getMhwMantles(id)
      list = all.filter((m) => m.tool_type === 'booster')
      error = null
    } catch (e) {
      error = String(e)
    } finally {
      loading = false
    }
  }
  $effect(() => {
    if (dbId != null) {
      loading = true
      load(dbId)
    }
  })
  const filtered = $derived(
    list.filter((m) => search === '' || normKey(m.name).includes(normKey(search))),
  )
  function open(id: number) {
    if (game) goto(`/${game.id}/tools/boosters/${id}`)
  }
</script>

<div class="max-w-6xl mx-auto">
  <div class="flex gap-2 mb-4 items-center">
    <input
      bind:value={search}
      placeholder="Search boosters..."
      class="px-3 py-1.5 text-sm bg-[var(--theme-bg-surface)] border border-[var(--theme-border)] rounded-lg text-gray-100 placeholder-gray-600 focus:outline-none"
    />
    <span class="text-xs text-gray-500">{filtered.length} / {list.length} boosters</span>
  </div>
  {#if loading}<div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Loading boosters...</p>
    </div>
  {:else if error}<div class="bg-red-950/30 border border-red-900 rounded-lg p-8 text-center">
      <p class="text-red-400">{error}</p>
    </div>
  {:else}
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
      {#each filtered as m (m.id)}
        <button onclick={() => open(m.id)} class="text-left">
          <Card class="p-4 border themed-card hover:scale-[1.01] transition-all">
            <div class="flex items-start gap-3">
              <ItemIcon
                iconUrl={m.icon_url}
                iconName={m.icon_name}
                iconColor={m.icon_color}
                size={36}
                alt={m.name}
              />
              <div class="min-w-0 flex-1">
                <p class="font-semibold text-gray-100">{m.name}</p>
                <p class="text-[11px] text-gray-500 mt-1 line-clamp-2">{m.effect}</p>
                <p class="text-[11px] mt-1 text-gray-400">
                  ⏱ {m.duration_sec}s · ↻ {m.cooldown_sec}s → {m.cooldown_upgraded_sec}s (+)
                </p>
                {#if m.upgrade_quest}<p class="text-[10px] text-emerald-400/80 mt-1 truncate">
                    ★ {m.upgrade_quest}
                  </p>{/if}
              </div>
            </div>
          </Card>
        </button>
      {/each}
    </div>
  {/if}
</div>
