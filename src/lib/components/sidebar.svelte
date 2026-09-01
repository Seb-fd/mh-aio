<script lang="ts">
  import { page } from '$app/state'
  import { goto } from '$app/navigation'
  import { selectedGame } from '$lib/stores/game'
  import ItemIcon from '$lib/components/item-icon.svelte'

  let { onclose }: { onclose?: () => void } = $props()

  const game = $derived($selectedGame)
  const currentPath = $derived(page.url.pathname)

  const navItems = [
    {
      href: '',
      label: 'Home',
      icon: '🏠',
      iconUrl: '/icons/mhfu/home.png',
      iconName: 'Home',
      iconColor: 'Red',
    },
    {
      href: '/monsters',
      label: 'Monsters',
      icon: '🐉',
      iconUrl: '/icons/mhfu/quests/hunt.png',
      iconName: 'Hunting',
      iconColor: 'Red',
    },
    {
      href: '/weapons',
      label: 'Weapons',
      icon: '⚔️',
      iconUrl: '/icons/mhfu/quests/slay.png',
      iconName: 'Slaying',
      iconColor: 'Orange',
    },
    {
      href: '/armor',
      label: 'Armor',
      icon: '🛡️',
      iconUrl: '/icons/mhfu/armor/head.png',
      iconName: 'Head',
      iconColor: 'Gray',
    },
    {
      href: '/quests',
      label: 'Quests',
      icon: '📜',
      iconUrl: '/icons/mhfu/quests/event.png',
      iconName: 'Event',
      iconColor: 'Gray',
    },
    {
      href: '/items',
      label: 'Items',
      icon: '🎒',
      iconUrl: '/icons/mhfu/MH4G-Medicine_Icon_Green.png',
      iconName: 'Medicine',
      iconColor: 'Green',
    },
    {
      href: '/skills',
      label: 'Skills',
      icon: '✨',
      iconUrl: '/icons/mhfu/skills/skills.png',
      iconName: 'Skills',
      iconColor: 'Violet',
    },
    {
      href: '/decorations',
      label: 'Decorations',
      icon: '💎',
      iconUrl: '/icons/mhfu/decorations/ItemIcon017i.png',
      iconName: 'Attack',
      iconColor: 'Red',
    },
    {
      href: '/builds',
      label: 'Builds',
      icon: '🔧',
      iconUrl: '/icons/mhfu/builds/forging.png',
      iconName: 'Forging',
      iconColor: 'Gray',
    },
  ]

  function navigate(href: string) {
    if (!game) return
    goto(`/${game.id}${href}`)
    onclose?.()
  }

  function isActive(href: string): boolean {
    if (!game) return false
    const full = `/${game.id}${href}`
    if (href === '') return currentPath === full
    return currentPath.startsWith(full)
  }

  function changeGame() {
    selectedGame.clear()
    goto('/')
    onclose?.()
  }
</script>

<aside
  class="w-64 flex flex-col h-full lg:h-full border-r sidebar overflow-hidden"
  style="background-color: var(--theme-bg-surface); border-color: var(--theme-border); padding-top: calc(env(safe-area-inset-top) + 2px);"
>
  {#if game}
    <div class="p-4 pt-2 border-b" style="border-color: var(--theme-border);">
      <button
        onclick={changeGame}
        class="text-xs text-gray-500 hover:text-[var(--theme-text-accent)] transition-colors mb-2"
      >
        ← Change Game
      </button>
      <div class="flex items-center gap-3">
        {#if game.iconUrl}
          <img
            src={game.iconUrl}
            alt={game.name}
            width="36"
            height="36"
            class="w-9 h-9 object-contain rounded-md shrink-0"
            loading="lazy"
          />
        {/if}
        <div class="min-w-0">
          <h2 class="text-lg font-bold {game.color} leading-none">{game.shortName}</h2>
          <p class="text-xs text-gray-500 truncate">{game.name}</p>
        </div>
      </div>
    </div>
  {/if}

  <nav class="flex-1 p-3 space-y-1 overflow-y-auto overscroll-contain min-h-0">
    {#each navItems as item}
      {@const active = isActive(item.href)}
      <button
        onclick={() => navigate(item.href)}
        aria-current={active ? 'page' : undefined}
        class="sidebar-item w-full flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm transition-colors text-left"
        class:active
      >
        {#if item.iconUrl}
          <ItemIcon
            iconUrl={item.iconUrl}
            iconName={item.iconName}
            iconColor={item.iconColor}
            size={20}
            alt={item.label}
          />
        {:else}
          <span class="text-base">{item.icon}</span>
        {/if}
        <span>{item.label}</span>
      </button>
    {/each}
  </nav>

  <div class="p-4 border-t" style="border-color: var(--theme-border);">
    <p class="text-xs text-gray-600 text-center">MH-AIO v0.1.0</p>
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
