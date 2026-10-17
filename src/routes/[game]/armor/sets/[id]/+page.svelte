<script lang="ts">
  import { page } from '$app/state'
  import ErrorState from '$lib/components/ui/error-state.svelte'
  import { api, type ArmorSetDetail } from '$lib/api'
  import DetailHeader from '$lib/components/detail-header.svelte'
  import Skeleton from '$lib/components/ui/skeleton.svelte'
  import EmptyState from '$lib/components/ui/empty-state.svelte'
  import Badge from '$lib/components/ui/badge.svelte'
  import { rankTone, formatBonus } from '$lib/utils/mh'
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

  function openPiece(id: number) {
    if (!$selectedGame) return
    goto(`/${$selectedGame.id}/armor/${id}`)
  }

  function openSkill(id: number) {
    if (!$selectedGame) return
    goto(`/${$selectedGame.id}/skills/${id}`)
  }

  const bonus1 = $derived(set ? formatBonus(set.bonus_skill, set.bonus_required) : null)
  const bonus2 = $derived(set ? formatBonus(set.bonus_skill2, set.bonus_required2) : null)
  const hasBonus = $derived(bonus1 != null || bonus2 != null)
</script>

<div class="max-w-5xl mx-auto numbered-sections">
  {#if loading}
    <div class="space-y-3" aria-busy="true">
      <Skeleton lines={2} />
      <Skeleton lines={3} />
    </div>
  {:else if error}
    <ErrorState title="Failed to load set" {error} />
  {:else if !set}
    <EmptyState title="Set not found" hint="It may belong to another game." />
  {:else}
    <DetailHeader
      title={set.name}
      subtitle="{set.pieces.length} pieces · {set.pieces[0]?.rank ?? ''}"
      favKind="armor_set"
      favId={set.id}
      tags={[
        {
          label: `${set.pieces.length} pcs`,
          color: 'bg-[var(--theme-bg-elevated)] text-gray-300 border-[var(--theme-border)]',
        },
        {
          label: set.pieces[0]?.rank ?? '',
          tone: rankTone(set.pieces[0]?.rank),
        },
      ]}
    />

    {#if hasBonus}
      <section class="mb-6">
        <h2 class="section-title mb-3">Set Bonus</h2>
        <div class="rounded-lg border themed-card p-4 space-y-2">
          {#if bonus1}
            <div class="flex flex-wrap items-center gap-2 text-sm">
              {#if set.bonus_skill_id != null}
                {@const skillId = set.bonus_skill_id}
                <button
                  onclick={() => openSkill(skillId)}
                  class="font-semibold text-gray-100 hover:text-[var(--theme-text-accent)] transition-colors"
                >
                  {set.bonus_skill}
                </button>
              {:else}
                <span class="font-semibold text-gray-100">{set.bonus_skill}</span>
              {/if}
              <span class="text-xs text-gray-400">— {set.bonus_required} pieces</span>
            </div>
          {/if}
          {#if bonus2}
            <div class="flex flex-wrap items-center gap-2 text-sm">
              {#if set.bonus_skill2_id != null}
                {@const skill2Id = set.bonus_skill2_id}
                <button
                  onclick={() => openSkill(skill2Id)}
                  class="font-semibold text-gray-100 hover:text-[var(--theme-text-accent)] transition-colors"
                >
                  {set.bonus_skill2}
                </button>
              {:else}
                <span class="font-semibold text-gray-100">{set.bonus_skill2}</span>
              {/if}
              <span class="text-xs text-gray-400">— {set.bonus_required2} pieces</span>
            </div>
          {/if}
        </div>
      </section>
    {/if}

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
              <Badge tone={rankTone(piece.rank)}>
                {piece.rank}
              </Badge>
            </div>
            <p class="text-xs text-gray-400 mb-3">
              {slotLabel[piece.slot_type] ?? piece.slot_type} · {piece.armor_type ?? 'both'} · R{piece.rarity ??
                1}
            </p>
            <div class="grid grid-cols-2 gap-x-3 gap-y-1 text-xs">
              <div>
                <span class="text-gray-400">DEF</span><span class="text-gray-100 font-medium ml-1"
                  >{piece.defense_base ?? 0}-{piece.defense_max ?? 0}</span
                >
              </div>
              <div>
                <span class="text-gray-400">Slots</span><span class="text-gray-100 font-medium ml-1"
                  >{piece.slots ?? '0'}</span
                >
              </div>
              {#if piece.skills}
                <div class="col-span-2 mt-1">
                  <span class="text-gray-400">Skills</span>
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
