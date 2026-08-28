<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { api, type CombineView } from '$lib/api';
  import { selectedGame } from '$lib/stores/game';
  import { normKey } from '$lib/utils/norm';

  const game = $derived($selectedGame);
  const dbId = $derived(game?.dbId);

  let combines = $state<CombineView[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let filterType = $state<string>('all'); // all | normal | alchemy | treasure
  let searchTerm = $state('');

  $effect(() => {
    if (dbId == null) return;
    loading = true;
    error = null;
    api.getCombinations(dbId)
      .then((data) => {
        combines = data;
      })
      .catch((e) => {
        error = String(e);
      })
      .finally(() => {
        loading = false;
      });
  });

  const filtered = $derived.by(() => {
    let arr = combines;
    if (filterType !== 'all') {
      arr = arr.filter(c => c.combine_type === filterType);
    }
    if (searchTerm !== '') {
      const term = normKey(searchTerm);
      arr = arr.filter(c =>
        normKey(c.result_name).includes(term) ||
        c.components.some(comp => normKey(comp.component_name).includes(term))
      );
    }
    return arr;
  });

  const counts = $derived.by(() => {
    const c = { all: combines.length, normal: 0, alchemy: 0, treasure: 0 };
    for (const r of combines) {
      if (r.combine_type === 'normal') c.normal++;
      else if (r.combine_type === 'alchemy') c.alchemy++;
      else if (r.combine_type === 'treasure') c.treasure++;
    }
    return c;
  });

  function openResult(id: number) {
    if (!game) return;
    goto(`/${game.id}/items/${id}`);
  }
  function openComponent(id: number) {
    if (!game) return;
    goto(`/${game.id}/items/${id}`);
  }

  const typeBadge: Record<string, { label: string; cls: string }> = {
    normal: { label: 'Normal', cls: 'bg-sky-900/30 text-sky-300 border-sky-800' },
    alchemy: { label: 'Alchemy', cls: 'bg-amber-900/30 text-amber-300 border-amber-800' },
    treasure: { label: 'Treasure', cls: 'bg-purple-900/30 text-purple-300 border-purple-800' },
  };
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-6">
    <div class="flex items-center gap-2">
      <button
        onclick={() => { if (game) goto(`/${game.id}/items`); }}
        class="text-xs px-2 py-1 rounded border border-[var(--theme-border)] text-gray-400 hover:text-gray-200 hover:border-[var(--theme-border-strong)] transition-colors"
      >
        ← Items
      </button>
      <h1 class="text-2xl font-bold text-gray-100">Combinations</h1>
      <span class="text-xs px-2 py-0.5 rounded-full border bg-[var(--theme-bg-surface)] text-gray-400 border-[var(--theme-border)]">{combines.length} recipes</span>
    </div>
    <p class="text-sm text-gray-500 mt-1">
      {#if game}
        {game.shortName} · Book order (game) · Normal + Alchemy
      {:else}
        Select a game first
      {/if}
    </p>
    {#if filtered.length > 0}
      <p class="text-[11px] text-gray-600 mt-1">💡 All recipes sorted as in the in-game Combination List (Book of Combos 1-5 + Alchemy Guide)</p>
    {/if}
  </div>

  {#if loading}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Loading combinations...</p>
    </div>
  {:else if error}
    <div class="bg-red-950/30 border border-red-900 rounded-lg p-8 text-center">
      <p class="text-red-400">Failed to load combinations</p>
      <p class="text-gray-500 text-sm mt-2">{error}</p>
    </div>
  {:else}
    <div class="flex flex-wrap gap-2 mb-4 items-center">
      <input
        type="text"
        bind:value={searchTerm}
        placeholder="Search result or ingredient..."
        class="px-3 py-1.5 text-sm bg-[var(--theme-bg-surface)] border border-[var(--theme-border)] rounded-lg text-gray-100 placeholder-gray-600 focus:outline-none focus:border-[var(--theme-border-strong)] w-64"
      />
      {#each [
        { key: 'all', label: `All (${counts.all})` },
        { key: 'normal', label: `Normal (${counts.normal})` },
        { key: 'alchemy', label: `⚗️ Alchemy (${counts.alchemy})` },
        { key: 'treasure', label: `Treasure (${counts.treasure})` },
      ] as f}
        <button
          onclick={() => (filterType = f.key)}
          class="px-3 py-1.5 text-xs rounded-full border transition-colors"
          style={filterType === f.key
            ? `background-color: color-mix(in oklab, var(--theme-accent) 12%, transparent); border-color: color-mix(in oklab, var(--theme-accent) 50%, transparent); color: var(--theme-accent);`
            : `background-color: var(--theme-bg-surface); border-color: var(--theme-border); color: rgb(156 163 175);`}
        >
          {f.label}
        </button>
      {/each}
    </div>

    {#if filtered.length === 0}
      <div class="border rounded-lg p-8 text-center themed-card">
        <p class="text-gray-400">No combinations match your filter</p>
      </div>
    {:else}
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-3">
        {#each filtered as rec}
          {@const badge = typeBadge[rec.combine_type] ?? { label: rec.combine_type, cls: 'bg-gray-800 text-gray-300 border-gray-700' }}
          <div class="rounded-lg border themed-card p-4 flex flex-col gap-2">
            <div class="flex items-center gap-2 flex-wrap">
              <button
                onclick={() => openResult(rec.result_item_id)}
                class="text-sm font-semibold text-gray-100 hover:text-[var(--theme-accent)] transition-colors text-left"
              >
                {rec.result_name}
              </button>
              <span class="text-[10px] px-2 py-0.5 rounded-full border font-semibold {badge.cls}">{badge.label}</span>
              {#if rec.chance != null}
                <span class="text-[10px] px-1.5 py-0.5 rounded border bg-[var(--theme-bg-elevated)] text-gray-400 border-[var(--theme-border)]">{rec.chance}% success</span>
              {/if}
              <span class="text-[10px] text-gray-600 ml-auto">#{rec.result_item_id}</span>
            </div>
            <div class="flex flex-wrap items-center gap-1.5">
              {#each rec.components as comp, i}
                {#if i > 0}
                  <span class="text-gray-600 text-sm">+</span>
                {/if}
                <button
                  onclick={() => openComponent(comp.component_item_id)}
                  class="px-2.5 py-1 rounded-md bg-[var(--theme-bg-elevated)] border border-[var(--theme-border)] flex items-center gap-1.5 hover:border-[var(--theme-border-strong)] hover:bg-[var(--theme-bg-surface)] transition-colors"
                  title="Go to {comp.component_name}"
                >
                  <span class="text-xs text-gray-200">{comp.component_name}</span>
                  <span class="text-[10px] font-semibold" style="color: var(--theme-accent);">x{comp.quantity}</span>
                </button>
              {/each}
              <span class="text-gray-600 text-sm">=</span>
              <button
                onclick={() => openResult(rec.result_item_id)}
                class="px-2.5 py-1 rounded-md flex items-center gap-1.5 hover:opacity-90 transition-opacity"
                style="background-color: color-mix(in oklab, var(--theme-accent) 15%, var(--theme-bg-elevated)); border: 1px solid color-mix(in oklab, var(--theme-accent) 40%, transparent);"
              >
                <span class="text-xs text-gray-100">{rec.result_name}</span>
                <span class="text-[10px] font-semibold" style="color: var(--theme-accent);">x{rec.components[0]?.result_quantity ?? 1}</span>
              </button>
            </div>
            {#if rec.combine_type === 'alchemy'}
              <p class="text-[10px] text-amber-400/60">※ Requires Alchemy Guide (progressively unlocks with Books 1-5)</p>
            {/if}
          </div>
        {/each}
      </div>
    {/if}
  {/if}
</div>
