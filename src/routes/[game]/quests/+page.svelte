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
  let rankFilter = $state<string>('all');

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

  const ranks = $derived(['all', ...Array.from(new Set(quests.map(q => q.rank).filter((r): r is string => !!r)))]);
  const filtered = $derived(
    rankFilter === 'all' ? quests : quests.filter(q => q.rank === rankFilter)
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
  };
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-6">
    <h1 class="text-2xl font-bold text-gray-100">Quests</h1>
    <p class="text-sm text-gray-500 mt-1">
      {#if game}
        {game.shortName} · {quests.length} quests · Key quests, rewards and drop rates
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
    <div class="flex flex-wrap gap-2 mb-4">
      {#each ranks as rank}
        <button
          onclick={() => (rankFilter = rank)}
          class="px-3 py-1.5 text-xs rounded-full border transition-colors"
          style={rankFilter === rank
            ? `background-color: color-mix(in oklab, var(--theme-accent) 12%, transparent); border-color: color-mix(in oklab, var(--theme-accent) 50%, transparent); color: var(--theme-accent);`
            : `background-color: var(--theme-bg-surface); border-color: var(--theme-border); color: rgb(156 163 175);`}
        >
          {rank === 'all' ? 'All' : rank}
        </button>
      {/each}
    </div>

    <div class="space-y-2">
      {#each filtered as quest}
        <button onclick={() => open(quest.id)} class="w-full text-left">
          <Card class="p-4 border transition-all cursor-pointer themed-card">
            <div class="flex items-start justify-between gap-3">
              <div class="min-w-0 flex-1">
                <div class="flex items-center gap-2 mb-1">
                  <span class="text-base">{typeIcon[quest.type ?? ''] ?? '📜'}</span>
                  <h3 class="font-semibold text-gray-100">{quest.name}</h3>
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
                  {#if quest.time_limit}
                    <span>⏱️ {quest.time_limit} min</span>
                  {/if}
                  {#if quest.faints_allowed !== null && quest.faints_allowed !== undefined}
                    <span>💀 Faints: {quest.faints_allowed}</span>
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
</div>
