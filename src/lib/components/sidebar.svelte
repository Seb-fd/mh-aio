<script lang="ts">
  import { page } from '$app/state'
  import { goto } from '$app/navigation'
  import { selectedGame } from '$lib/stores/game'
  import { prefetchSection } from '$lib/prefetch'
  import ItemIcon from '$lib/components/item-icon.svelte'

  let { onclose }: { onclose?: () => void } = $props()

  const game = $derived($selectedGame)
  const currentPath = $derived(page.url.pathname)

  interface NavItem {
    href: string
    label: string
    iconUrl: string
    iconName: string
    iconColor: string
    gameOnly?: string
  }

  const navItems: NavItem[] = [
    {
      href: '',
      label: 'Home',
      iconUrl: '/icons/mhfu/home.png',
      iconName: 'Home',
      iconColor: 'Red',
    },
    {
      href: '/monsters',
      label: 'Monsters',
      iconUrl: '/icons/mhfu/quests/hunt.png',
      iconName: 'Hunting',
      iconColor: 'Red',
    },
    {
      href: '/quests',
      label: 'Quests',
      iconUrl: '/icons/mhfu/quests/event.png',
      iconName: 'Event',
      iconColor: 'Gray',
    },
    {
      href: '/weapons',
      label: 'Weapons',
      iconUrl: '/icons/mhfu/quests/slay.png',
      iconName: 'Slaying',
      iconColor: 'Orange',
    },
    {
      href: '/armor',
      label: 'Armor',
      iconUrl: '/icons/mhfu/armor/head.png',
      iconName: 'Head',
      iconColor: 'Gray',
    },
    {
      href: '/decorations',
      label: 'Decorations',
      iconUrl: '/icons/mhfu/decorations/ItemIcon017i.png',
      iconName: 'Attack',
      iconColor: 'Red',
    },
    {
      href: '/skills',
      label: 'Skills',
      iconUrl: '/icons/mhfu/skills/skills.png',
      iconName: 'Skills',
      iconColor: 'Violet',
    },
    {
      href: '/items',
      label: 'Items',
      iconUrl: '/icons/mhfu/MH4G-Medicine_Icon_Green.png',
      iconName: 'Medicine',
      iconColor: 'Green',
    },
    {
      href: '/builds',
      label: 'Builds',
      iconUrl: '/icons/mhfu/builds/forging.png',
      iconName: 'Forging',
      iconColor: 'Gray',
    },
    {
      href: '/tools',
      label: 'Tools',
      iconUrl: '/icons/mhw/tools/Tools.png',
      iconName: 'Ghillie Mantle',
      iconColor: '#3D7F3A',
      gameOnly: 'mhw',
    },
  ]

  const visibleItems = $derived(navItems.filter((i) => !i.gameOnly || i.gameOnly === game?.id))

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
  aria-label="Game sections"
  class="w-[84vw] max-w-80 sm:w-72 lg:w-64 flex flex-col h-full lg:h-full border-r sidebar overflow-hidden safe-bottom"
  style="background-color: var(--theme-bg-surface); border-color: var(--theme-border); padding-top: calc(env(safe-area-inset-top) + 2px);"
>
  {#if game}
    <div class="p-4 pt-2 border-b" style="border-color: var(--theme-border);">
      <button
        onclick={changeGame}
        class="inline-flex items-center rounded-md text-xs text-[var(--theme-text-muted)] hover:text-[var(--theme-text-accent)] transition-colors motion-safe:transition-colors motion-reduce:transition-none mb-2 px-1 min-h-[44px] focus-visible:outline-none focus-visible:ring-2"
      >
        <span aria-hidden="true">←&nbsp;</span>Change Game
      </button>
      <div class="flex items-center gap-3">
        {#if game.iconUrl}
          <img
            src={game.iconUrl}
            alt=""
            width="36"
            height="36"
            class="w-9 h-9 object-contain rounded-md shrink-0"
            loading="lazy"
          />
        {/if}
        <div class="min-w-0">
          <h2 class="text-lg font-bold leading-none" style="color: var(--theme-text-accent);">
            {game.shortName}
          </h2>
          <p class="text-xs text-[var(--theme-text-muted)] truncate">{game.name}</p>
        </div>
      </div>
    </div>
  {/if}

  <nav
    aria-label="Game sections"
    class="flex-1 px-3 py-2 space-y-1 overflow-y-auto overscroll-contain min-h-0"
  >
    {#each visibleItems as item}
      {@const active = isActive(item.href)}
      <button
        onclick={() => navigate(item.href)}
        onmouseenter={() => prefetchSection(item.href, game?.dbId)}
        onfocusin={() => prefetchSection(item.href, game?.dbId)}
        aria-current={active ? 'page' : undefined}
        class="sidebar-item w-full flex items-center gap-3 px-3 rounded-lg text-sm transition-colors motion-safe:transition-colors motion-reduce:transition-none text-left min-h-[44px] py-2.5 focus-visible:outline-none focus-visible:ring-2"
        class:active
      >
        <ItemIcon
          iconUrl={item.iconUrl}
          iconName={item.iconName}
          iconColor={item.iconColor}
          size={20}
          alt=""
        />
        <span>{item.label}</span>
      </button>
    {/each}
  </nav>

  <div class="p-4 border-t" style="border-color: var(--theme-border);">
    <p class="text-xs text-[var(--theme-text-muted)] text-center">MH-AIO v0.1.0</p>
  </div>
</aside>

<style>
  .sidebar-item {
    color: var(--theme-text-muted, rgb(156 163 175));
  }
  .sidebar-item:hover {
    background-color: var(--theme-bg-elevated);
    color: rgb(229 231 235);
  }
  .sidebar-item.active {
    /* Fallback for WebViews without color-mix */
    background-color: rgba(255, 255, 255, 0.06);
    background-color: color-mix(in oklab, var(--theme-primary) 12%, transparent);
    color: var(--theme-accent);
    font-weight: 500;
  }
  .sidebar-item.active:hover {
    background-color: rgba(255, 255, 255, 0.09);
    background-color: color-mix(in oklab, var(--theme-primary) 18%, transparent);
  }
</style>
