<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { selectedGame } from '$lib/stores/game';

  let { onclose }: { onclose?: () => void } = $props();

  const game = $derived($selectedGame);
  const currentPath = $derived($page.url.pathname);

  const navItems = [
    { href: '', label: 'Home', icon: '🏠' },
    { href: '/monsters', label: 'Monsters', icon: '🐉' },
    { href: '/weapons', label: 'Weapons', icon: '⚔️' },
    { href: '/armor', label: 'Armor', icon: '🛡️' },
    { href: '/quests', label: 'Quests', icon: '📜' },
    { href: '/items', label: 'Items', icon: '🎒' },
    { href: '/skills', label: 'Skills', icon: '✨' },
    { href: '/builds', label: 'Builds', icon: '🔧' },
  ];

  function navigate(href: string) {
    if (!game) return;
    goto(`/${game.id}${href}`);
    onclose?.();
  }

  function isActive(href: string): boolean {
    if (!game) return false;
    const full = `/${game.id}${href}`;
    if (href === '') return currentPath === full;
    return currentPath.startsWith(full);
  }

  function changeGame() {
    selectedGame.clear();
    goto('/');
    onclose?.();
  }
</script>

<aside
  class="w-64 flex flex-col h-full border-r sidebar"
  style="background-color: var(--theme-bg-surface); border-color: var(--theme-border);"
>
  {#if game}
    <div class="p-4 border-b" style="border-color: var(--theme-border);">
      <button onclick={changeGame} class="text-xs text-gray-500 hover:text-[var(--theme-text-accent)] transition-colors mb-2">
        ← Change Game
      </button>
      <h2 class="text-lg font-bold {game.color}">{game.shortName}</h2>
      <p class="text-xs text-gray-500">{game.name}</p>
    </div>
  {/if}

  <nav class="flex-1 p-3 space-y-1">
    {#each navItems as item}
      {@const active = isActive(item.href)}
      <button
        onclick={() => navigate(item.href)}
        class="sidebar-item w-full flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm transition-colors text-left"
        class:active={active}
      >
        <span class="text-base">{item.icon}</span>
        <span>{item.label}</span>
      </button>
    {/each}
  </nav>

  <div class="p-4 border-t" style="border-color: var(--theme-border);">
    <p class="text-xs text-gray-600 text-center">MH-AIO v0.2.0</p>
  </div>
</aside>

<style>
  .sidebar-item {
    color: rgb(156 163 175);
  }
  .sidebar-item:hover {
    background-color: var(--theme-bg-elevated);
    color: rgb(229 231 235);
  }
  .sidebar-item.active {
    background-color: color-mix(in oklab, var(--theme-primary) 12%, transparent);
    color: var(--theme-accent);
    font-weight: 500;
  }
  .sidebar-item.active:hover {
    background-color: color-mix(in oklab, var(--theme-primary) 18%, transparent);
  }
</style>
