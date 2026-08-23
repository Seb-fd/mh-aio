<script lang="ts">
  import '../app.css';
  import { page } from '$app/stores';
  import { selectedGame, GAMES } from '$lib/stores/game';
  import Sidebar from '$lib/components/sidebar.svelte';
  import Header from '$lib/components/header.svelte';

  let { children } = $props();

  let sidebarOpen = $state(false);

  const isHome = $derived($page.url.pathname === '/');
  const game = $derived($selectedGame);

  $effect(() => {
    const pathGameId = $page.url.pathname.split('/')[1];
    if (pathGameId && pathGameId !== '') {
      const found = GAMES.find(g => g.id === pathGameId);
      if (found && $selectedGame?.id !== pathGameId) {
        selectedGame.select(found);
      }
    }
  });
</script>

{#if isHome}
  <div class="min-h-screen bg-gray-950 text-gray-100">
    {@render children()}
  </div>
{:else}
  <div class="min-h-screen bg-gray-950 text-gray-100 flex flex-col">
    <Header onMenuClick={() => sidebarOpen = !sidebarOpen} />
    
    <div class="flex flex-1 overflow-hidden">
      {#if sidebarOpen}
        <div
          class="fixed inset-0 bg-black/50 z-40 lg:hidden"
          onclick={() => sidebarOpen = false}
          role="presentation"
        ></div>
      {/if}

      <div
        class="fixed lg:static inset-y-0 left-0 z-50 transform transition-transform duration-200 ease-in-out
          {sidebarOpen ? 'translate-x-0' : '-translate-x-full lg:translate-x-0'}"
      >
        <Sidebar onclose={() => sidebarOpen = false} />
      </div>

      <main class="flex-1 overflow-auto p-6">
        {@render children()}
      </main>
    </div>
  </div>
{/if}
