<script lang="ts">
  import { page } from '$app/stores';
  import { api, type MonsterDetail } from '$lib/api';
  import DetailHeader from '$lib/components/detail-header.svelte';
  import { selectedGame } from '$lib/stores/game';

  const id = $derived(Number($page.params.id));
  let monster = $state<MonsterDetail | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);

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
