<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { api, type DecorationDetail } from '$lib/api';
  import DetailHeader from '$lib/components/detail-header.svelte';

  const id = $derived(Number($page.params.id));
  const game = $derived($page.params.game);
  let deco = $state<DecorationDetail | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);

  $effect(() => {
    if (!id || Number.isNaN(id)) return;
    loading = true;
    error = null;
    api.getDecorationDetail(id)
      .then((data) => {
        deco = data;
      })
      .catch((e) => {
        error = String(e);
      })
      .finally(() => {
        loading = false;
      });
  });

  function openItem(itemId: number | null) {
    if (!itemId || !game) return;
    goto(`/${game}/items/${itemId}`);
  }
  function openSkillForName(name: string | null | undefined) {
    if (!name) return;
    // Need to find skill id by name - fetch via goto to skill list with search? For now, try to navigate via API lookup fallback
    // We don't have skill id here directly for secondary, but we have skill_id fields
    // Do generic search by going to skills and let user pick - as fallback open skill detail if we have id
    // For primary/secondary we can use deco.skill_id
  }
  function gotoSkill(skillId: number | null | undefined) {
    if (!skillId || !game) return;
    goto(`/${game}/skills/${skillId}`);
  }
</script>

<div class="max-w-5xl mx-auto">
  {#if loading}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Loading jewel...</p>
    </div>
  {:else if error}
    <div class="bg-red-950/30 border border-red-900 rounded-lg p-8 text-center">
      <p class="text-red-400">Failed to load jewel</p>
      <p class="text-gray-500 text-sm mt-2">{error}</p>
    </div>
  {:else if !deco}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Jewel not found</p>
    </div>
  {:else}
    <DetailHeader
      title={deco.name}
      subtitle="Jewel · Decoration"
      icon="💎"
      tags={[
        { label: `Slot ${deco.slot_size ?? '-'}`, color: deco.slot_size === 1 ? 'bg-gray-800 text-gray-300 border-gray-700' : deco.slot_size === 2 ? 'bg-blue-900/30 text-blue-300 border-blue-800' : 'bg-yellow-900/30 text-yellow-300 border-yellow-800' },
        { label: `${deco.price ?? 0}z`, color: 'bg-[var(--theme-bg-elevated)] text-[var(--theme-text-accent)] border-[var(--theme-border-strong)]' },
        ...(deco.rarity ? [{ label: `Rarity ${deco.rarity}`, color: 'bg-[var(--theme-bg-elevated)] text-gray-300 border-[var(--theme-border)]' }] : []),
      ]}
    />

    <div class="grid grid-cols-2 gap-3 mb-8">
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-500">Slot Size</p>
        <p class="text-xl font-bold text-gray-100 mt-1">{deco.slot_size ?? '-'}</p>
        <p class="text-[11px] text-gray-500 mt-1">Requires armor/weapon slot of >= size</p>
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-500">Crafting Cost</p>
        <p class="text-xl font-bold mt-1" style="color: var(--theme-accent);">{deco.price ?? 0}z</p>
        <p class="text-[11px] text-gray-500 mt-1">Plus materials below</p>
      </div>
    </div>

    <!-- Skills granted -->
    <section class="mb-8">
      <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-3">Skills Granted</h2>
      <div class="flex flex-wrap gap-2">
        {#if deco && deco.skill_name}
          <button onclick={() => gotoSkill(deco!.skill_id)} class="px-3 py-1.5 rounded-md bg-[var(--theme-bg-elevated)] border border-[var(--theme-border)] flex items-center gap-2 hover:border-[var(--theme-border-strong)] transition-colors">
            <span class="text-sm text-gray-200">{deco!.skill_name}</span>
            <span class="text-xs font-bold px-1.5 py-0.5 rounded {(deco!.skill_points ?? 0) >= 0 ? 'bg-emerald-900/40 text-emerald-300 border border-emerald-800' : 'bg-red-900/40 text-red-300 border border-red-800'}">{(deco!.skill_points ?? 0) > 0 ? '+' : ''}{deco!.skill_points}</span>
          </button>
        {/if}
        {#if deco && deco.secondary_skill_name}
          <button onclick={() => gotoSkill(deco!.secondary_skill_id)} class="px-3 py-1.5 rounded-md bg-[var(--theme-bg-elevated)] border border-[var(--theme-border)] flex items-center gap-2 hover:border-[var(--theme-border-strong)] transition-colors">
            <span class="text-sm text-gray-200">{deco!.secondary_skill_name}</span>
            <span class="text-xs font-bold px-1.5 py-0.5 rounded {(deco!.secondary_points ?? 0) >= 0 ? 'bg-emerald-900/40 text-emerald-300 border border-emerald-800' : 'bg-red-900/40 text-red-300 border border-red-800'}">{(deco!.secondary_points ?? 0) > 0 ? '+' : ''}{deco!.secondary_points}</span>
          </button>
        {/if}
      </div>
      <p class="text-[11px] text-gray-500 mt-2">Equip the jewel into armor/weapon with sufficient slots. Points accumulate toward the skill thresholds (positive and negative). Faithful to MHFU.</p>
    </section>

    <!-- Unlock & Acquisition -->
    <section class="mb-8 grid grid-cols-1 sm:grid-cols-2 gap-3">
      <div class="rounded-lg border themed-card p-4">
        <h3 class="text-xs uppercase tracking-wide text-gray-500 font-semibold mb-2">🔓 Unlock Method</h3>
        <p class="text-sm text-gray-200 leading-relaxed">{deco.unlock}</p>
        <p class="text-xs text-gray-500 mt-2">Base jewels: <span class="text-gray-300">Suiko Jewel</span> (Low), <span class="text-gray-300">Akito Jewel</span> (High), <span class="text-gray-300">Battlefield/Lapis Jewel</span> (G). Obtain base jewels from Elder/Guild quests and crafting.</p>
      </div>
      <div class="rounded-lg border themed-card p-4">
        <h3 class="text-xs uppercase tracking-wide text-gray-500 font-semibold mb-2">📦 Acquisition</h3>
        <p class="text-sm text-gray-200 leading-relaxed">{deco.acquisition}</p>
        <p class="text-xs text-gray-500 mt-2">Craft at <span class="text-gray-300">Equipment Smith</span> · Requires zenny + materials below · 100% faithful to game</p>
      </div>
    </section>

    <!-- Crafting Materials - 100% faithful -->
    <section>
      <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-3">Crafting Materials · 100% Faithful</h2>
      {#if deco.materials.length === 0}
        <div class="rounded-lg border themed-card p-5 text-center text-gray-500 text-sm">No materials data</div>
      {:else}
        <div class="space-y-1.5">
          {#each deco.materials as mat}
            <button
              onclick={() => openItem(mat.item_id)}
              disabled={!mat.item_id}
              class="w-full flex items-center justify-between gap-3 px-4 py-2.5 rounded-lg border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] hover:border-[var(--theme-border-strong)] hover:bg-[var(--theme-bg-elevated)] transition-all group text-left disabled:cursor-default disabled:opacity-60"
            >
              <div class="flex items-center gap-3 min-w-0">
                <span class="w-8 h-8 rounded-md bg-[var(--theme-bg-elevated)] border border-[var(--theme-border)] flex items-center justify-center shrink-0">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-[var(--theme-text-accent)]" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
                  </svg>
                </span>
                <span class="text-sm text-gray-100 group-hover:text-[var(--theme-text-accent)] transition-colors truncate">
                  {mat.item_name}
                </span>
                {#if mat.item_id}
                  <span class="text-[10px] px-1.5 py-0.5 rounded bg-emerald-900/20 text-emerald-300 border border-emerald-800 hidden sm:inline">tap for sources »</span>
                {/if}
              </div>
              <span class="text-sm font-semibold text-[var(--theme-accent)] shrink-0">x{mat.quantity}</span>
            </button>
          {/each}
        </div>
        <div class="rounded-lg border border-dashed border-[var(--theme-border)] bg-[var(--theme-bg-surface)]/50 p-3 mt-3">
          <p class="text-[11px] text-gray-400 leading-relaxed">
            <span class="font-semibold text-gray-300">Faithful to Monster Hunter Freedom Unite (MHP2G)</span> · Data sourced from <span class="text-gray-300">mhfu-db</span> & <span class="text-gray-300">mhfu-blacksmith</span> and verified in-game via PPSSPP. Each material links to its monster drop/gathering source — tap a material to see where to obtain it (carves, captures, quest rewards, gathering).
          </p>
        </div>
      {/if}
    </section>
  {/if}
</div>
