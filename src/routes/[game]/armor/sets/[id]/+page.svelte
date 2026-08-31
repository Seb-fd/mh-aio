<script lang="ts">
  import { page } from '$app/state'
  import { api, type ArmorSetDetail } from '$lib/api'
  import DetailHeader from '$lib/components/detail-header.svelte'
  import { goto } from '$app/navigation'
  import { selectedGame } from '$lib/stores/game'
  import ItemIcon from '$lib/components/item-icon.svelte'

  const setId = $derived(Number(page.params.id))
  let set = $state<ArmorSetDetail | null>(null)
  let loading = $state(true)
  let error = $state<string | null>(null)

  $effect(() => {
    if (!setId || Number.isNaN(setId)) return
    loading = true
    error = null
    api
      .getArmorSetDetail(setId)
      .then((data) => {
        set = data
      })
      .catch((e) => {
        error = String(e)
      })
      .finally(() => {
        loading = false
      })
  })

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
  }

  function openPiece(id: number) {
    if (!$selectedGame) return
    goto(`/${$selectedGame.id}/armor/${id}`)
  }
</script>

<div class="max-w-5xl mx-auto">
  {#if loading}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Loading set...</p>
    </div>
  {:else if error}
    <div class="bg-red-950/30 border border-red-900 rounded-lg p-8 text-center">
      <p class="text-red-400">Failed to load set</p>
      <p class="text-gray-500 text-sm mt-2">{error}</p>
    </div>
  {:else if !set}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Set not found</p>
    </div>
  {:else}
    <DetailHeader
      title={set.name}
      subtitle="{set.pieces.length} pieces · {set.pieces[0]?.rank ?? ''}"
      icon="🛡️"
      tags={[
        {
          label: `${set.pieces.length} pcs`,
          color: 'bg-[var(--theme-bg-elevated)] text-gray-300 border-[var(--theme-border)]',
        },
        {
          label: set.pieces[0]?.rank ?? '',
          color: rankColor[set.pieces[0]?.rank ?? 'Low'] ?? 'bg-gray-800 text-gray-400',
        },
      ]}
    />

    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
      {#each set.pieces as piece (piece.id)}
        <button onclick={() => openPiece(piece.id)} class="text-left">
          <div
            class="rounded-lg border themed-card p-4 hover:border-[var(--theme-border-strong)] transition-colors"
          >
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
              {slotLabel[piece.slot_type] ?? piece.slot_type} · {piece.armor_type ?? 'both'} · R{piece.rarity ??
                1}
            </p>
            <div class="grid grid-cols-2 gap-x-3 gap-y-1 text-xs">
              <div>
                <span class="text-gray-500">DEF</span><span class="text-gray-100 font-medium ml-1"
                  >{piece.defense_base ?? 0}-{piece.defense_max ?? 0}</span
                >
              </div>
              <div>
                <span class="text-gray-500">Slots</span><span class="text-gray-100 font-medium ml-1"
                  >{piece.slots ?? '0'}</span
                >
              </div>
              {#if piece.skills}
                <div class="col-span-2 mt-1">
                  <span class="text-gray-500">Skills</span>
                  <span class="text-gray-100 ml-1 text-[11px]">{piece.skills}</span>
                </div>
              {/if}
            </div>
          </div>
        </button>
      {/each}
    </div>
  {/if}
</div>
