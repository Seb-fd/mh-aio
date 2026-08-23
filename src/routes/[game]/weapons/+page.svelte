<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { selectedGame } from '$lib/stores/game';
  import { api, type Weapon } from '$lib/api';
  import Card from '$lib/components/ui/card.svelte';

  const game = $derived($selectedGame);
  const dbId = $derived(game?.dbId);

  let weapons = $state<Weapon[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let typeFilter = $state<string>('all');

  $effect(() => {
    if (dbId == null) return;
    loading = true;
    error = null;
    api.getWeapons(dbId)
      .then((data) => {
        weapons = data;
      })
      .catch((e) => {
        error = String(e);
      })
      .finally(() => {
        loading = false;
      });
  });

  const weaponTypes = $derived(['all', ...Array.from(new Set(weapons.map(w => w.weapon_type)))]);
  const filtered = $derived(
    typeFilter === 'all' ? weapons : weapons.filter(w => w.weapon_type === typeFilter)
  );

  function open(id: number) {
    if (!game) return;
    goto(`/${game.id}/weapons/${id}`);
  }

  function elementColor(elem: string | null): string {
    if (!elem) return 'text-gray-500';
    const lower = elem.toLowerCase();
    if (lower === 'fire') return 'text-orange-400';
    if (lower === 'water') return 'text-blue-400';
    if (lower === 'thunder') return 'text-yellow-400';
    if (lower === 'ice') return 'text-cyan-400';
    if (lower === 'dragon') return 'text-purple-400';
    return 'text-gray-400';
  }
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-6">
    <h1 class="text-2xl font-bold text-gray-100">Weapons</h1>
    <p class="text-sm text-gray-500 mt-1">
      {#if game}
        {game.shortName} · {weapons.length} weapons · Stats, elements and upgrade tree
      {:else}
        Select a game first
      {/if}
    </p>
  </div>

  {#if loading}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Loading weapons...</p>
    </div>
  {:else if error}
    <div class="bg-red-950/30 border border-red-900 rounded-lg p-8 text-center">
      <p class="text-red-400">Failed to load weapons</p>
      <p class="text-gray-500 text-sm mt-2">{error}</p>
    </div>
  {:else if weapons.length === 0}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">No weapons found for {game?.shortName ?? 'this game'}</p>
    </div>
  {:else}
    <div class="flex flex-wrap gap-2 mb-4">
      {#each weaponTypes as type}
        <button
          onclick={() => (typeFilter = type)}
          class="px-3 py-1.5 text-xs rounded-full border transition-colors"
          style={typeFilter === type
            ? `background-color: color-mix(in oklab, var(--theme-accent) 12%, transparent); border-color: color-mix(in oklab, var(--theme-accent) 50%, transparent); color: var(--theme-accent);`
            : `background-color: var(--theme-bg-surface); border-color: var(--theme-border); color: rgb(156 163 175);`}
        >
          {type === 'all' ? 'All' : type}
        </button>
      {/each}
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
      {#each filtered as weapon}
        <button onclick={() => open(weapon.id)} class="text-left">
          <Card class="p-4 border transition-all cursor-pointer hover:scale-[1.02] themed-card">
            <div class="flex items-start justify-between gap-2 mb-3">
              <h3 class="font-semibold text-gray-100 truncate">{weapon.name}</h3>
              <span class="text-[10px] uppercase tracking-wide text-gray-500 bg-[var(--theme-bg-elevated)] px-2 py-0.5 rounded shrink-0 border border-[var(--theme-border)]">
                R{weapon.rarity ?? 1}
              </span>
            </div>
            <p class="text-xs text-gray-500 mb-3">{weapon.weapon_type}</p>
            <div class="grid grid-cols-2 gap-2 text-xs">
              <div>
                <span class="text-gray-500">ATK</span>
                <span class="text-gray-100 font-medium ml-1">{weapon.attack ?? 0}</span>
              </div>
              <div>
                <span class="text-gray-500">AFF</span>
                <span class="text-gray-100 font-medium ml-1">
                  {weapon.affinity ?? 0}%
                </span>
              </div>
              {#if weapon.element_type && weapon.element_type !== ''}
                <div class="col-span-2">
                  <span class="text-gray-500">Element</span>
                  <span class="{elementColor(weapon.element_type)} font-medium ml-1">
                    {weapon.element_type} {weapon.element_value ?? 0}
                  </span>
                </div>
              {/if}
            </div>
          </Card>
        </button>
      {/each}
    </div>
  {/if}
</div>
