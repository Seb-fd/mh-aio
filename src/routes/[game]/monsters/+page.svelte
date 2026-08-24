<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { selectedGame, GAMES } from '$lib/stores/game';
  import { api, type Monster } from '$lib/api';
  import Card from '$lib/components/ui/card.svelte';

  const game = $derived($selectedGame);
  const dbId = $derived(game?.dbId);

  let monsters = $state<Monster[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);

  $effect(() => {
    if (dbId == null) return;
    console.log('[monsters] loading gameId', dbId);
    loading = true;
    error = null;
    api.getMonsters(dbId)
      .then((data) => {
        console.log('[monsters] loaded', data.length);
        monsters = data;
      })
      .catch((e) => {
        console.error('[monsters] failed', e);
        error = String(e);
      })
      .finally(() => {
        loading = false;
      });
  });

  function open(id: number) {
    if (!game) return;
    goto(`/${game.id}/monsters/${id}`);
  }

  function speciesColor(species: string | null): string {
    if (!species) return 'text-gray-400';
    if (species === 'Elder Dragon') return 'text-yellow-400';
    if (species === 'Flying Wyvern') return 'text-red-400';
    if (species === 'Bird Wyvern') return 'text-orange-400';
    if (species === 'Fanged Beast') return 'text-amber-400';
    if (species === 'Leviathan') return 'text-blue-400';
    if (species === 'Carapaceon') return 'text-cyan-400';
    if (species === 'Amphibian') return 'text-emerald-400';
    return 'text-gray-400';
  }
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-6">
    <h1 class="text-2xl font-bold text-gray-100">Monsters</h1>
    <p class="text-sm text-gray-500 mt-1">
      {#if game}
        {game.shortName} · {monsters.length} monster{monsters.length === 1 ? '' : 's'} registered
      {:else}
        Select a game first
      {/if}
    </p>
  </div>

  {#if loading}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Loading monsters...</p>
    </div>
  {:else if error}
    <div class="bg-red-950/30 border border-red-900 rounded-lg p-8 text-center">
      <p class="text-red-400">Failed to load monsters</p>
      <p class="text-gray-500 text-sm mt-2">{error}</p>
    </div>
  {:else if monsters.length === 0}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">No monsters found for {game?.shortName ?? 'this game'}</p>
    </div>
  {:else}
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-3">
      {#each monsters as monster}
        <button onclick={() => open(monster.id)} class="text-left">
          <Card class="p-4 border transition-all cursor-pointer hover:scale-[1.02] themed-card">
            <div class="flex items-start justify-between gap-2">
              <div class="min-w-0">
                <h3 class="font-semibold text-gray-100 truncate group-hover:text-[var(--theme-text-accent)]">{monster.name}</h3>
                {#if monster.species}
                  <p class="text-xs {speciesColor(monster.species)} mt-0.5">{monster.species}</p>
                {/if}
              </div>
              {#if monster.size}
                <span class="text-[10px] uppercase tracking-wide text-gray-500 bg-[var(--theme-bg-elevated)] px-2 py-0.5 rounded shrink-0 border border-[var(--theme-border)]">
                  {monster.size}
                </span>
              {/if}
            </div>
          </Card>
        </button>
      {/each}
    </div>
  {/if}
</div>
