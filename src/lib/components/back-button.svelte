<script lang="ts">
  import { goto } from '$app/navigation';
  import { selectedGame } from '$lib/stores/game';

  let { fallbackHref }: { fallbackHref?: string } = $props();

  const game = $derived($selectedGame);

  function goBack() {
    if (typeof window !== 'undefined' && window.history.length > 1) {
      window.history.back();
    } else {
      const fallback = fallbackHref ?? (game ? `/${game.id}` : '/');
      goto(fallback);
    }
  }
</script>

<button
  onclick={goBack}
  class="inline-flex items-center gap-1.5 text-sm text-gray-400 hover:text-[var(--theme-text-accent)] transition-colors group"
>
  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 transition-transform group-hover:-translate-x-0.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
    <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
  </svg>
  <span>Back</span>
</button>
