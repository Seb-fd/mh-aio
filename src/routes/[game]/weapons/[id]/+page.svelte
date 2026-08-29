<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { selectedGame } from '$lib/stores/game';
  import { api, type WeaponDetail, type Weapon } from '$lib/api';
  import DetailHeader from '$lib/components/detail-header.svelte';
  import MaterialList from '$lib/components/material-list.svelte';
  import { elementColor, sharpnessValues, SHARP_COLORS_ARR as SHARP_COLORS, SHARP_LABELS } from '$lib/utils/mh';

  const id = $derived(Number($page.params.id));
  let weapon = $state<WeaponDetail | null>(null);
  let allWeapons = $state<Weapon[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);

  $effect(() => {
    if (!id || Number.isNaN(id)) return;
    loading = true;
    error = null;
    let done = 0;
    const finish = () => { if (++done >= 2) loading = false; };
    api.getWeaponDetail(id)
      .then((data) => { weapon = data; })
      .catch((e) => { error = String(e); })
      .finally(finish);
    const g = $selectedGame;
    if (g?.dbId != null) {
      api.getWeapons(g.dbId)
        .then((data) => { allWeapons = data; })
        .catch(() => {})
        .finally(finish);
    } else { finish(); }
  });

  const byName = $derived(new Map(allWeapons.map(w => [w.name, w])));

  const baseWeapon = $derived<Weapon | null>(
    weapon?.upgrade_path ? (byName.get(weapon.upgrade_path) ?? null) : null,
  );

  interface TreeNode { weapon: Weapon; children: TreeNode[] }

  // Full subtree rooted at the current weapon's furthest ancestor (tree-style like weapon trees)
  const treeRoots = $derived.by<TreeNode[]>(() => {
    if (!weapon) return [];
    // walk up to the root
    let root: Weapon = weapon;
    let guard = 0;
    while (root.upgrade_path && guard++ < 40) {
      const parent = byName.get(root.upgrade_path);
      if (!parent) break;
      root = parent;
    }
    // build tree from root
    const childrenOf = new Map<string, Weapon[]>();
    for (const w of allWeapons) {
      if (!w.upgrade_path) continue;
      const arr = childrenOf.get(w.upgrade_path) ?? [];
      arr.push(w);
      childrenOf.set(w.upgrade_path, arr);
    }
    const build = (w: Weapon): TreeNode => ({
      weapon: w,
      children: (childrenOf.get(w.name) ?? [])
        .sort((a, b) => (a.attack ?? 0) - (b.attack ?? 0))
        .map(build),
    });
    return [build(root)];
  });

  const isUpgrade = $derived(!!weapon?.upgrade_path);

  // Helpers now from $lib/utils/mh (DRY)
  const sharpnessSegments = sharpnessValues;
  const sharpTotal = $derived(sharpnessSegments(weapon?.sharpness).reduce((a, b) => a + b, 0));

  function openWeapon(wid: number | null | undefined) {
    if (wid == null) return;
    const gameId = $page.params.game;
    if (gameId) goto(`/${gameId}/weapons/${wid}`);
  }
</script>

<div class="max-w-5xl mx-auto">
  {#if loading}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Loading weapon...</p>
    </div>
  {:else if error}
    <div class="bg-red-950/30 border border-red-900 rounded-lg p-8 text-center">
      <p class="text-red-400">Failed to load weapon</p>
      <p class="text-gray-500 text-sm mt-2">{error}</p>
    </div>
  {:else if !weapon}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Weapon not found</p>
    </div>
  {:else}
    <DetailHeader
      title={weapon.name}
      subtitle={weapon.weapon_type}
      icon="⚔️"
      tags={[
        { label: `Rarity ${weapon.rarity ?? 1}`, color: 'bg-[var(--theme-bg-elevated)] text-gray-300 border-[var(--theme-border)]' },
        ...(weapon.element_type && weapon.element_type !== ''
          ? [{ label: `${weapon.element_type} ${weapon.element_value ?? 0}`, color: `bg-[var(--theme-bg-elevated)] ${elementColor(weapon.element_type)} border-[var(--theme-border-strong)]` }]
          : []),
      ]}
    />

    <div class="grid grid-cols-2 sm:grid-cols-4 gap-3 mb-8">
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-500">Attack</p>
        <p class="text-2xl font-bold text-gray-100 mt-1">{weapon.attack ?? 0}</p>
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-500">Affinity</p>
        <p class="text-2xl font-bold mt-1" class:text-emerald-400={(weapon.affinity ?? 0) > 0} class:text-red-400={(weapon.affinity ?? 0) < 0} class:text-gray-100={(weapon.affinity ?? 0) === 0}>
          {weapon.affinity ?? 0}%
        </p>
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-500">Element</p>
        <p class="text-lg font-bold mt-1 {elementColor(weapon.element_type)}">
          {weapon.element_type ? `${weapon.element_type} ${weapon.element_value ?? 0}` : '—'}
        </p>
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-500">Status</p>
        <p class="text-lg font-bold mt-1 {weapon.status_type ? 'text-fuchsia-300' : 'text-gray-500'}">
          {weapon.status_type ? `${weapon.status_type} ${weapon.status_value ?? 0}` : '—'}
        </p>
      </div>
    </div>

    <div class="grid grid-cols-2 sm:grid-cols-4 gap-3 mb-8">
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-500">Cost</p>
        <p class="text-lg font-bold mt-1" style="color: var(--theme-accent);">{weapon.crafting_cost ?? 0}z</p>
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-500">Slots</p>
        <p class="text-lg font-bold text-gray-100 mt-1">{weapon.slots ?? '0'}</p>
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-500">Defense Bonus</p>
        <p class="text-lg font-bold text-gray-100 mt-1">{weapon.defense_bonus ?? 0}</p>
      </div>
      {#if weapon.upgrade_path}
        <div class="rounded-lg border themed-card p-3 text-center col-span-2 sm:col-span-1 flex flex-col justify-center overflow-hidden">
          <p class="text-[10px] uppercase tracking-wide text-gray-500">Upgraded From</p>
          <p class="text-xs font-semibold text-gray-200 mt-1 truncate px-2">{weapon.upgrade_path}</p>
        </div>
      {/if}
    </div>

    <section class="mb-8">
      <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-3">Upgrade Tree</h2>
      <div class="rounded-lg border themed-card p-4">
        <div class="flex flex-wrap items-center gap-2 mb-4">
          {#if weapon.is_forgeable}
            <span
              class="px-2.5 py-1 rounded-md text-xs font-medium border"
              style="background-color: color-mix(in oklab, #22c55e 16%, var(--theme-bg-elevated)); border-color: color-mix(in oklab, #22c55e 45%, transparent); color: #4ade80;"
            >
              🛠️ Forgeable directly
            </span>
          {/if}
          {#if isUpgrade}
            <button
              onclick={() => openWeapon(baseWeapon?.id)}
              class="px-2.5 py-1 rounded-md text-xs font-medium border cursor-pointer transition-all hover:border-[var(--theme-border-strong)]"
              style="background-color: color-mix(in oklab, var(--theme-accent) 16%, var(--theme-bg-elevated)); border-color: color-mix(in oklab, var(--theme-accent) 45%, transparent); color: var(--theme-accent);"
            >
              ⬆ Crafted from {baseWeapon?.name ?? weapon.upgrade_path ?? ''}
            </button>
          {/if}
        </div>
        {#each treeRoots as rootNode}
          {@render treeNode(rootNode, 0)}
        {/each}
      </div>
    </section>

    {#if sharpTotal > 0}
      <section class="mb-8">
        <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-3">Sharpness</h2>
        <div class="rounded-lg border themed-card p-4">
          <div class="flex items-center gap-[2px] h-4">
            {#each sharpnessSegments(weapon.sharpness) as seg, i}
              {#if seg > 0}
                <div
                  class="rounded-[2px]"
                  style="height: 14px; width: {seg * 3}px; background: {SHARP_COLORS[i]}; box-shadow: inset 0 -3px 0 rgba(0,0,0,0.25);"
                ></div>
              {/if}
            {/each}
          </div>
          <div class="flex flex-wrap gap-x-4 gap-y-1 mt-3">
            {#each sharpnessSegments(weapon.sharpness) as seg, i}
              {#if seg > 0}
                <div class="flex items-center gap-1.5">
                  <span class="inline-block w-2.5 h-2.5 rounded-sm" style="background: {SHARP_COLORS[i]};"></span>
                  <span class="text-xs text-gray-300">{SHARP_LABELS[i]}</span>
                  <span class="text-xs text-gray-500 tabular-nums">{seg}</span>
                </div>
              {/if}
            {/each}
          </div>
        </div>
      </section>
    {/if}

    {#if weapon.description}
      <section class="mb-8">
        <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-3">Description</h2>
        <div class="rounded-lg border themed-card p-5 leading-relaxed text-gray-200 text-[15px]">
          {weapon.description}
        </div>
      </section>
    {/if}

    {#if weapon.forge_materials.length > 0}
      <section class="mb-8">
        <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-3">🛠️ Forge (Direct) Materials</h2>
        <div class="rounded-lg border themed-card p-4">
          <MaterialList materials={weapon.forge_materials} showCraftingCost={false} />
        </div>
      </section>
    {/if}

    {#if weapon.upgrade_materials.length > 0}
      <section class="mb-8">
        <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-3">⬆ Upgrade Materials {isUpgrade ? '(from ' + (baseWeapon?.name ?? weapon.upgrade_path ?? '') + ')' : ''}</h2>
        <div class="rounded-lg border themed-card p-4">
          <MaterialList materials={weapon.upgrade_materials} showCraftingCost={false} />
        </div>
      </section>
    {/if}

    {#if weapon.forge_materials.length === 0 && weapon.upgrade_materials.length === 0}
      <section>
        <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-3">Crafting Materials</h2>
        <MaterialList materials={weapon.materials} showCraftingCost={false} />
      </section>
    {/if}
  {/if}
</div>

{#snippet treeNode(node: TreeNode, depth: number)}
  <div class="flex items-center gap-2 mb-1.5" style="margin-left: {depth * 18}px">
    {#if depth > 0}
      <span class="text-gray-700 text-xs shrink-0">└─</span>
    {:else}
      <span class="text-gray-700 text-xs shrink-0">├─</span>
    {/if}
    <button
      onclick={() => openWeapon(node.weapon.id)}
      class="flex-1 text-left px-3 py-2 rounded-lg border transition-all cursor-pointer"
      style={node.weapon.id === weapon?.id
        ? 'background-color: color-mix(in oklab, var(--theme-accent) 20%, var(--theme-bg-elevated)); border-color: color-mix(in oklab, var(--theme-accent) 60%, transparent); color: var(--theme-accent); font-weight: 600;'
        : 'border-color: var(--theme-border); background-color: var(--theme-bg-surface); color: rgb(209 213 219);'}
    >
      <div class="flex items-center gap-2 min-w-0">
        <span class="text-[10px] shrink-0 w-9 text-center rounded py-0.5 border border-[var(--theme-border)]" style="color: rgb(156 163 175);">{node.weapon.rarity ?? 1}</span>
        {#if node.weapon.is_forgeable}
          <span class="text-[12px] shrink-0" title="Crafted directly from materials">🛠️</span>
        {/if}
        <span class="text-sm font-medium truncate">{node.weapon.attack ?? 0} · {node.weapon.name}</span>
        {#if node.weapon.element_type}
          <span class="text-[11px] {elementColor(node.weapon.element_type)} shrink-0">{node.weapon.element_type} {node.weapon.element_value ?? 0}</span>
        {/if}
      </div>
    </button>
  </div>
  {#if node.children.length > 0}
    {#each node.children as child}
      {@render treeNode(child, depth + 1)}
    {/each}
  {/if}
{/snippet}
