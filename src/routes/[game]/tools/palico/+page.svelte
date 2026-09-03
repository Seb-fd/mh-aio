<script lang="ts">
  import { goto } from '$app/navigation'
  import { selectedGame } from '$lib/stores/game'
  import { api, type PalicoGadget } from '$lib/api'
  import Card from '$lib/components/ui/card.svelte'
  import ItemIcon from '$lib/components/item-icon.svelte'
  import { normKey } from '$lib/utils/norm'

  const game = $derived($selectedGame)
  const dbId = $derived(game?.dbId)
  let gadgets = $state<PalicoGadget[]>([])
  let loading = $state(true)
  let error = $state<string | null>(null)
  let subtab = $state<'gadget' | 'tailraider' | 'safari'>('gadget')
  let search = $state('')

  async function load(id: number) {
    try {
      gadgets = await api.getPalicoGadgets(id)
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
  <div class="flex gap-2 mb-4">
    <button
      onclick={() => (subtab = 'gadget')}
      class="px-3 py-1.5 rounded-full text-xs font-medium border {subtab === 'gadget'
        ? 'bg-[var(--theme-primary)] text-white border-transparent'
        : 'bg-[var(--theme-bg-surface)] text-gray-400 border-[var(--theme-border)]'}"
      >Gadgets (6)</button
    >
    <button
      onclick={() => (subtab = 'tailraider')}
      class="px-3 py-1.5 rounded-full text-xs font-medium border {subtab === 'tailraider'
        ? 'bg-[var(--theme-primary)] text-white border-transparent'
        : 'bg-[var(--theme-bg-surface)] text-gray-400 border-[var(--theme-border)]'}"
      >Tailraider Signal</button
    >
    <button
      onclick={() => (subtab = 'safari')}
      class="px-3 py-1.5 rounded-full text-xs font-medium border {subtab === 'safari'
        ? 'bg-[var(--theme-primary)] text-white border-transparent'
        : 'bg-[var(--theme-bg-surface)] text-gray-400 border-[var(--theme-border)]'}">Safari</button
    >
    <input
      bind:value={search}
      placeholder="Search..."
      class="ml-auto px-3 py-1.5 text-sm bg-[var(--theme-bg-surface)] border border-[var(--theme-border)] rounded-lg text-gray-100 placeholder-gray-600 focus:outline-none"
    />
  </div>

  {#if loading}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Loading Palico tools...</p>
    </div>
  {:else if error}
    <div class="bg-red-950/30 border border-red-900 rounded-lg p-8 text-center">
      <p class="text-red-400">{error}</p>
    </div>
  {:else if filtered.length === 0}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">No results for {subtab}</p>
    </div>
  {:else}
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
      {#each filtered as g (g.id)}
        <button onclick={() => open(g.id)} class="text-left">
          <Card class="p-4 border themed-card hover:scale-[1.01] transition-all">
            <div class="flex items-start gap-3">
              <ItemIcon
                iconUrl={g.icon_url}
                iconName={g.icon_name}
                iconColor={g.icon_color}
                size={36}
                alt={g.name}
              />
              <div class="min-w-0 flex-1">
                <p class="font-semibold text-gray-100">{g.name}</p>
                {#if g.tribe}<p class="text-[11px] text-[var(--theme-accent)] mt-0.5">
                    {g.tribe}
                  </p>{/if}
                <p class="text-[11px] text-gray-500 mt-1 line-clamp-2">
                  {g.effect ?? g.description}
                </p>
                {#if g.acquisition}<p class="text-[10px] text-gray-500 mt-2 line-clamp-2">
                    🔓 {g.acquisition}
                  </p>{/if}
                <span
                  class="inline-block mt-2 text-[10px] px-2 py-0.5 rounded bg-[var(--theme-bg-elevated)] border border-[var(--theme-border)] text-gray-400"
                  >{g.gadget_type}</span
                >
              </div>
            </div>
          </Card>
        </button>
      {/each}
    </div>
  {/if}
</div>
