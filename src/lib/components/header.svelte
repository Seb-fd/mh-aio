<script lang="ts">
  import { selectedGame } from '$lib/stores/game';

  let { onMenuClick }: { onMenuClick?: () => void } = $props();
  const game = $derived($selectedGame);
</script>

<header
  class="border-b flex items-center px-4 gap-4 shrink-0 backdrop-blur-md sticky top-0 z-30"
  style="background-color: color-mix(in oklab, var(--theme-bg-surface) 85%, transparent); border-color: var(--theme-border); margin-top: calc(env(safe-area-inset-top) + 4px); min-height: 2.25rem;"
>
  {#if onMenuClick}
    <button
      type="button"
      onclick={onMenuClick}
      class="lg:hidden -ml-2 p-3 text-gray-400 hover:text-[var(--theme-text-accent)] transition-colors touch-manipulation flex items-center justify-center"
      aria-label="Open menu"
      style="min-width:44px;min-height:44px;"
    >
      <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
      </svg>
    </button>
  {/if}

  <a href="/" class="flex items-center gap-2">
    <span class="text-xl font-bold" style="color: var(--theme-accent);">MH-AIO</span>
  </a>

  {#if game}
    <div class="hidden sm:flex items-center gap-2 ml-4 px-3 py-1 rounded-full border" style="background-color: var(--theme-bg-elevated); border-color: var(--theme-border-strong);">
      <span class="w-2 h-2 rounded-full" style="background-color: var(--theme-accent);"></span>
      <span class="text-sm {game.color} font-medium">{game.shortName}</span>
      <span class="text-xs text-gray-500">·</span>
      <span class="text-xs text-gray-500">{game.year}</span>
    </div>
  {/if}

  <div class="flex-1"></div>

  {#if game}
    <span class="text-xs text-gray-500 hidden md:block">{game.platform}</span>
  {/if}
</header>
