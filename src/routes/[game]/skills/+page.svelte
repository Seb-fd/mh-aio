<script lang="ts">
  import { page } from '$app/stores';
  import { selectedGame } from '$lib/stores/game';

  const game = $derived($selectedGame);
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-6">
    <h1 class="text-2xl font-bold text-gray-100">✨ Skills</h1>
    <p class="text-sm text-gray-500 mt-1">
      {#if game}{game.shortName} · Effects per level and synergies{:else}Select a game first{/if}
    </p>
  </div>
  <div class="bg-gray-900 border border-gray-800 rounded-lg p-8 text-center">
    <p class="text-gray-400 text-lg">Coming soon</p>
    <p class="text-gray-600 text-sm mt-2">Skills from {game?.shortName ?? 'this game'} will be shown here</p>
  </div>
</div>
