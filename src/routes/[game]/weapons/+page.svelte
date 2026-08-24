<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { selectedGame } from '$lib/stores/game';
  import { api, type Weapon } from '$lib/api';

  const game = $derived($selectedGame);
  const dbId = $derived(game?.dbId);

  let weapons = $state<Weapon[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let typeFilter = $state<string>('all');

  async function loadWeapons(id: number, attempt = 0) {
    try {
      const data = await api.getWeapons(id);
      console.log('[weapons] loaded', data.length);
      weapons = data;
      error = null;
    } catch (e) {
      const msg = String(e);
      console.error('[weapons] failed', msg);
      if (msg.includes('state not managed') && attempt < 6) {
        error = 'Preparing database...';
        setTimeout(() => loadWeapons(id, attempt + 1), 400 * (attempt + 1));
        return;
      }
      error = msg;
    } finally {
      if (error !== 'Preparing database...') loading = false;
    }
  }

  $effect(() => {
    if (dbId == null) return;
    console.log('[weapons] loading gameId', dbId);
    loading = true;
    error = null;
    loadWeapons(dbId);
  });

  const weaponTypes = $derived(['all', ...Array.from(new Set(weapons.map(w => w.weapon_type)))]);
  const filtered = $derived(
    typeFilter === 'all' ? weapons : weapons.filter(w => w.weapon_type === typeFilter)
  );

  interface TreeNode { weapon: Weapon; children: TreeNode[] }

  function buildForest(typeWeapons: Weapon[]): TreeNode[] {
    const set = new Set(typeWeapons.map(w => w.name));
    const childrenOf = new Map<string, Weapon[]>();
    for (const w of typeWeapons) {
      if (!w.upgrade_path) continue;
      const arr = childrenOf.get(w.upgrade_path) ?? [];
      arr.push(w);
      childrenOf.set(w.upgrade_path, arr);
    }
    const roots = typeWeapons.filter(w => !w.upgrade_path || !set.has(w.upgrade_path));
    const build = (w: Weapon): TreeNode => ({
      weapon: w,
      children: (childrenOf.get(w.name) ?? []).sort((a, b) => (b.attack ?? 0) - (a.attack ?? 0)).map(build),
    });
    return roots.sort((a, b) => (a.attack ?? 0) - (b.attack ?? 0)).map(build);
  }

  const tree = $derived.by<{ type: string; forests: TreeNode[] }[]>(() => {
    const byType = new Map<string, Weapon[]>();
    for (const w of filtered) {
      const arr = byType.get(w.weapon_type) ?? [];
      arr.push(w);
      byType.set(w.weapon_type, arr);
    }
    return [...byType.entries()].map(([type, ws]) => ({ type, forests: buildForest(ws) }));
  });

  const allCount = $derived(tree.reduce((n, t) => n + countForests(t.forests), 0));
  function countForests(f: TreeNode[]): number {
    return f.reduce((n, node) => n + 1 + countForests(node.children), 0);
  }

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

  const SHARP_COLORS = ['#e74c3c', '#ff9800', '#f4d03f', '#58d68d', '#5dade2', '#ffffff'];
  const SHARP_LABELS = ['Red', 'Orange', 'Yellow', 'Green', 'Blue', 'White'];
  function sharpnessSegments(raw: string | null | undefined): number[] {
    if (!raw) return [];
    try {
      const a = JSON.parse(raw);
      return (Array.isArray(a) ? a.map(Number) : []).slice(0, 6);
    } catch {
      return [];
    }
  }
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-6">
    <h1 class="text-2xl font-bold text-gray-100">Weapon Trees</h1>
    <p class="text-sm text-gray-500 mt-1">
      {#if game}
        {game.shortName} · {weapons.length} weapons · craft + upgrade tree
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
    <div class="flex flex-wrap gap-2 mb-6">
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

    {#each tree as group}
      <section class="mb-10">
        <h2 class="text-sm font-semibold uppercase tracking-wider text-gray-400 mb-4">{group.type}</h2>
        {#each group.forests as node}
          {@render treeNode(node, 0)}
        {/each}
      </section>
    {/each}
  {/if}
</div>

{#snippet treeNode(node: TreeNode, depth: number)}
  <div class="mb-1.5" style="margin-left: {depth * 18}px">
    <div class="flex items-center gap-2">
      <button
        onclick={() => open(node.weapon.id)}
        class="flex-1 text-left px-3 py-2 rounded-lg border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] hover:border-[var(--theme-border-strong)] hover:bg-[var(--theme-bg-elevated)] transition-all"
      >
        <div class="flex items-center gap-2 min-w-0">
          {#if node.weapon.is_forgeable}
            <span class="text-[12px] shrink-0" title="Crafted directly from materials">🛠️</span>
          {/if}
          <span class="text-[10px] text-gray-500 shrink-0 w-10 text-center rounded bg-[var(--theme-bg-elevated)] py-0.5 border border-[var(--theme-border)]">R{node.weapon.rarity ?? 1}</span>
          <span class="text-sm text-gray-100 font-medium truncate">{node.weapon.name}</span>
          {#if node.weapon.element_type}
            <span class="text-[11px] {elementColor(node.weapon.element_type)} shrink-0">{node.weapon.element_type} {node.weapon.element_value ?? 0}</span>
          {/if}
          <span class="text-[11px] text-gray-500 ml-auto shrink-0">ATK {node.weapon.attack ?? 0}</span>
        </div>
        {#if sharpnessSegments(node.weapon.sharpness).length > 0}
          <div class="flex items-center gap-[1px] mt-1.5 h-1.5">
            {#each sharpnessSegments(node.weapon.sharpness) as seg, i}
              {#if seg > 0}
                <div class="rounded-[1px]" style="height: 6px; width: {seg}px; background: {SHARP_COLORS[i] ?? '#666'};"></div>
              {/if}
            {/each}
          </div>
        {/if}
      </button>
    </div>
    {#if node.children.length > 0}
      <div class="border-l border-[var(--theme-border)] ml-4 mt-1.5 pl-2">
        {#each node.children as child}
          {@render treeNode(child, 0)}
        {/each}
      </div>
    {/if}
  </div>
{/snippet}
