<script lang="ts">
  import { selectedGame } from '$lib/stores/game';

  let { onMenuClick }: { onMenuClick?: () => void } = $props();
  const game = $derived($selectedGame);
</script>

<header class="h-14 bg-gray-900 border-b border-gray-800 flex items-center px-4 gap-4 shrink-0">
  {#if onMenuClick}
    <button
      onclick={onMenuClick}
      class="lg:hidden text-gray-400 hover:text-gray-200 transition-colors"
    >
      <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
      </svg>
    </button>
  {/if}

  <a href="/" class="flex items-center gap-2">
    <span class="text-xl font-bold text-yellow-500">MH-AIO</span>
  </a>

  {#if game}
    <div class="hidden sm:flex items-center gap-2 ml-4 px-3 py-1 rounded-full bg-gray-800 border border-gray-700">
      <span class="w-2 h-2 rounded-full bg-yellow-500"></span>
      <span class="text-sm {game.color} font-medium">{game.shortName}</span>
      <span class="text-xs text-gray-500">·</span>
      <span class="text-xs text-gray-500">{game.year}</span>
    </div>
  {/if}

  <div class="flex-1"></div>

  {#if game}
    <span class="text-xs text-gray-600 hidden md:block">{game.platform}</span>
  {/if}
</header>
