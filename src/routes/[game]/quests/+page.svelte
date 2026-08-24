<script lang="ts">
  import { goto } from '$app/navigation';
  import { selectedGame } from '$lib/stores/game';
  import { api, type Quest } from '$lib/api';
  import Card from '$lib/components/ui/card.svelte';

  const game = $derived($selectedGame);
  const dbId = $derived(game?.dbId);

  let quests = $state<Quest[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let hubFilter = $state<string>('all');

  $effect(() => {
    if (dbId == null) return;
    loading = true;
    error = null;
    api.getQuests(dbId)
      .then((data) => {
        quests = data;
      })
      .catch((e) => {
        error = String(e);
      })
      .finally(() => {
        loading = false;
      });
  });

  const hubMeta: Record<string, { label: string; sub: string; icon: string }> = {
    elder: { label: 'Village Elder', sub: 'Low Rank Village', icon: '🏠' },
    nekoto: { label: 'Village Nekoto', sub: 'High Rank Village', icon: '🐱' },
    guild_low: { label: 'Guild Low', sub: '★1-5', icon: '⚔️' },
    guild_high: { label: 'Guild High', sub: '★6-8', icon: '🛡️' },
    guild_g: { label: 'Guild G', sub: 'G★1-3', icon: '👑' },
    other: { label: 'Other', sub: 'Training & Treasure', icon: '📦' },
  };

  const hubs = $derived(
    Array.from(new Set(quests.map(q => q.hub).filter((h): h is string => !!h))).sort((a,b)=>{
      const order = ['elder','nekoto','guild_low','guild_high','guild_g','other'];
      return order.indexOf(a)-order.indexOf(b);
    })
  );

  const hubCounts = $derived(
    hubs.reduce((acc, h) => { acc[h] = quests.filter(q=>q.hub===h).length; return acc; }, {} as Record<string,number>)
  );

  const filtered = $derived(
    hubFilter === 'all' ? quests : quests.filter(q => q.hub === hubFilter)
  );

  const grouped = $derived(
    hubFilter === 'all'
      ? hubs.map(h => ({ hub: h, items: quests.filter(q=>q.hub===h) }))
      : [{ hub: hubFilter, items: filtered }]
  );

  function open(id: number) {
    if (!game) return;
    goto(`/${game.id}/quests/${id}`);
  }

  const rankColor: Record<string, string> = {
    Low: 'bg-gray-700 text-gray-300',
    High: 'bg-blue-900/40 text-blue-300',
    G: 'bg-yellow-900/40 text-yellow-300',
  };

  const typeIcon: Record<string, string> = {
    Hunting: '⚔️',
    Gathering: '🧺',
    Slaying: '🗡️',
    Capturing: '🪤',
    Training: '🎓',
  };

  function starsLabel(q: Quest): string {
    if (q.stars == null) return '';
    if (q.hub === 'guild_g') return `G★${q.stars}`;
    if (q.hub === 'other') return `★${q.stars}`;
    return `★${q.stars}`;
  }
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-6">
    <h1 class="text-2xl font-bold text-gray-100">Quests</h1>
    <p class="text-sm text-gray-500 mt-1">
      {#if game}
        {game.shortName} · {quests.length} quests · {hubs.length} hubs · Key quests, rewards and drop rates
      {:else}
        Select a game first
      {/if}
    </p>
  </div>

  {#if loading}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Loading quests...</p>
    </div>
  {:else if error}
    <div class="bg-red-950/30 border border-red-900 rounded-lg p-8 text-center">
      <p class="text-red-400">Failed to load quests</p>
      <p class="text-gray-500 text-sm mt-2">{error}</p>
    </div>
  {:else if quests.length === 0}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">No quests found for {game?.shortName ?? 'this game'}</p>
    </div>
  {:else}
    <!-- Hub filter tabs -->
    <div class="flex flex-wrap gap-2 mb-6">
      <button
        onclick={() => (hubFilter = 'all')}
        class="px-3 py-1.5 text-xs rounded-full border transition-colors"
        style={hubFilter === 'all'
          ? `background-color: color-mix(in oklab, var(--theme-accent) 12%, transparent); border-color: color-mix(in oklab, var(--theme-accent) 50%, transparent); color: var(--theme-accent);`
          : `background-color: var(--theme-bg-surface); border-color: var(--theme-border); color: rgb(156 163 175);`}
      >
        All ({quests.length})
      </button>
      {#each hubs as hub}
        {@const meta = hubMeta[hub] ?? { label: hub, sub: '', icon: '📜' }}
        <button
          onclick={() => (hubFilter = hub)}
          class="px-3 py-1.5 text-xs rounded-full border transition-colors flex items-center gap-1.5"
          style={hubFilter === hub
            ? `background-color: color-mix(in oklab, var(--theme-accent) 12%, transparent); border-color: color-mix(in oklab, var(--theme-accent) 50%, transparent); color: var(--theme-accent);`
            : `background-color: var(--theme-bg-surface); border-color: var(--theme-border); color: rgb(156 163 175);`}
        >
          <span>{meta.icon}</span>
          <span>{meta.label}</span>
          <span class="text-[10px] opacity-60">({hubCounts[hub] ?? 0})</span>
        </button>
      {/each}
    </div>

    {#each grouped as group}
      {#if hubFilter === 'all' && group.items.length > 0}
        {@const meta = hubMeta[group.hub] ?? { label: group.hub ?? 'Unknown', sub: '', icon: '📜' }}
        <div class="mb-6">
          <div class="flex items-center gap-2 mb-3">
            <span class="text-sm">{meta.icon}</span>
            <h2 class="text-sm font-semibold text-gray-200">{meta.label}</h2>
            <span class="text-[11px] text-gray-500">{meta.sub} · {group.items.length}</span>
            <div class="flex-1 h-px bg-[var(--theme-border)] ml-2"></div>
          </div>
          <div class="space-y-2">
            {#each group.items as quest}
              <button onclick={() => open(quest.id)} class="w-full text-left">
                <Card class="p-4 border transition-all cursor-pointer themed-card">
                  <div class="flex items-start justify-between gap-3">
                    <div class="min-w-0 flex-1">
                      <div class="flex items-center gap-2 mb-1 flex-wrap">
                        <span class="text-base">{typeIcon[quest.type ?? ''] ?? '📜'}</span>
                        <h3 class="font-semibold text-gray-100">{quest.name}</h3>
                        {#if quest.stars}
                          <span class="text-[10px] px-1.5 py-0.5 rounded bg-[var(--theme-bg-elevated)] text-gray-300 border border-[var(--theme-border)]">{starsLabel(quest)}</span>
                        {/if}
                        {#if quest.is_key_quest}
                          <span class="text-[10px] uppercase tracking-wide px-1.5 py-0.5 rounded bg-yellow-500/10 text-yellow-500 border border-yellow-500/30">
                            Key
                          </span>
                        {/if}
                      </div>
                      <div class="flex flex-wrap gap-3 text-xs text-gray-500">
                        {#if quest.type}
                          <span>{quest.type}</span>
                        {/if}
                        {#if quest.location}
                          <span>📍 {quest.location}</span>
                        {/if}
                        {#if quest.client}
                          <span>👤 {quest.client}</span>
                        {/if}
                        {#if quest.time_limit}
                          <span>⏱️ {quest.time_limit} min</span>
                        {/if}
                        {#if quest.requirements}
                          <span class="text-amber-400/70">🔓 {quest.requirements}</span>
                        {/if}
                      </div>
                    </div>
                    {#if quest.rank}
                      <span class="text-[10px] uppercase tracking-wide px-2 py-0.5 rounded shrink-0 {rankColor[quest.rank] ?? 'bg-gray-800 text-gray-400'}">
                        {quest.rank}
                      </span>
                    {/if}
                  </div>
                </Card>
              </button>
            {/each}
          </div>
        </div>
      {:else if hubFilter !== 'all'}
        <div class="space-y-2">
          {#each group.items as quest}
            <button onclick={() => open(quest.id)} class="w-full text-left">
              <Card class="p-4 border transition-all cursor-pointer themed-card">
                <div class="flex items-start justify-between gap-3">
                  <div class="min-w-0 flex-1">
                    <div class="flex items-center gap-2 mb-1 flex-wrap">
                      <span class="text-base">{typeIcon[quest.type ?? ''] ?? '📜'}</span>
                      <h3 class="font-semibold text-gray-100">{quest.name}</h3>
                      {#if quest.stars}
                        <span class="text-[10px] px-1.5 py-0.5 rounded bg-[var(--theme-bg-elevated)] text-gray-300 border border-[var(--theme-border)]">{starsLabel(quest)}</span>
                      {/if}
                      {#if quest.is_key_quest}
                        <span class="text-[10px] uppercase tracking-wide px-1.5 py-0.5 rounded bg-yellow-500/10 text-yellow-500 border border-yellow-500/30">
                          Key
                        </span>
                      {/if}
                    </div>
                    <div class="flex flex-wrap gap-3 text-xs text-gray-500">
                      {#if quest.type}
                        <span>{quest.type}</span>
                      {/if}
                      {#if quest.location}
                        <span>📍 {quest.location}</span>
                      {/if}
                      {#if quest.client}
                        <span>👤 {quest.client}</span>
                      {/if}
                      {#if quest.time_limit}
                        <span>⏱️ {quest.time_limit} min</span>
                      {/if}
                      {#if quest.requirements}
                        <span class="text-amber-400/70">🔓 {quest.requirements}</span>
                      {/if}
                    </div>
                  </div>
                  {#if quest.rank}
                    <span class="text-[10px] uppercase tracking-wide px-2 py-0.5 rounded shrink-0 {rankColor[quest.rank] ?? 'bg-gray-800 text-gray-400'}">
                      {quest.rank}
                    </span>
                  {/if}
                </div>
              </Card>
            </button>
          {/each}
        </div>
      {/if}
    {/each}
  {/if}
</div>
