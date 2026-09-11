<script lang="ts">
  import { selectedGame } from '$lib/stores/game'
  import GlobalSearch from '$lib/components/global-search.svelte'
  import { Menu, Search, X } from '@lucide/svelte'

  let { onMenuClick }: { onMenuClick?: () => void } = $props()
  const game = $derived($selectedGame)
  let mobileSearch = $state(false)
</script>

<header
  class="border-b shrink-0 sticky top-0 z-30"
  style="background-color: var(--theme-bg-surface); border-color: var(--theme-border); margin-top: env(safe-area-inset-top); padding-top: 2px;"
>
  <div class="flex items-center px-3 sm:px-4 gap-1 sm:gap-2 min-h-[56px]">
    {#if onMenuClick}
      <button
        type="button"
        onclick={onMenuClick}
        class="lg:hidden -ml-1 p-2 text-[var(--theme-text-muted)] hover:text-[var(--theme-text-accent)] transition-colors motion-safe:transition-colors motion-reduce:transition-none touch-manipulation flex items-center justify-center rounded-md focus-visible:outline-none focus-visible:ring-2"
        aria-label="Open menu"
        style="min-width:44px;min-height:44px;"
      >
        <Menu class="h-6 w-6" aria-hidden="true" />
      </button>
    {/if}

    <a
      href="/"
      class="flex items-center gap-2 rounded-md px-1 py-2 min-h-[44px] shrink-0 focus-visible:outline-none focus-visible:ring-2"
      aria-label="MH-AIO home"
    >
      <span class="text-lg sm:text-xl font-bold truncate" style="color: var(--theme-accent);"
        >MH-AIO</span
      >
    </a>

    {#if game}
      <div
        class="hidden md:flex items-center gap-2 ml-2 px-3 py-1 rounded-full border shrink-0"
        style="background-color: var(--theme-bg-elevated); border-color: var(--theme-border-strong);"
      >
        {#if game.iconUrl}
          <img
            src={game.iconUrl}
            alt=""
            width="16"
            height="16"
            class="w-4 h-4 object-contain rounded-sm shrink-0"
            loading="lazy"
          />
        {:else}
          <span
            class="w-2 h-2 rounded-full shrink-0"
            style="background-color: var(--theme-accent);"
            aria-hidden="true"
          ></span>
        {/if}
        <span class="text-sm font-medium whitespace-nowrap" style="color: var(--theme-text-accent);"
          >{game.shortName}</span
        >
        <span class="text-xs text-[var(--theme-text-muted)]" aria-hidden="true">·</span>
        <span class="text-xs text-[var(--theme-text-muted)] whitespace-nowrap">{game.year}</span>
      </div>
    {/if}

    <div class="flex-1 flex justify-end px-1 sm:px-2 min-w-0">
      {#if game}
        <div class="hidden sm:block w-full max-w-xs md:max-w-sm ml-auto">
          <GlobalSearch inputId="global-search-desktop" />
        </div>
        <button
          type="button"
          class="sm:hidden flex items-center justify-center rounded-md text-[var(--theme-text-muted)] hover:text-[var(--theme-text-accent)]"
          style="min-width:44px;min-height:44px;"
          aria-label={mobileSearch ? 'Close search' : 'Open search'}
          aria-expanded={mobileSearch}
          onclick={() => (mobileSearch = !mobileSearch)}
        >
          {#if mobileSearch}
            <X class="h-5 w-5" aria-hidden="true" />
          {:else}
            <Search class="h-5 w-5" aria-hidden="true" />
          {/if}
        </button>
      {/if}
    </div>

    {#if game}
      <span class="text-xs text-[var(--theme-text-muted)] hidden lg:block shrink-0"
        >{game.platform}</span
      >
    {/if}
  </div>

  {#if game && mobileSearch}
    <div class="sm:hidden px-3 pb-3">
      <GlobalSearch inputId="global-search-mobile" autoFocus={true} />
    </div>
  {/if}
</header>
