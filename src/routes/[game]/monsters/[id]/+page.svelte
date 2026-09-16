<script lang="ts">
  import { goto } from '$app/navigation'
  import ErrorState from '$lib/components/ui/error-state.svelte'
  import { page } from '$app/state'
  import { api, type MonsterDetail, type MonsterDrop, type ArmorSetDetail } from '$lib/api'
  import DetailHeader from '$lib/components/detail-header.svelte'
  import Skeleton from '$lib/components/ui/skeleton.svelte'
  import EmptyState from '$lib/components/ui/empty-state.svelte'
  import { selectedGame } from '$lib/stores/game'
  import { fallbackLabel } from '$lib/utils/mh'
  import type { Component } from 'svelte'
  import { Swords, Lock, Sparkles, Hammer, CircleHelp } from '@lucide/svelte'

  const id = $derived(Number(page.params.id))
  let monster = $state<MonsterDetail | null>(null)
  let loading = $state(true)
  let error = $state<string | null>(null)
  // Large portrait (256px masters for Rise/Wilds) with fallback chain:
  // master -> `-sm` variant -> hidden.
  let portraitFailedLg = $state(false)
  let portraitFailedAll = $state(false)
  let dedicatedSets = $state<ArmorSetDetail[]>([])
  let dedicatedLoading = $state(false)
  let armorViewMode = $state<'dedicated' | 'uses'>('dedicated') // dedicated default (60% score), uses secondary

  $effect(() => {
    if (!id || Number.isNaN(id)) return
    loading = true
    error = null
    portraitFailedLg = false
    portraitFailedAll = false
    api
      .getMonsterDetail(id)
      .then((data) => {
        monster = data
      })
      .catch((e) => {
        error = String(e)
      })
      .finally(() => {
        loading = false
      })
  })

  // Fetch dedicated armor sets (60% threshold, rank-filtered) whenever monster + rank changes
  $effect(() => {
    if (!monster || !id) return
    dedicatedLoading = true
    api
      .getMonsterDedicatedSets(id, activeRank)
      .then((data) => {
        dedicatedSets = data
      })
      .catch(() => {
        dedicatedSets = []
      })
      .finally(() => {
        dedicatedLoading = false
      })
  })

  interface MethodMeta {
    label: string
    Icon: Component
    color: string
  }

  const methodLabel: Record<string, MethodMeta> = {
    carve: { label: 'Carve', Icon: Swords, color: 'text-red-400' },
    capture: { label: 'Capture', Icon: Lock, color: 'text-emerald-400' },
    drop: { label: 'Shiny Drop', Icon: Sparkles, color: 'text-yellow-400' },
    break: { label: 'Break Part', Icon: Hammer, color: 'text-orange-400' },
  }
  const fallbackMethod: MethodMeta = { label: '', Icon: CircleHelp, color: 'text-gray-400' }

  const slotLabel: Record<string, string> = {
    head: 'Helm',
    chest: 'Mail',
    arms: 'Vambraces',
    waist: 'Coil',
    legs: 'Greaves',
  }

  const rankOrder = ['Low', 'High', 'G']
  const methodOrder = ['carve', 'break', 'capture', 'drop']
  function goToItem(drop: MonsterDrop) {
    if ($selectedGame) goto(`/${$selectedGame.id}/items/${drop.item_id}`)
  }

  function weaknessColor(value: number | null): string {
    if (value == null) return 'text-gray-600'
    if (value >= 25) return 'text-emerald-400'
    if (value >= 15) return 'text-yellow-400'
    if (value >= 5) return 'text-orange-400'
    if (value <= -10) return 'text-red-400'
    return 'text-gray-400'
  }

  function weaknessBg(value: number | null): string {
    if (value == null) return 'bg-gray-800/30'
    // Top tier pops with the theme ring — best weak point at a glance.
    if (value >= 25) return 'bg-emerald-900/40 font-bold ring-1 ring-[var(--theme-border-strong)]'
    if (value >= 15) return 'bg-yellow-900/30'
    if (value >= 5) return 'bg-orange-900/30'
    if (value <= -10) return 'bg-red-900/40'
    return 'bg-gray-800/40'
  }

  function sortDrops(drops: MonsterDrop[]): MonsterDrop[] {
    return [...drops].sort((a, b) => {
      const ra = rankOrder.indexOf(a.rank ?? '')
      const rb = rankOrder.indexOf(b.rank ?? '')
      if (ra !== rb) return ra - rb
      const ma = methodOrder.indexOf(a.method)
      const mb = methodOrder.indexOf(b.method)
      if (ma !== mb) return ma - mb
      return b.probability - a.probability
    })
  }

  const rankTabs: string[] = $derived(
    sortDrops(monster?.drops ?? []).reduce((acc, d) => {
      const r = d.rank ?? 'Low'
      if (!acc.includes(r)) acc.push(r)
      return acc
    }, [] as string[]),
  )

  let activeRank = $state('Low')

  $effect(() => {
    if (rankTabs.length > 0 && !rankTabs.includes(activeRank)) {
      activeRank = rankTabs[0]
    }
  })

  const visibleDrops = $derived(
    sortDrops((monster?.drops ?? []).filter((d) => (d.rank ?? 'Low') === activeRank)),
  )
</script>

<div class="max-w-5xl mx-auto numbered-sections">
  {#if loading}
    <div class="space-y-3" aria-busy="true">
      <Skeleton lines={2} />
      <Skeleton lines={3} />
    </div>
  {:else if error}
    <ErrorState title="Failed to load monster" {error} />
  {:else if !monster}
    <EmptyState title="Monster not found" hint="It may belong to another game." />
  {:else}
    <DetailHeader
      title={monster.name}
      subtitle={monster.species ?? ''}
      iconUrl={monster.icon_url}
      favKind="monster"
      favId={monster.id}
      tags={[
        {
          label: fallbackLabel(monster.size),
          color: 'bg-[var(--theme-bg-elevated)] text-gray-300 border-[var(--theme-border)]',
        },
        {
          label: 'Monster',
          color:
            'bg-[var(--theme-bg-elevated)] text-[var(--theme-text-accent)] border-[var(--theme-border-strong)]',
        },
      ]}
    />

    {#if !portraitFailedAll && (monster.icon_url_lg ?? monster.icon_url)}
      <div class="mb-8 flex justify-center">
        <div
          class="rounded-2xl border border-[var(--theme-border-strong)] bg-[var(--theme-bg-elevated)] p-3 shadow-lg"
          style="box-shadow: 0 0 30px var(--theme-glow);"
        >
          <img
            src={portraitFailedLg ? monster.icon_url : (monster.icon_url_lg ?? monster.icon_url)}
            alt={monster.name}
            width="160"
            height="160"
            class="h-40 w-40 object-contain"
            loading="lazy"
            decoding="async"
            onerror={() => {
              if (!portraitFailedLg && monster?.icon_url_lg) portraitFailedLg = true
              else portraitFailedAll = true
            }}
          />
        </div>
      </div>
    {/if}

    {#if monster.description}
      <section class="mb-8">
        <h2 class="section-title mb-3">Description</h2>
        <div class="rounded-lg border themed-card p-5 leading-relaxed text-gray-200 text-[15px]">
          {monster.description}
        </div>
      </section>
    {/if}

    {#if monster.drops.length > 0}
      <section class="mb-8">
        <h2 class="section-title mb-3">Material Drops</h2>

        <div class="flex gap-2 mb-4 flex-wrap" role="group" aria-label="Filter drops by rank">
          {#each rankTabs as rank}
            <button
              onclick={() => (activeRank = rank)}
              aria-pressed={rank === activeRank}
              class="px-4 rounded-md text-xs font-medium border transition-colors motion-safe:transition-colors motion-reduce:transition-none min-h-[44px] sm:min-h-[36px] sm:px-3 sm:py-1.5 focus-visible:outline-none focus-visible:ring-2
                {rank === activeRank
                ? 'bg-[var(--theme-primary)] text-[var(--theme-text-on-primary)] border-transparent'
                : 'bg-[var(--theme-bg-elevated)] text-gray-300 border-[var(--theme-border)] hover:border-[var(--theme-border-strong)]'}"
            >
              {rank}
            </button>
          {/each}
        </div>

        <div class="space-y-3">
          {#each visibleDrops as drop, i (drop.method + '|' + drop.item_id + '|' + (drop.part ?? '') + '|' + (drop.rank ?? '') + '|' + i)}
            {@const raw = methodLabel[drop.method]}
            {@const meta: MethodMeta = raw ? { ...raw } : { ...fallbackMethod, label: drop.method }}
            <button
              onclick={() => goToItem(drop)}
              aria-label="{meta.label}: {drop.item_name} x{drop.quantity}, {Math.round(
                drop.probability * 100,
              )} percent"
              class="w-full block px-4 min-h-[52px] py-3 rounded-lg border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] hover:border-[var(--theme-border-strong)] hover:bg-[var(--theme-bg-elevated)] transition-colors motion-safe:transition-colors motion-reduce:transition-none text-left group focus-visible:outline-none focus-visible:ring-2"
            >
              <div class="flex items-center gap-3">
                <meta.Icon class="h-5 w-5 shrink-0 {meta.color}" aria-hidden="true" />
                <div class="flex-1 min-w-0">
                  <div class="flex items-center gap-2">
                    <span class="text-xs uppercase tracking-wide {meta.color} font-medium"
                      >{meta.label}</span
                    >
                    {#if drop.part}
                      <span class="text-xs text-gray-400">· {drop.part}</span>
                    {/if}
                    <span
                      class="text-sm text-gray-100 truncate group-hover:text-[var(--theme-text-accent)] transition-colors"
                    >
                      {drop.item_name}
                      <span class="text-xs text-gray-400"> x{drop.quantity}</span>
                    </span>
                  </div>
                  {#if drop.condition}
                    <p class="text-[11px] text-gray-400 mt-0.5">※ {drop.condition}</p>
                  {/if}
                  <div class="mt-2 flex items-center gap-2">
                    <div class="probability-bar" role="presentation">
                      <span style="--prob: {Math.round(drop.probability * 100)}"></span>
                    </div>
                    <span class="text-[10px] text-gray-400 shrink-0 tabular-nums w-9 text-right">
                      {Math.round(drop.probability * 100)}%
                    </span>
                  </div>
                </div>
              </div>
            </button>
          {/each}
        </div>
      </section>
    {/if}

    {#if monster.armor.length > 0 || monster.weapons.length > 0 || dedicatedSets.length > 0}
      <section class="mb-8">
        <h2 class="section-title mb-3">
          Equipment · Rank filter: {activeRank} (unified with drops)
        </h2>
        <!-- Armor: Dedicated (default, 60% score) vs Uses 1 Material (secondary) -->
        {#if monster.armor.length > 0 || dedicatedSets.length > 0}
          <div class="flex items-center gap-2 mb-2">
            <h3 class="text-sm font-semibold text-gray-200">Armor</h3>
            <div
              class="flex rounded-full border border-[var(--theme-border)] overflow-hidden ml-2"
              role="group"
              aria-label="Armor view"
            >
              <button
                onclick={() => (armorViewMode = 'dedicated')}
                aria-pressed={armorViewMode === 'dedicated'}
                class="px-3 min-h-[44px] sm:min-h-[36px] py-1 text-[11px] font-medium focus-visible:outline-none focus-visible:ring-2 {armorViewMode ===
                'dedicated'
                  ? 'bg-[var(--theme-primary)] text-[var(--theme-text-on-primary)]'
                  : 'bg-[var(--theme-bg-surface)] text-gray-400'}"
                >Dedicated ({dedicatedSets.length})</button
              >
              <button
                onclick={() => (armorViewMode = 'uses')}
                aria-pressed={armorViewMode === 'uses'}
                class="px-3 min-h-[44px] sm:min-h-[36px] py-1 text-[11px] font-medium focus-visible:outline-none focus-visible:ring-2 {armorViewMode ===
                'uses'
                  ? 'bg-[var(--theme-primary)] text-[var(--theme-text-on-primary)]'
                  : 'bg-[var(--theme-bg-surface)] text-gray-400'}"
                >Uses 1 Material ({monster.armor.length})</button
              >
            </div>
          </div>
          {#if armorViewMode === 'dedicated'}
            {#if dedicatedLoading}
              <p class="text-xs text-gray-400 mb-4">
                Loading dedicated sets (≥60% mats, exact monster, rank {activeRank})…
              </p>
            {:else if dedicatedSets.length === 0}
              <p class="text-xs text-gray-400 mb-4">
                No dedicated sets for {monster.name} at rank {activeRank} — try another rank or check
                “Uses 1 Material”.
              </p>
            {:else}
              <div class="grid grid-cols-1 lg:grid-cols-2 gap-3 mb-4">
                {#each dedicatedSets as set, si (set.id + '-' + si)}
                  <div
                    class="rounded-lg border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] p-3"
                  >
                    <div class="flex items-center justify-between mb-2">
                      <span class="text-sm font-semibold text-gray-100">{set.name}</span>
                      <span
                        class="text-[10px] px-2 py-0.5 rounded bg-[var(--theme-bg-elevated)] border border-[var(--theme-border)] text-gray-400"
                        >{set.pieces[0]?.rank ?? activeRank} · {set.pieces.length} pcs</span
                      >
                    </div>
                    <div class="flex flex-wrap gap-1">
                      {#each set.pieces as piece, pi (piece.id + '-' + pi)}
                        <button
                          onclick={() => goto(`/${$selectedGame?.id ?? ''}/armor/${piece.id}`)}
                          class="text-[11px] px-2 py-1 rounded bg-[var(--theme-bg-elevated)] border border-[var(--theme-border)] text-gray-300 hover:border-[var(--theme-border-strong)]"
                        >
                          {piece.name}
                          <span class="text-gray-400"
                            >[{slotLabel[piece.slot_type] ?? piece.slot_type}]</span
                          >
                        </button>
                      {/each}
                    </div>
                  </div>
                {/each}
              </div>
            {/if}
          {:else}
            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2 mb-4">
              {#each monster.armor as piece, ai (piece.id + '-' + ai)}
                <button
                  onclick={() => goto(`/${$selectedGame?.id ?? ''}/armor/${piece.id}`)}
                  class="text-left px-3 py-2 rounded-lg border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] hover:border-[var(--theme-border-strong)] hover:bg-[var(--theme-bg-elevated)] transition-colors"
                >
                  <div class="text-sm text-gray-100 font-medium truncate">{piece.name}</div>
                  <div class="text-[11px] text-gray-400 mt-0.5">
                    {slotLabel[piece.slot_type] ?? piece.slot_type} · {piece.rank} · Def {piece.defense_base ??
                      0}
                  </div>
                </button>
              {/each}
            </div>
          {/if}
        {/if}
        {#if monster.weapons.length > 0}
          <h3 class="text-sm font-semibold text-gray-200 mb-2">Weapons</h3>
          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2">
            {#each monster.weapons as w, wi (w.id + '-' + wi)}
              <button
                onclick={() => goto(`/${$selectedGame?.id ?? ''}/weapons/${w.id}`)}
                class="text-left px-3 py-2 rounded-lg border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] hover:border-[var(--theme-border-strong)] hover:bg-[var(--theme-bg-elevated)] transition-colors"
              >
                <div class="text-sm text-gray-100 font-medium truncate">{w.name}</div>
                <div class="text-[11px] text-gray-400 mt-0.5">
                  {w.weapon_type} · R{w.rarity ?? 1} · Atk {w.attack ?? 0}
                </div>
              </button>
            {/each}
          </div>
        {/if}
      </section>
    {/if}

    {#if monster.weaknesses.length > 0}
      <section class="mb-8">
        <h2 class="section-title mb-3">Weaknesses</h2>
        <div class="space-y-2">
          {#each monster.weaknesses as w}
            <div class="rounded-lg border themed-card p-4">
              <div class="flex items-center justify-between mb-3">
                <span class="font-semibold text-gray-100">{w.part_name}</span>
              </div>
              <div class="grid grid-cols-4 sm:grid-cols-4 lg:grid-cols-8 gap-2 text-xs">
                <div
                  class="px-2 py-1.5 rounded {weaknessBg(w.sever)} {weaknessColor(
                    w.sever,
                  )} flex flex-col items-center"
                >
                  <span class="text-[9px] uppercase opacity-70">Sever</span>
                  <span class="font-medium">{w.sever ?? '-'}</span>
                </div>
                <div
                  class="px-2 py-1.5 rounded {weaknessBg(w.blunt)} {weaknessColor(
                    w.blunt,
                  )} flex flex-col items-center"
                >
                  <span class="text-[9px] uppercase opacity-70">Blunt</span>
                  <span class="font-medium">{w.blunt ?? '-'}</span>
                </div>
                <div
                  class="px-2 py-1.5 rounded {weaknessBg(w.projectile)} {weaknessColor(
                    w.projectile,
                  )} flex flex-col items-center"
                >
                  <span class="text-[9px] uppercase opacity-70">Shot</span>
                  <span class="font-medium">{w.projectile ?? '-'}</span>
                </div>
                <div
                  class="px-2 py-1.5 rounded {weaknessBg(w.fire)} {weaknessColor(
                    w.fire,
                  )} flex flex-col items-center"
                >
                  <span class="text-[9px] uppercase opacity-70 text-orange-300">Fire</span>
                  <span class="font-medium">{w.fire ?? '-'}</span>
                </div>
                <div
                  class="px-2 py-1.5 rounded {weaknessBg(w.water)} {weaknessColor(
                    w.water,
                  )} flex flex-col items-center"
                >
                  <span class="text-[9px] uppercase opacity-70 text-blue-300">Water</span>
                  <span class="font-medium">{w.water ?? '-'}</span>
                </div>
                <div
                  class="px-2 py-1.5 rounded {weaknessBg(w.thunder)} {weaknessColor(
                    w.thunder,
                  )} flex flex-col items-center"
                >
                  <span class="text-[9px] uppercase opacity-70 text-yellow-300">Thunder</span>
                  <span class="font-medium">{w.thunder ?? '-'}</span>
                </div>
                <div
                  class="px-2 py-1.5 rounded {weaknessBg(w.ice)} {weaknessColor(
                    w.ice,
                  )} flex flex-col items-center"
                >
                  <span class="text-[9px] uppercase opacity-70 text-cyan-300">Ice</span>
                  <span class="font-medium">{w.ice ?? '-'}</span>
                </div>
                <div
                  class="px-2 py-1.5 rounded {weaknessBg(w.dragon)} {weaknessColor(
                    w.dragon,
                  )} flex flex-col items-center"
                >
                  <span class="text-[9px] uppercase opacity-70 text-purple-300">Dragon</span>
                  <span class="font-medium">{w.dragon ?? '-'}</span>
                </div>
              </div>
            </div>
          {/each}
        </div>
      </section>
    {/if}
  {/if}
</div>
