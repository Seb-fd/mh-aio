<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { api, type MonsterDetail, type MonsterDrop, type ArmorSetDetail } from '$lib/api';
  import DetailHeader from '$lib/components/detail-header.svelte';
  import { selectedGame } from '$lib/stores/game';

  const id = $derived(Number($page.params.id));
  let monster = $state<MonsterDetail | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let dedicatedSets = $state<ArmorSetDetail[]>([]);
  let dedicatedLoading = $state(false);
  let armorViewMode = $state<'dedicated' | 'uses'>('dedicated'); // dedicated default (60% score), uses secondary

  $effect(() => {
    if (!id || Number.isNaN(id)) return;
    loading = true;
    error = null;
    api.getMonsterDetail(id)
      .then((data) => {
        monster = data;
      })
      .catch((e) => {
        error = String(e);
      })
      .finally(() => {
        loading = false;
      });
  });

  // Fetch dedicated armor sets (60% threshold, rank-filtered) whenever monster + rank changes
  $effect(() => {
    if (!monster || !id) return;
    dedicatedLoading = true;
    api.getMonsterDedicatedSets(id, activeRank)
      .then((data) => {
        dedicatedSets = data;
      })
      .catch(() => {
        dedicatedSets = [];
      })
      .finally(() => {
        dedicatedLoading = false;
      });
  });

  const methodLabel: Record<string, { label: string; icon: string; color: string }> = {
    carve: { label: 'Carve', icon: '⚔️', color: 'text-red-400' },
    capture: { label: 'Capture', icon: '🪤', color: 'text-emerald-400' },
    drop: { label: 'Shiny Drop', icon: '✨', color: 'text-yellow-400' },
    break: { label: 'Break Part', icon: '🔨', color: 'text-orange-400' },
  };

  const slotLabel: Record<string, string> = {
    head: 'Helm', chest: 'Mail', arms: 'Vambraces', waist: 'Coil', legs: 'Greaves',
  };

  const rankOrder = ['Low', 'High', 'G'];
  const methodOrder = ['carve', 'break', 'capture', 'drop'];
  function goToItem(drop: MonsterDrop) {
    if ($selectedGame) goto(`/${$selectedGame.id}/items/${drop.item_id}`);
  }

  function weaknessColor(value: number | null): string {
    if (value == null) return 'text-gray-600';
    if (value >= 25) return 'text-emerald-400';
    if (value >= 15) return 'text-yellow-400';
    if (value >= 5) return 'text-orange-400';
    if (value <= -10) return 'text-red-400';
    return 'text-gray-400';
  }

  function weaknessBg(value: number | null): string {
    if (value == null) return 'bg-gray-800/30';
    if (value >= 25) return 'bg-emerald-900/40';
    if (value >= 15) return 'bg-yellow-900/30';
    if (value >= 5) return 'bg-orange-900/30';
    if (value <= -10) return 'bg-red-900/40';
    return 'bg-gray-800/40';
  }

  function sortDrops(drops: MonsterDrop[]): MonsterDrop[] {
    return [...drops].sort((a, b) => {
      const ra = rankOrder.indexOf(a.rank ?? '');
      const rb = rankOrder.indexOf(b.rank ?? '');
      if (ra !== rb) return ra - rb;
      const ma = methodOrder.indexOf(a.method);
      const mb = methodOrder.indexOf(b.method);
      if (ma !== mb) return ma - mb;
      return b.probability - a.probability;
    });
  }

  const rankTabs: string[] = $derived(
    sortDrops(monster?.drops ?? []).reduce((acc, d) => {
      const r = d.rank ?? 'Low';
      if (!acc.includes(r)) acc.push(r);
      return acc;
    }, [] as string[]),
  );

  let activeRank = $state('Low');

  $effect(() => {
    if (rankTabs.length > 0 && !rankTabs.includes(activeRank)) {
      activeRank = rankTabs[0];
    }
  });

  const visibleDrops = $derived(
    sortDrops((monster?.drops ?? []).filter((d) => (d.rank ?? 'Low') === activeRank)),
  );
</script>

<div class="max-w-5xl mx-auto">
  {#if loading}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Loading monster...</p>
    </div>
  {:else if error}
    <div class="bg-red-950/30 border border-red-900 rounded-lg p-8 text-center">
      <p class="text-red-400">Failed to load monster</p>
      <p class="text-gray-500 text-sm mt-2">{error}</p>
    </div>
  {:else if !monster}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Monster not found</p>
    </div>
  {:else}
    <DetailHeader
      title={monster.name}
      subtitle={monster.species ?? ''}
      icon="🐉"
      tags={[
        { label: monster.size ?? 'Unknown', color: 'bg-[var(--theme-bg-elevated)] text-gray-300 border-[var(--theme-border)]' },
        { label: 'Monster', color: 'bg-[var(--theme-bg-elevated)] text-[var(--theme-text-accent)] border-[var(--theme-border-strong)]' },
      ]}
    />

    {#if monster.description}
      <section class="mb-8">
        <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-3">Description</h2>
        <div class="rounded-lg border themed-card p-5 leading-relaxed text-gray-200 text-[15px]">
          {monster.description}
        </div>
      </section>
    {/if}

    {#if monster.drops.length > 0}
      <section class="mb-8">
        <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-3">Material Drops</h2>

        <div class="flex gap-2 mb-4 flex-wrap">
          {#each rankTabs as rank}
            <button
              onclick={() => (activeRank = rank)}
              class="px-3 py-1.5 rounded-md text-xs font-medium border transition-all
                {rank === activeRank
                  ? 'bg-[var(--theme-accent)] text-white border-transparent'
                  : 'bg-[var(--theme-bg-elevated)] text-gray-300 border-[var(--theme-border)] hover:border-[var(--theme-border-strong)]'}"
            >
              {rank}
            </button>
          {/each}
        </div>

        <div class="space-y-3">
          {#each visibleDrops as drop}
            {@const meta = methodLabel[drop.method] ?? { label: drop.method, icon: '❓', color: 'text-gray-400' }}
            <button
              onclick={() => goToItem(drop)}
              class="w-full block px-4 py-3 rounded-lg border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] hover:border-[var(--theme-border-strong)] hover:bg-[var(--theme-bg-elevated)] transition-all text-left group"
            >
              <div class="flex items-center gap-3">
                <span class="text-lg shrink-0">{meta.icon}</span>
                <div class="flex-1 min-w-0">
                  <div class="flex items-center gap-2">
                    <span class="text-xs uppercase tracking-wide {meta.color} font-medium">{meta.label}</span>
                    {#if drop.part}
                      <span class="text-xs text-gray-500">· {drop.part}</span>
                    {/if}
                    <span class="text-sm text-gray-100 truncate group-hover:text-[var(--theme-text-accent)] transition-colors">
                      {drop.item_name}
                      <span class="text-xs text-gray-500"> x{drop.quantity}</span>
                    </span>
                  </div>
                  {#if drop.condition}
                    <p class="text-[11px] text-gray-500 mt-0.5">※ {drop.condition}</p>
                  {/if}
                  <div class="mt-2 flex items-center gap-2">
                    <div class="flex-1 h-1.5 rounded-full bg-[var(--theme-bg-elevated)] overflow-hidden">
                      <div
                        class="h-full bg-[var(--theme-accent)] rounded-full transition-all"
                        style="width: {Math.round(drop.probability * 100)}%"
                      ></div>
                    </div>
                    <span class="text-[10px] text-gray-400 shrink-0 tabular-nums w-9 text-right">
                      {Math.round(drop.probability * 100)}%
                    </span>
                  </div>
                </div>
              </div>
            </button>
          {/each}
        </div>
      </section>
    {/if}

    {#if monster.armor.length > 0 || monster.weapons.length > 0 || dedicatedSets.length > 0}
      <section class="mb-8">
        <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-3">Equipment · Rank filter: {activeRank} (unified with drops)</h2>
        <!-- Armor: Dedicated (default, 60% score) vs Uses 1 Material (secondary) -->
        {#if monster.armor.length > 0 || dedicatedSets.length > 0}
          <div class="flex items-center gap-2 mb-2">
            <h3 class="text-sm font-semibold text-gray-200">Armor</h3>
            <div class="flex rounded-full border border-[var(--theme-border)] overflow-hidden ml-2">
              <button onclick={() => (armorViewMode = 'dedicated')} class="px-3 py-1 text-[11px] font-medium {armorViewMode==='dedicated' ? 'bg-[var(--theme-primary)] text-white' : 'bg-[var(--theme-bg-surface)] text-gray-400'}">Dedicated ({dedicatedSets.length})</button>
              <button onclick={() => (armorViewMode = 'uses')} class="px-3 py-1 text-[11px] font-medium {armorViewMode==='uses' ? 'bg-[var(--theme-primary)] text-white' : 'bg-[var(--theme-bg-surface)] text-gray-400'}">Uses 1 Material ({monster.armor.length})</button>
            </div>
          </div>
          {#if armorViewMode === 'dedicated'}
            {#if dedicatedLoading}
              <p class="text-xs text-gray-500 mb-4">Loading dedicated sets (≥60% mats, exact monster, rank {activeRank})…</p>
            {:else if dedicatedSets.length === 0}
              <p class="text-xs text-gray-500 mb-4">No dedicated sets for {monster.name} at rank {activeRank} — try another rank or check “Uses 1 Material”.</p>
            {:else}
              <div class="grid grid-cols-1 lg:grid-cols-2 gap-3 mb-4">
                {#each dedicatedSets as set (set.id)}
                  <div class="rounded-lg border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] p-3">
                    <div class="flex items-center justify-between mb-2">
                      <span class="text-sm font-semibold text-gray-100">{set.name}</span>
                      <span class="text-[10px] px-2 py-0.5 rounded bg-[var(--theme-bg-elevated)] border border-[var(--theme-border)] text-gray-400">{set.pieces[0]?.rank ?? activeRank} · {set.pieces.length} pcs</span>
                    </div>
                    <div class="flex flex-wrap gap-1">
                      {#each set.pieces as piece (piece.id)}
                        <button
                          onclick={() => goto(`/${$selectedGame?.id ?? ''}/armor/${piece.id}`)}
                          class="text-[11px] px-2 py-1 rounded bg-[var(--theme-bg-elevated)] border border-[var(--theme-border)] text-gray-300 hover:border-[var(--theme-border-strong)]"
                        >
                          {piece.name} <span class="text-gray-500">[{slotLabel[piece.slot_type] ?? piece.slot_type}]</span>
                        </button>
                      {/each}
                    </div>
                  </div>
                {/each}
              </div>
            {/if}
          {:else}
            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2 mb-4">
              {#each monster.armor as piece (piece.id)}
                <button
                  onclick={() => goto(`/${$selectedGame?.id ?? ''}/armor/${piece.id}`)}
                  class="text-left px-3 py-2 rounded-lg border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] hover:border-[var(--theme-border-strong)] hover:bg-[var(--theme-bg-elevated)] transition-all"
                >
                  <div class="text-sm text-gray-100 font-medium truncate">{piece.name}</div>
                  <div class="text-[11px] text-gray-500 mt-0.5">{slotLabel[piece.slot_type] ?? piece.slot_type} · {piece.rank} · Def {piece.defense_base ?? 0}</div>
                </button>
              {/each}
            </div>
          {/if}
        {/if}
        {#if monster.weapons.length > 0}
          <h3 class="text-sm font-semibold text-gray-200 mb-2">Weapons</h3>
          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2">
            {#each monster.weapons as w (w.id)}
              <button
                onclick={() => goto(`/${$selectedGame?.id ?? ''}/weapons/${w.id}`)}
                class="text-left px-3 py-2 rounded-lg border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] hover:border-[var(--theme-border-strong)] hover:bg-[var(--theme-bg-elevated)] transition-all"
              >
                <div class="text-sm text-gray-100 font-medium truncate">{w.name}</div>
                <div class="text-[11px] text-gray-500 mt-0.5">{w.weapon_type} · R{w.rarity ?? 1} · Atk {w.attack ?? 0}</div>
              </button>
            {/each}
          </div>
        {/if}
      </section>
    {/if}

    {#if monster.weaknesses.length > 0}
      <section class="mb-8">
        <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-3">Weaknesses</h2>
        <div class="space-y-2">
          {#each monster.weaknesses as w}
            <div class="rounded-lg border themed-card p-4">
              <div class="flex items-center justify-between mb-3">
                <span class="font-semibold text-gray-100">{w.part_name}</span>
              </div>
              <div class="grid grid-cols-3 sm:grid-cols-8 gap-2 text-xs">
                <div class="px-2 py-1.5 rounded {weaknessBg(w.sever)} {weaknessColor(w.sever)} flex flex-col items-center">
                  <span class="text-[9px] uppercase opacity-70">Sever</span>
                  <span class="font-medium">{w.sever ?? '-'}</span>
                </div>
                <div class="px-2 py-1.5 rounded {weaknessBg(w.blunt)} {weaknessColor(w.blunt)} flex flex-col items-center">
                  <span class="text-[9px] uppercase opacity-70">Blunt</span>
                  <span class="font-medium">{w.blunt ?? '-'}</span>
                </div>
                <div class="px-2 py-1.5 rounded {weaknessBg(w.projectile)} {weaknessColor(w.projectile)} flex flex-col items-center">
                  <span class="text-[9px] uppercase opacity-70">Shot</span>
                  <span class="font-medium">{w.projectile ?? '-'}</span>
                </div>
                <div class="px-2 py-1.5 rounded {weaknessBg(w.fire)} {weaknessColor(w.fire)} flex flex-col items-center">
                  <span class="text-[9px] uppercase opacity-70 text-orange-300">Fire</span>
                  <span class="font-medium">{w.fire ?? '-'}</span>
                </div>
                <div class="px-2 py-1.5 rounded {weaknessBg(w.water)} {weaknessColor(w.water)} flex flex-col items-center">
                  <span class="text-[9px] uppercase opacity-70 text-blue-300">Water</span>
                  <span class="font-medium">{w.water ?? '-'}</span>
                </div>
                <div class="px-2 py-1.5 rounded {weaknessBg(w.thunder)} {weaknessColor(w.thunder)} flex flex-col items-center">
                  <span class="text-[9px] uppercase opacity-70 text-yellow-300">Thunder</span>
                  <span class="font-medium">{w.thunder ?? '-'}</span>
                </div>
                <div class="px-2 py-1.5 rounded {weaknessBg(w.ice)} {weaknessColor(w.ice)} flex flex-col items-center">
                  <span class="text-[9px] uppercase opacity-70 text-cyan-300">Ice</span>
                  <span class="font-medium">{w.ice ?? '-'}</span>
                </div>
                <div class="px-2 py-1.5 rounded {weaknessBg(w.dragon)} {weaknessColor(w.dragon)} flex flex-col items-center">
                  <span class="text-[9px] uppercase opacity-70 text-purple-300">Dragon</span>
                  <span class="font-medium">{w.dragon ?? '-'}</span>
                </div>
              </div>
            </div>
          {/each}
        </div>
      </section>
    {/if}
  {/if}
</div>
