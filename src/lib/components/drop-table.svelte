<script lang="ts">
  import { goto } from '$app/navigation';
  import { selectedGame } from '$lib/stores/game';
  import type { ItemSource } from '$lib/api';

  let { sources }: { sources: ItemSource[] } = $props();

  const game = $derived($selectedGame);

  const sourceTypeLabel: Record<string, { label: string; icon: string; color: string }> = {
    carve: { label: 'Carve', icon: '⚔️', color: 'text-red-400' },
    capture: { label: 'Capture', icon: '🪤', color: 'text-emerald-400' },
    drop: { label: 'Shiny Drop', icon: '✨', color: 'text-yellow-400' },
    break: { label: 'Break Part', icon: '🔨', color: 'text-orange-400' },
    quest_reward: { label: 'Quest Reward', icon: '📜', color: 'text-blue-400' },
    shiny: { label: 'Shiny Pickup', icon: '✨', color: 'text-yellow-400' },
    mining: { label: 'Mining', icon: '⛏️', color: 'text-orange-400' },
    gather: { label: 'Gathering', icon: '🧺', color: 'text-emerald-400' },
    bug: { label: 'Bug', icon: '🐛', color: 'text-green-400' },
    fish: { label: 'Fishing', icon: '🎣', color: 'text-cyan-400' },
  };

  function goToSource(source: ItemSource) {
    if (!game) return;
    const monsterSources = ['carve', 'capture', 'drop', 'break'];
    if (monsterSources.includes(source.source_type) && source.source_id != null) {
      goto(`/${game.id}/monsters/${source.source_id}`);
    } else if (source.source_type === 'quest_reward' && source.source_id != null) {
      goto(`/${game.id}/quests/${source.source_id}`);
    }
  }

  function pct(p: number | null | undefined): number {
    if (p == null) return 0;
    return Math.round(p * 100);
  }

  function qtyLabel(s: ItemSource): string {
    const min = s.quantity_min ?? 0;
    const max = s.quantity_max ?? 0;
    if (min === max) return `x${min}`;
    return `x${min}-${max}`;
  }
</script>

<div class="space-y-2">
  {#if sources.length === 0}
    <div class="px-4 py-3 rounded-lg border border-dashed border-[var(--theme-border)] text-center">
      <p class="text-sm text-gray-500">No known sources</p>
    </div>
  {:else}
    {#each sources as source}
      {@const meta = sourceTypeLabel[source.source_type] ?? { label: source.source_type, icon: '❓', color: 'text-gray-400' }}
      {@const canNavigate = (['carve', 'capture', 'drop', 'break', 'quest_reward'].includes(source.source_type)) && source.source_id != null}
      {#if canNavigate}
        <button
          onclick={() => goToSource(source)}
          class="w-full block px-4 py-3 rounded-lg border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] hover:border-[var(--theme-border-strong)] hover:bg-[var(--theme-bg-elevated)] transition-all text-left group"
        >
          {@render rowContent(source, meta, true)}
        </button>
      {:else}
        <div class="block px-4 py-3 rounded-lg border border-[var(--theme-border)] bg-[var(--theme-bg-surface)]">
          {@render rowContent(source, meta, false)}
        </div>
      {/if}
    {/each}
  {/if}
</div>

{#snippet rowContent(source: ItemSource, meta: { label: string; icon: string; color: string }, clickable: boolean)}
  <div class="flex items-center gap-3">
    <span class="text-lg shrink-0">{meta.icon}</span>
    <div class="flex-1 min-w-0">
      <div class="flex items-center gap-2">
        <span class="text-xs uppercase tracking-wide {meta.color} font-medium">{meta.label}</span>
        {#if source.source_name}
          <span class="text-sm text-gray-100 truncate {clickable ? 'group-hover:text-[var(--theme-text-accent)]' : ''} transition-colors">
            · {source.source_name}
          </span>
        {/if}
        <span class="text-xs text-gray-500 ml-auto shrink-0">{qtyLabel(source)}</span>
      </div>
      {#if source.location}
        <p class="text-[11px] text-gray-500 mt-0.5">📍 {source.location}</p>
      {/if}
      {#if source.probability != null}
        <div class="mt-2 flex items-center gap-2">
          <div class="flex-1 h-1.5 rounded-full bg-[var(--theme-bg-elevated)] overflow-hidden">
            <div
              class="h-full bg-[var(--theme-accent)] rounded-full transition-all"
              style="width: {pct(source.probability)}%"
            ></div>
          </div>
          <span class="text-[10px] text-gray-400 shrink-0 tabular-nums w-9 text-right">{pct(source.probability)}%</span>
        </div>
      {/if}
    </div>
  </div>
{/snippet}
