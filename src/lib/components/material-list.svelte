<script lang="ts">
  import { goto } from '$app/navigation';
  import { selectedGame } from '$lib/stores/game';
  import type { MaterialRef } from '$lib/api';

  let {
    materials,
    showCraftingCost,
    craftingCost,
  }: {
    materials: MaterialRef[];
    showCraftingCost?: boolean;
    craftingCost?: number | null;
  } = $props();

  const game = $derived($selectedGame);

  function goToItem(itemId: number) {
    if (!game) return;
    goto(`/${game.id}/items/${itemId}`);
  }
</script>

<div class="space-y-3">
  {#if showCraftingCost}
    <div class="flex items-center justify-between px-4 py-2.5 rounded-lg border border-[var(--theme-border)] bg-[var(--theme-bg-surface)]">
      <span class="text-xs uppercase tracking-wide text-gray-500">Crafting Cost</span>
      <span class="text-base font-semibold text-[var(--theme-accent)]">{craftingCost ?? 0}z</span>
    </div>
  {/if}

  {#if materials.length === 0}
    <div class="px-4 py-3 rounded-lg border border-dashed border-[var(--theme-border)] text-center">
      <p class="text-sm text-gray-500">No materials required</p>
    </div>
  {:else}
    <div class="space-y-1.5">
      {#each materials as mat}
        <button
          onclick={() => goToItem(mat.item_id)}
          class="w-full flex items-center justify-between gap-3 px-4 py-2.5 rounded-lg border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] hover:border-[var(--theme-border-strong)] hover:bg-[var(--theme-bg-elevated)] transition-all group"
        >
          <div class="flex items-center gap-3 min-w-0">
            <span class="w-8 h-8 rounded-md bg-[var(--theme-bg-elevated)] border border-[var(--theme-border)] flex items-center justify-center shrink-0">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-[var(--theme-text-accent)]" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
              </svg>
            </span>
            <span class="text-sm text-gray-100 group-hover:text-[var(--theme-text-accent)] transition-colors truncate text-left">
              {mat.item_name}
            </span>
          </div>
          <span class="text-sm font-semibold text-[var(--theme-accent)] shrink-0">x{mat.quantity}</span>
        </button>
      {/each}
    </div>
  {/if}
</div>
