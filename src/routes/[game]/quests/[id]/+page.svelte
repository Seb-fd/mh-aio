<script lang="ts">
  import { page } from '$app/state'
  import ErrorState from '$lib/components/ui/error-state.svelte'
  import { goto } from '$app/navigation'
  import { selectedGame } from '$lib/stores/game'
  import { api, type QuestDetail } from '$lib/api'
  import DetailHeader from '$lib/components/detail-header.svelte'
  import ItemIcon from '$lib/components/item-icon.svelte'
  import Skeleton from '$lib/components/ui/skeleton.svelte'
  import EmptyState from '$lib/components/ui/empty-state.svelte'
  import { Swords, Package } from '@lucide/svelte'
  import { fallbackLabel } from '$lib/utils/mh'

  const id = $derived(Number(page.params.id))
  let quest = $state<QuestDetail | null>(null)
  let loading = $state(true)
  let error = $state<string | null>(null)

  const game = $derived($selectedGame)

  $effect(() => {
    if (!id || Number.isNaN(id)) return
    loading = true
    error = null
    api
      .getQuestDetail(id)
      .then((data) => {
        quest = data
      })
      .catch((e) => {
        error = String(e)
      })
      .finally(() => {
        loading = false
      })
  })

  const hubMeta: Record<string, { label: string }> = {
    elder: { label: 'Village Elder' },
    nekoto: { label: 'Village Nekoto' },
    guild_low: { label: 'Guild Low' },
    guild_high: { label: 'Guild High' },
    guild_g: { label: 'Guild G' },
    event: { label: 'Event Quest' },
    challenge: { label: 'Challenge Quest' },
    training: { label: 'Training School' },
    treasure: { label: 'Treasure Hunt' },
    hot_spring: { label: 'Hot Spring Quest' },
    drink: { label: 'Drink Quest' },
    nyanta: { label: 'Nyanta Quest' },
    other: { label: 'Other' },
  }

  function hubLabel(hub: string | null | undefined, stars: number | null | undefined): string {
    if (!hub) return ''
    const meta = hubMeta[hub]
    const base = meta ? meta.label : hub
    if (stars != null) {
      if (hub === 'guild_g') return `${base} G★${stars}`
      return `${base} ★${stars}`
    }
    return base
  }

  function goToItem(itemId: number) {
    if (!game) return
    goto(`/${game.id}/items/${itemId}`)
  }

  function parseMonsters(json: string | null): string[] {
    if (!json) return []
    try {
      const arr = JSON.parse(json)
      return Array.isArray(arr) ? arr : []
    } catch {
      return []
    }
  }
</script>

<div class="max-w-5xl mx-auto numbered-sections">
  {#if loading}
    <div class="space-y-3" aria-busy="true">
      <Skeleton lines={2} />
      <Skeleton lines={3} />
    </div>
  {:else if error}
    <ErrorState title="Failed to load quest" {error} />
  {:else if !quest}
    <EmptyState title="Quest not found" hint="It may belong to another game." />
  {:else}
    <DetailHeader
      title={quest.name}
      subtitle={quest.name_original
        ? `${quest.name_original} · ${quest.client ?? quest.type ?? ''}`
        : (quest.client ?? quest.type ?? '')}
      iconUrl={quest.icon_url}
      tags={[
        {
          label: fallbackLabel(quest.rank),
          color:
            'bg-[var(--theme-bg-elevated)] text-[var(--theme-text-accent)] border-[var(--theme-border-strong)]',
        },
        ...(quest.hub
          ? [
              {
                label: hubLabel(quest.hub, quest.stars),
                color: 'bg-[var(--theme-bg-elevated)] text-gray-300 border-[var(--theme-border)]',
              },
            ]
          : []),
        ...(quest.is_key_quest
          ? [
              {
                label: 'Key Quest',
                color: 'bg-yellow-500/10 text-yellow-500 border border-yellow-500/30',
              },
            ]
          : []),
        ...(quest.is_urgent
          ? [
              {
                label: 'Urgent Quest',
                color: 'bg-red-500/10 text-red-400 border border-red-500/30',
              },
            ]
          : []),
        ...((quest.hub === 'event' || quest.hub === 'challenge') &&
        quest.description?.includes('Exclusive')
          ? [
              {
                label: 'Exclusive',
                color: 'bg-purple-500/20 text-purple-300 border border-purple-500/30',
              },
            ]
          : []),
      ]}
    />

    <div class="flex flex-wrap gap-2 mb-4">
      <span
        class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-[var(--theme-bg-elevated)] border border-[var(--theme-border)] text-xs text-gray-300"
      >
        <ItemIcon
          iconUrl={quest.icon_url}
          iconName={quest.icon_name}
          iconColor={quest.icon_color}
          size={18}
          alt={quest.type ?? 'type'}
        />
        {quest.type ?? '—'}
      </span>
      <span
        class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-[var(--theme-bg-elevated)] border border-[var(--theme-border)] text-xs text-gray-300"
      >
        <ItemIcon
          iconUrl={quest.hub_icon_url}
          iconName={quest.hub_icon_name}
          iconColor={quest.hub_icon_color}
          size={18}
          alt=""
        />
        {hubLabel(quest.hub, quest.stars)}
      </span>
    </div>

    <div class="stat-grid gap-3 mb-8">
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-400">Type</p>
        <p class="text-base font-semibold text-gray-100 mt-1">{quest.type ?? '—'}</p>
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-400">Location</p>
        <p class="text-base font-semibold text-gray-100 mt-1">{quest.location ?? '—'}</p>
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-400">Time</p>
        <p class="text-base font-semibold text-gray-100 mt-1 tabular-nums">
          {quest.time_limit ?? '—'} min
        </p>
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-400">Faints</p>
        <p class="text-base font-semibold text-gray-100 mt-1 tabular-nums">
          {quest.faints_allowed ?? '—'}
        </p>
      </div>
    </div>

    <div class="stat-grid gap-3 mb-8">
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-400">Client</p>
        <p class="text-sm font-semibold text-gray-100 mt-1">{quest.client ?? '—'}</p>
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-400">Hub</p>
        <p class="text-sm font-semibold text-gray-100 mt-1">{hubLabel(quest.hub, quest.stars)}</p>
        {#if quest.requirements}
          <p class="text-[11px] text-amber-400/70 mt-1">{quest.requirements}</p>
        {/if}
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-400">Reward</p>
        <p class="text-base font-semibold text-gray-100 mt-1">
          {quest.reward_money ?? '—'}{quest.reward_money != null ? 'z' : ''}
        </p>
        {#if quest.contract_fee != null}
          <p class="text-[11px] text-gray-400">Fee: {quest.contract_fee}z</p>
        {/if}
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-400">Objective</p>
        <p class="text-sm font-semibold text-gray-100 mt-1">{quest.objective ?? '—'}</p>
      </div>
    </div>

    {#if parseMonsters(quest.main_monsters).length > 0}
      <section class="mb-8">
        <h2 class="section-title mb-3">Target Monsters</h2>
        <div class="flex flex-wrap gap-2">
          {#each parseMonsters(quest.main_monsters) as mon}
            <span
              class="inline-flex items-center gap-1.5 px-3 min-h-[36px] py-1.5 rounded-full bg-[var(--theme-bg-elevated)] border border-[var(--theme-border)] text-sm text-gray-200"
              ><Swords class="h-3.5 w-3.5 shrink-0" aria-hidden="true" />{mon}</span
            >
          {/each}
        </div>
      </section>
    {/if}

    {#if quest.description}
      <section class="mb-8">
        <h2 class="section-title mb-3">Description</h2>
        <div class="rounded-lg border themed-card p-5 leading-relaxed text-gray-200 text-[15px]">
          {quest.description}
        </div>
      </section>
    {/if}

    {#if quest.requirements}
      <section class="mb-8">
        <h2 class="section-title mb-3">Requirements</h2>
        <div class="rounded-lg border themed-card p-4 text-gray-200 text-sm">
          {quest.requirements}
        </div>
      </section>
    {/if}

    {#if quest.rewards && quest.rewards.length > 0}
      <section class="mb-8">
        <h2 class="section-title mb-3">
          Quest Rewards · {quest.rewards.length}
        </h2>
        <div class="space-y-2">
          {#each quest.rewards as r}
            <button
              onclick={() => goToItem(r.item_id)}
              aria-label="{r.item_name} x{r.quantity}{r.probability != null
                ? `, ${Math.round(r.probability * 100)} percent`
                : ''}"
              class="w-full text-left px-4 min-h-[52px] py-3 rounded-lg border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] hover:border-[var(--theme-border-strong)] hover:bg-[var(--theme-bg-elevated)] transition-colors motion-safe:transition-colors motion-reduce:transition-none group focus-visible:outline-none focus-visible:ring-2"
            >
              <div class="flex items-center gap-3">
                <Package class="h-5 w-5 shrink-0 text-gray-400" aria-hidden="true" />
                <div class="flex-1 min-w-0">
                  <div class="flex items-center gap-2">
                    <span
                      class="text-sm text-gray-100 group-hover:text-[var(--theme-text-accent)] transition-colors motion-safe:transition-colors motion-reduce:transition-none"
                      >{r.item_name}</span
                    >
                    <span class="text-xs text-gray-400 tabular-nums">x{r.quantity}</span>
                    {#if r.condition}
                      <span class="text-[11px] text-amber-400/70">· {r.condition}</span>
                    {/if}
                    <span class="text-xs text-gray-400 ml-auto tabular-nums"
                      >{r.probability != null ? `${Math.round(r.probability * 100)}%` : '—'}</span
                    >
                  </div>
                  {#if r.probability != null}
                    <div class="mt-1.5 flex items-center gap-2">
                      <div class="probability-bar" role="presentation">
                        <span style="--prob: {Math.round(r.probability * 100)}"></span>
                      </div>
                    </div>
                  {/if}
                </div>
              </div>
            </button>
          {/each}
        </div>
      </section>
    {/if}
  {/if}
</div>
