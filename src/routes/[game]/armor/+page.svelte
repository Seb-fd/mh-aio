<script lang="ts">
  import { goto } from '$app/navigation';
  import { selectedGame } from '$lib/stores/game';
  import { api, type Armor } from '$lib/api';
  import Card from '$lib/components/ui/card.svelte';

  const game = $derived($selectedGame);
  const dbId = $derived(game?.dbId);

  let armors = $state<Armor[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let rankFilter = $state<string>('all');

  $effect(() => {
    if (dbId == null) return;
    console.log('[armor] loading gameId', dbId);
    loading = true;
    error = null;
    api.getArmor(dbId)
      .then((data) => {
        console.log('[armor] loaded', data.length);
        armors = data;
      })
      .catch((e) => {
        console.error('[armor] failed', e);
        error = String(e);
      })
      .finally(() => {
        loading = false;
      });
  });

  const ranks = $derived(['all', ...Array.from(new Set(armors.map(a => a.rank)))]);
  const filtered = $derived(
    rankFilter === 'all' ? armors : armors.filter(a => a.rank === rankFilter)
  );

  function open(id: number) {
    if (!game) return;
    goto(`/${game.id}/armor/${id}`);
  }

  const slotLabel: Record<string, string> = {
    head: 'Helm',
    chest: 'Mail',
    arms: 'Vambraces',
    waist: 'Coil',
    legs: 'Greaves',
  };

  const rankColor: Record<string, string> = {
    Low: 'bg-gray-700 text-gray-300',
    High: 'bg-blue-900/40 text-blue-300',
    G: 'bg-yellow-900/40 text-yellow-300',
  };
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-6">
    <h1 class="text-2xl font-bold text-gray-100">Armor</h1>
    <p class="text-sm text-gray-500 mt-1">
      {#if game}
        {game.shortName} · {armors.length} pieces · Sets, skills and elemental resistances
      {:else}
        Select a game first
      {/if}
    </p>
  </div>

  {#if loading}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Loading armor...</p>
    </div>
  {:else if error}
    <div class="bg-red-950/30 border border-red-900 rounded-lg p-8 text-center">
      <p class="text-red-400">Failed to load armor</p>
      <p class="text-gray-500 text-sm mt-2">{error}</p>
    </div>
  {:else if armors.length === 0}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">No armor found for {game?.shortName ?? 'this game'}</p>
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

    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
      {#each filtered as piece}
        <button onclick={() => open(piece.id)} class="text-left">
          <Card class="p-4 border transition-all cursor-pointer hover:scale-[1.02] themed-card">
            <div class="flex items-start justify-between gap-2 mb-2">
              <h3 class="font-semibold text-gray-100 truncate">{piece.name}</h3>
              <span class="text-[10px] uppercase tracking-wide px-2 py-0.5 rounded shrink-0 {rankColor[piece.rank] ?? 'bg-gray-800 text-gray-400'}">
                {piece.rank}
              </span>
            </div>
            <p class="text-xs text-gray-500 mb-3">{slotLabel[piece.slot_type] ?? piece.slot_type}</p>
            <div class="grid grid-cols-2 gap-x-3 gap-y-1 text-xs">
              <div>
                <span class="text-gray-500">DEF</span>
                <span class="text-gray-100 font-medium ml-1">
                  {piece.defense_base ?? 0}-{piece.defense_max ?? 0}
                </span>
              </div>
              <div>
                <span class="text-gray-500">Rarity</span>
                <span class="text-gray-100 font-medium ml-1">{piece.rarity ?? 1}</span>
              </div>
              <div>
                <span class="text-orange-400">Fire</span>
                <span class="text-gray-100 ml-1">{piece.resistance_fire ?? 0}</span>
              </div>
              <div>
                <span class="text-blue-400">Water</span>
                <span class="text-gray-100 ml-1">{piece.resistance_water ?? 0}</span>
              </div>
              <div>
                <span class="text-yellow-400">Thunder</span>
                <span class="text-gray-100 ml-1">{piece.resistance_thunder ?? 0}</span>
              </div>
              <div>
                <span class="text-cyan-400">Ice</span>
                <span class="text-gray-100 ml-1">{piece.resistance_ice ?? 0}</span>
              </div>
              <div class="col-span-2">
                <span class="text-purple-400">Dragon</span>
                <span class="text-gray-100 ml-1">{piece.resistance_dragon ?? 0}</span>
              </div>
            </div>
          </Card>
        </button>
      {/each}
    </div>
  {/if}
</div>
