<script lang="ts">
  import { page } from '$app/state'
  import { goto } from '$app/navigation'
  import { selectedGame } from '$lib/stores/game'
  import { api, type QuestDetail } from '$lib/api'
  import DetailHeader from '$lib/components/detail-header.svelte'
  import ItemIcon from '$lib/components/item-icon.svelte'

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

  const typeIcon: Record<string, string> = {
    Hunting: '⚔️',
    Gathering: '🧺',
    Slaying: '🗡️',
    Capturing: '🪤',
    Training: '🎓',
  }

  const rankColor: Record<string, string> = {
    Low: 'bg-gray-700 text-gray-300',
    High: 'bg-blue-900/40 text-blue-300',
    G: 'bg-yellow-900/40 text-yellow-300',
  }

  const hubMeta: Record<string, { label: string; icon: string }> = {
    elder: { label: 'Village Elder', icon: '🏠' },
    nekoto: { label: 'Village Nekoto', icon: '🐱' },
    guild_low: { label: 'Guild Low', icon: '⚔️' },
    guild_high: { label: 'Guild High', icon: '🛡️' },
    guild_g: { label: 'Guild G', icon: '👑' },
    event: { label: 'Event Quest', icon: '🎉' },
    challenge: { label: 'Challenge Quest', icon: '🏆' },
    training: { label: 'Training School', icon: '🎓' },
    treasure: { label: 'Treasure Hunt', icon: '💰' },
    hot_spring: { label: 'Hot Spring Quest', icon: '♨️' },
    drink: { label: 'Drink Quest', icon: '🍺' },
    nyanta: { label: 'Nyanta Quest', icon: '🐈' },
    other: { label: 'Other', icon: '📦' },
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

<div class="max-w-5xl mx-auto">
  {#if loading}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Loading quest...</p>
    </div>
  {:else if error}
    <div class="bg-red-950/30 border border-red-900 rounded-lg p-8 text-center">
      <p class="text-red-400">Failed to load quest</p>
      <p class="text-gray-500 text-sm mt-2">{error}</p>
    </div>
  {:else if !quest}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Quest not found</p>
    </div>
  {:else}
    <DetailHeader
      title={quest.name}
      subtitle={quest.name_original
        ? `${quest.name_original} · ${quest.client ?? quest.type ?? ''}`
        : (quest.client ?? quest.type ?? '')}
      icon={typeIcon[quest.type ?? ''] ?? '📜'}
      iconUrl={quest.icon_url}
      tags={[
        {
          label: quest.rank ?? 'Unknown',
          color: rankColor[quest.rank ?? ''] ?? 'bg-gray-800 text-gray-300',
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
                label: '✨ Exclusive',
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
          alt={quest.hub ?? 'hub'}
        />
        {hubLabel(quest.hub, quest.stars)}
      </span>
    </div>

    <div class="grid grid-cols-2 sm:grid-cols-4 gap-3 mb-8">
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-500">Type</p>
        <p class="text-base font-semibold text-gray-100 mt-1">{quest.type ?? '—'}</p>
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-500">Location</p>
        <p class="text-base font-semibold text-gray-100 mt-1">{quest.location ?? '—'}</p>
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-500">Time</p>
        <p class="text-base font-semibold text-gray-100 mt-1">{quest.time_limit ?? '—'} min</p>
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-500">Faints</p>
        <p class="text-base font-semibold text-gray-100 mt-1">{quest.faints_allowed ?? '—'}</p>
      </div>
    </div>

    <div class="grid grid-cols-2 sm:grid-cols-4 gap-3 mb-8">
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-500">Client</p>
        <p class="text-sm font-semibold text-gray-100 mt-1">{quest.client ?? '—'}</p>
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-500">Hub</p>
        <p class="text-sm font-semibold text-gray-100 mt-1">{hubLabel(quest.hub, quest.stars)}</p>
        {#if quest.requirements}
          <p class="text-[11px] text-amber-400/70 mt-1">{quest.requirements}</p>
        {/if}
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-500">Reward</p>
        <p class="text-base font-semibold text-gray-100 mt-1">
          {quest.reward_money ?? '—'}{quest.reward_money != null ? 'z' : ''}
        </p>
        {#if quest.contract_fee != null}
          <p class="text-[11px] text-gray-500">Fee: {quest.contract_fee}z</p>
        {/if}
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-500">Objective</p>
        <p class="text-sm font-semibold text-gray-100 mt-1">{quest.objective ?? '—'}</p>
      </div>
    </div>

    {#if parseMonsters(quest.main_monsters).length > 0}
      <section class="mb-8">
        <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-3">
          Target Monsters
        </h2>
        <div class="flex flex-wrap gap-2">
          {#each parseMonsters(quest.main_monsters) as mon}
            <span
              class="px-3 py-1.5 rounded-full bg-[var(--theme-bg-elevated)] border border-[var(--theme-border)] text-sm text-gray-200"
              >🐉 {mon}</span
            >
          {/each}
        </div>
      </section>
    {/if}

    {#if quest.description}
      <section class="mb-8">
        <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-3">
          Description
        </h2>
        <div class="rounded-lg border themed-card p-5 leading-relaxed text-gray-200 text-[15px]">
          {quest.description}
        </div>
      </section>
    {/if}

    {#if quest.requirements}
      <section class="mb-8">
        <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-3">
          Requirements
        </h2>
        <div class="rounded-lg border themed-card p-4 text-gray-200 text-sm">
          {quest.requirements}
        </div>
      </section>
    {/if}

    {#if quest.rewards && quest.rewards.length > 0}
      <section class="mb-8">
        <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-3">
          Quest Rewards · {quest.rewards.length}
        </h2>
        <div class="space-y-2">
          {#each quest.rewards as r}
            <button
              onclick={() => goToItem(r.item_id)}
              class="w-full text-left px-4 py-3 rounded-lg border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] hover:border-[var(--theme-border-strong)] hover:bg-[var(--theme-bg-elevated)] transition-all group"
            >
              <div class="flex items-center gap-3">
                <span class="text-lg">📦</span>
                <div class="flex-1 min-w-0">
                  <div class="flex items-center gap-2">
                    <span
                      class="text-sm text-gray-100 group-hover:text-[var(--theme-text-accent)] transition-colors"
                      >{r.item_name}</span
                    >
                    <span class="text-xs text-gray-500">x{r.quantity}</span>
                    {#if r.condition}
                      <span class="text-[11px] text-amber-400/70">· {r.condition}</span>
                    {/if}
                    <span class="text-xs text-gray-500 ml-auto"
                      >{r.probability != null ? `${Math.round(r.probability * 100)}%` : '—'}</span
                    >
                  </div>
                  {#if r.probability != null}
                    <div class="mt-1.5 flex items-center gap-2">
                      <div
                        class="flex-1 h-1.5 rounded-full bg-[var(--theme-bg-elevated)] overflow-hidden"
                      >
                        <div
                          class="h-full bg-[var(--theme-accent)] rounded-full"
                          style="width: {Math.round(r.probability * 100)}%"
                        ></div>
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
