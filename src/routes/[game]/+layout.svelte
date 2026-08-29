<script lang="ts">
  import { page } from '$app/state'
  import { selectedGame, GAMES } from '$lib/stores/game'

  let { children } = $props()

  const gameId = $derived(page.params.game)

  $effect(() => {
    if (gameId) {
      const found = GAMES.find((g) => g.id === gameId)
      if (found && $selectedGame?.id !== found.id) {
        selectedGame.select(found)
      }
    }
  })
</script>

{@render children()}
