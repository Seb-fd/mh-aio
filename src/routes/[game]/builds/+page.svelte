<script lang="ts">
  import { selectedGame } from '$lib/stores/game';

  const game = $derived($selectedGame);
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-6">
    <h1 class="text-2xl font-bold text-gray-100">Builds</h1>
    <p class="text-sm text-gray-500 mt-1">
      {#if game}{game.shortName} · Suggestions and build planner{:else}Select a game first{/if}
    </p>
  </div>
  <div class="bg-gray-900 border border-gray-800 rounded-lg p-8 text-center">
    <p class="text-gray-400 text-lg">Coming soon</p>
    <p class="text-gray-600 text-sm mt-2">
      The build planner will use the loaded monsters, weapons, armor and skills data.
    </p>
  </div>
</div>
