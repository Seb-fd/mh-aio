<script lang="ts">
  import { goto } from '$app/navigation';
  import { GAMES, selectedGame, type Game } from '$lib/stores/game';
  import Card from '$lib/components/ui/card.svelte';

  function selectGame(game: Game) {
    selectedGame.select(game);
    goto(`/${game.id}`);
  }
</script>

<div class="min-h-screen flex flex-col items-center justify-center p-8">
  <div class="text-center mb-12">
    <h1 class="text-5xl font-bold text-yellow-500 mb-3">MH-AIO</h1>
    <p class="text-xl text-gray-400">Monster Hunter All-in-One Encyclopedia</p>
    <p class="text-sm text-gray-500 mt-2">Select a game to start</p>
  </div>

  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 max-w-4xl w-full">
    {#each GAMES as game}
      <button
        onclick={() => selectGame(game)}
        class="text-left"
      >
        <Card class="p-6 border-2 {game.borderColor} {game.bgHover} transition-all duration-200 cursor-pointer h-full hover:scale-105">
          <div class="flex flex-col h-full">
            <div class="flex items-center gap-3 mb-3">
              <div class="w-10 h-10 rounded-lg bg-gray-800 flex items-center justify-center">
                <span class="text-lg {game.color} font-bold">
                  {game.shortName.charAt(0)}
                </span>
              </div>
              <div>
                <h2 class="text-lg font-semibold {game.color}">{game.shortName}</h2>
                <p class="text-xs text-gray-500">{game.year}</p>
              </div>
            </div>
            <p class="text-sm text-gray-300 mb-2">{game.name}</p>
            <p class="text-xs text-gray-500 mt-auto">{game.platform}</p>
          </div>
        </Card>
      </button>
    {/each}
  </div>
</div>
