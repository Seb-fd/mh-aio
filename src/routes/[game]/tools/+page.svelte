<script lang="ts">
  import { goto } from '$app/navigation'
  import { selectedGame } from '$lib/stores/game'
  import { onMount } from 'svelte'
  onMount(() => {
    const g = $selectedGame
    if (g) goto(`/${g.id}/tools/mantles`)
  })
</script>
