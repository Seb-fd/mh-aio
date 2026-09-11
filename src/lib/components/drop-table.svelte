<script lang="ts">
  import { goto } from '$app/navigation'
  import { selectedGame } from '$lib/stores/game'
  import type { ItemSource } from '$lib/api'
  import type { Component } from 'svelte'
  import Badge from './ui/badge.svelte'
  import { rankTone } from '$lib/utils/mh'
  import {
    Swords,
    Lock,
    Sparkles,
    Hammer,
    ScrollText,
    Pickaxe,
    ShoppingBasket,
    Bug,
    Fish,
    ShoppingCart,
    Handshake,
    Wheat,
    Sailboat,
    Scissors,
    CircleHelp,
    MapPin,
  } from '@lucide/svelte'

  let { sources }: { sources: ItemSource[] } = $props()

  const game = $derived($selectedGame)

  interface SourceMeta {
    label: string
    Icon: Component
    color: string
  }

  const sourceTypeLabel: Record<string, SourceMeta> = {
    carve: { label: 'Carve', Icon: Swords, color: 'text-red-400' },
    capture: { label: 'Capture', Icon: Lock, color: 'text-emerald-400' },
    drop: { label: 'Shiny Drop', Icon: Sparkles, color: 'text-yellow-400' },
    break: { label: 'Break Part', Icon: Hammer, color: 'text-orange-400' },
    quest_reward: { label: 'Quest Reward', Icon: ScrollText, color: 'text-blue-400' },
    shiny: { label: 'Shiny Pickup', Icon: Sparkles, color: 'text-yellow-400' },
    mining: { label: 'Mining', Icon: Pickaxe, color: 'text-orange-400' },
    gather: { label: 'Gathering', Icon: ShoppingBasket, color: 'text-emerald-400' },
    bug: { label: 'Bug', Icon: Bug, color: 'text-green-400' },
    fish: { label: 'Fishing', Icon: Fish, color: 'text-cyan-400' },
    shop: { label: 'Shop', Icon: ShoppingCart, color: 'text-cyan-400' },
    trade: { label: 'Trade', Icon: Handshake, color: 'text-amber-400' },
    farm: { label: 'Farm', Icon: Wheat, color: 'text-green-400' },
  }

  const fallbackMeta: SourceMeta = { label: '', Icon: CircleHelp, color: 'text-gray-400' }

  const rankOrder: Record<string, number> = { Low: 0, High: 1, G: 2 }

  function sortByRankProb(a: ItemSource, b: ItemSource): number {
    const ra = rankOrder[a.rank ?? ''] ?? 99
    const rb = rankOrder[b.rank ?? ''] ?? 99
    if (ra !== rb) return ra - rb
    return (b.probability ?? 0) - (a.probability ?? 0)
  }

  function methodDetailLabel(s: ItemSource): string | null {
    if (s.source_type === 'carve' && s.part) {
      if (s.part.toLowerCase() === 'tail') return 'Tail Carve'
      if (s.part.toLowerCase() === 'body') return 'Body Carve'
      return `${s.part} Carve`
    }
    if (s.source_type === 'break' && s.part) return `Break: ${s.part}`
    if (s.source_type === 'drop' && s.part) return s.part
    if (s.part) return s.part
    return null
  }

  function goToSource(source: ItemSource) {
    if (!game) return
    const monsterSources = ['carve', 'capture', 'drop', 'break']
    if (monsterSources.includes(source.source_type) && source.source_id != null) {
      goto(`/${game.id}/monsters/${source.source_id}`)
    } else if (source.source_type === 'quest_reward' && source.source_id != null) {
      goto(`/${game.id}/quests/${source.source_id}`)
    }
  }

  function pct(p: number | null | undefined): number {
    if (p == null) return 0
    return Math.round(p * 100)
  }

  function qtyLabel(s: ItemSource): string {
    const min = s.quantity_min ?? 0
    const max = s.quantity_max ?? 0
    if (min === max) return `x${min}`
    return `x${min}-${max}`
  }

  type Section = {
    key: string
    title: string
    Icon: Component
    color: string
    items: ItemSource[]
  }

  const sections: Section[] = $derived.by(() => {
    const isTail = (s: ItemSource) => s.source_type === 'carve' && s.part?.toLowerCase() === 'tail'
    const isBodyCarve = (s: ItemSource) => s.source_type === 'carve' && !isTail(s)

    const carveBody = sources.filter(isBodyCarve).sort(sortByRankProb)
    const carveTail = sources.filter(isTail).sort(sortByRankProb)
    const breaks = sources.filter((s) => s.source_type === 'break').sort(sortByRankProb)
    const captures = sources.filter((s) => s.source_type === 'capture').sort(sortByRankProb)
    const drops = sources
      .filter((s) => s.source_type === 'drop' || s.source_type === 'shiny')
      .sort(sortByRankProb)
    const gathering = sources
      .filter((s) => ['gather', 'mining', 'bug', 'fish'].includes(s.source_type))
      .sort(sortByRankProb)
    const quests = sources.filter((s) => s.source_type === 'quest_reward').sort(sortByRankProb)
    const shops = sources.filter((s) => s.source_type === 'shop').sort(sortByRankProb)
    const tradesVeggie = sources
      .filter((s) => s.source_type === 'trade' && (s.location?.includes('Veggie Elder') ?? false))
      .sort(sortByRankProb)
    const tradesTrenya = sources
      .filter((s) => s.source_type === 'trade' && (s.location?.includes('Trenya') ?? false))
      .sort(sortByRankProb)
    const tradesOther = sources
      .filter(
        (s) =>
          s.source_type === 'trade' &&
          !s.location?.includes('Veggie Elder') &&
          !s.location?.includes('Trenya'),
      )
      .sort(sortByRankProb)
    const farms = sources.filter((s) => s.source_type === 'farm').sort(sortByRankProb)

    const out: Section[] = []
    if (carveBody.length)
      out.push({
        key: 'carve_body',
        title: 'Carve',
        Icon: Swords,
        color: 'text-red-400',
        items: carveBody,
      })
    if (carveTail.length)
      out.push({
        key: 'carve_tail',
        title: 'Tail Cut',
        Icon: Scissors,
        color: 'text-orange-400',
        items: carveTail,
      })
    if (breaks.length)
      out.push({
        key: 'break',
        title: 'Part Break',
        Icon: Hammer,
        color: 'text-orange-400',
        items: breaks,
      })
    if (captures.length)
      out.push({
        key: 'capture',
        title: 'Capture',
        Icon: Lock,
        color: 'text-emerald-400',
        items: captures,
      })
    if (drops.length)
      out.push({
        key: 'drop',
        title: 'Shiny Drop',
        Icon: Sparkles,
        color: 'text-yellow-400',
        items: drops,
      })
    if (gathering.length)
      out.push({
        key: 'gathering',
        title: 'Gathering',
        Icon: ShoppingBasket,
        color: 'text-emerald-400',
        items: gathering,
      })
    if (shops.length)
      out.push({
        key: 'shop',
        title: 'Shop (Consolidated)',
        Icon: ShoppingCart,
        color: 'text-cyan-400',
        items: shops,
      })
    if (tradesVeggie.length)
      out.push({
        key: 'trade_veggie',
        title: 'Veggie Elder Trade',
        Icon: Handshake,
        color: 'text-amber-400',
        items: tradesVeggie,
      })
    if (tradesTrenya.length)
      out.push({
        key: 'trade_trenya',
        title: 'Trenya Boat Trade',
        Icon: Sailboat,
        color: 'text-sky-400',
        items: tradesTrenya,
      })
    if (tradesOther.length)
      out.push({
        key: 'trade_other',
        title: 'Trade',
        Icon: Handshake,
        color: 'text-amber-400',
        items: tradesOther,
      })
    if (farms.length)
      out.push({
        key: 'farm',
        title: 'Pokke Farm',
        Icon: Wheat,
        color: 'text-green-400',
        items: farms,
      })
    if (quests.length)
      out.push({
        key: 'quest',
        title: 'Quest Rewards',
        Icon: ScrollText,
        color: 'text-blue-400',
        items: quests,
      })
    // Fallback: any remaining types not covered (e.g., future types) go last
    const covered = new Set([
      ...carveBody,
      ...carveTail,
      ...breaks,
      ...captures,
      ...drops,
      ...gathering,
      ...shops,
      ...tradesVeggie,
      ...tradesTrenya,
      ...tradesOther,
      ...farms,
      ...quests,
    ])
    const remaining = sources.filter((s) => !covered.has(s)).sort(sortByRankProb)
    if (remaining.length)
      out.push({
        key: 'other',
        title: 'Other',
        Icon: CircleHelp,
        color: 'text-gray-400',
        items: remaining,
      })
    return out
  })
</script>

<div class="space-y-6">
  {#if sources.length === 0}
    <div class="px-4 py-3 rounded-lg border border-dashed border-[var(--theme-border)] text-center">
      <p class="text-sm text-gray-400">No known sources</p>
    </div>
  {:else}
    {#each sections as sec (sec.key)}
      <div>
        <div class="flex items-center gap-2 mb-2">
          <sec.Icon class="h-3.5 w-3.5 shrink-0 {sec.color}" aria-hidden="true" />
          <h3
            class="text-xs uppercase tracking-wider font-semibold"
            style="color: var(--theme-accent);"
          >
            {sec.title}
          </h3>
          <span class="text-[11px] text-gray-400">· {sec.items.length}</span>
          <div class="flex-1 h-px bg-[var(--theme-border)] ml-2"></div>
        </div>
        <div class="space-y-2">
          {#each sec.items as source, si (source.source_type + '|' + (source.source_id ?? '') + '|' + (source.location ?? '') + '|' + (source.rank ?? '') + '|' + (source.condition ?? '') + '|' + si)}
            {@const raw = sourceTypeLabel[source.source_type]}
            {@const meta: SourceMeta = raw
              ? { ...raw }
              : { ...fallbackMeta, label: source.source_type }}
            {@const canNavigate =
              ['carve', 'capture', 'drop', 'break', 'quest_reward'].includes(source.source_type) &&
              source.source_id != null}
            {#if canNavigate}
              <button
                onclick={() => goToSource(source)}
                class="w-full block px-4 py-3 rounded-lg border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] hover:border-[var(--theme-border-strong)] hover:bg-[var(--theme-bg-elevated)] transition-colors text-left group"
              >
                {@render rowContent(source, meta, true)}
              </button>
            {:else}
              <div
                class="block px-4 py-3 rounded-lg border border-[var(--theme-border)] bg-[var(--theme-bg-surface)]"
              >
                {@render rowContent(source, meta, false)}
              </div>
            {/if}
          {/each}
        </div>
      </div>
    {/each}
  {/if}
</div>

{#snippet rowContent(source: ItemSource, meta: SourceMeta, clickable: boolean)}
  {@const detail = methodDetailLabel(source)}
  <div class="flex items-center gap-3">
    <meta.Icon class="h-5 w-5 shrink-0 {meta.color}" aria-hidden="true" />
    <div class="flex-1 min-w-0">
      <div class="flex items-center gap-2 flex-wrap">
        <span class="text-xs uppercase tracking-wide {meta.color} font-medium">{meta.label}</span>
        {#if source.source_name}
          <span
            class="text-sm text-gray-100 truncate {clickable
              ? 'group-hover:text-[var(--theme-text-accent)]'
              : ''} transition-colors"
          >
            · {source.source_name}
          </span>
        {/if}
        {#if source.rank}
          <Badge tone={rankTone(source.rank)}>{source.rank}</Badge>
        {/if}
        {#if detail}
          <span class="text-[11px] text-gray-400">· {detail}</span>
        {/if}
        <span class="text-xs text-gray-400 ml-auto shrink-0">{qtyLabel(source)}</span>
      </div>
      {#if source.condition}
        <p class="text-[11px] text-amber-400/80 mt-0.5">※ {source.condition}</p>
      {/if}
      {#if source.location}
        <p class="text-[11px] text-gray-400 mt-0.5 inline-flex items-start gap-1">
          <MapPin class="h-3 w-3 shrink-0 mt-px" aria-hidden="true" />{source.location}
        </p>
      {/if}
      {#if source.probability != null}
        <div class="mt-2 flex items-center gap-2">
          <div class="probability-bar" role="presentation">
            <span style="--prob: {pct(source.probability)}"></span>
          </div>
          <span class="text-[10px] text-gray-400 shrink-0 tabular-nums w-9 text-right"
            >{pct(source.probability)}%</span
          >
        </div>
      {/if}
    </div>
  </div>
{/snippet}
