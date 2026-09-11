<script lang="ts">
  import '../app.css'
  import { page } from '$app/state'
  import { selectedGame } from '$lib/stores/game'
  import { toolbarTarget, toolbarCount } from '$lib/stores/toolbar'
  import { getQueryClient } from '$lib/query-client'
  import { QueryClientProvider } from '@tanstack/svelte-query'
  import Sidebar from '$lib/components/sidebar.svelte'
  import Header from '$lib/components/header.svelte'

  const queryClient = getQueryClient()

  let { children } = $props()

  let sidebarOpen = $state(false)

  function closeSidebar() {
    sidebarOpen = false
  }

  function onWindowKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && sidebarOpen) closeSidebar()
  }

  const isHome = $derived(page.url.pathname === '/')
  const game = $derived($selectedGame)
  // Fixed sub-header portal target: list pages teleport their toolbars here so
  // the bar sits flush under the global header, outside the scroll container.
  let toolbarBar = $state<HTMLElement | null>(null)
  $effect(() => {
    toolbarTarget.set(toolbarBar)
  })

  const themeStyle = $derived.by(() => {
    if (!game) return ''
    const t = game.theme
    const parts = [
      `--theme-primary: ${t.primary};`,
      `--theme-primary-dark: ${t.primaryDark};`,
      `--theme-accent: ${t.accent};`,
      `--theme-accent-soft: ${t.accentSoft};`,
      `--theme-bg-base: ${t.bgBase};`,
      `--theme-bg-surface: ${t.bgSurface};`,
      `--theme-bg-elevated: ${t.bgElevated};`,
      `--theme-border: ${t.border};`,
      `--theme-border-strong: ${t.borderStrong};`,
      `--theme-text-accent: ${t.textAccent};`,
      `--theme-text-muted: ${t.textMuted};`,
      `--theme-text-on-primary: ${t.textOnPrimary};`,
      `--theme-banner-from: ${t.bannerFrom};`,
      `--theme-banner-to: ${t.bannerTo};`,
      `--theme-glow: ${t.glow};`,
      `--theme-ring: ${t.ring};`,
    ]
    // F0 infra: per-game display-font override (F2 ornaments may set this).
    if (t.fontDisplayOverride) parts.push(`--font-display: ${t.fontDisplayOverride};`)
    return parts.join(' ')
  })
</script>

<svelte:window onkeydown={onWindowKeydown} />

<QueryClientProvider client={queryClient}>
  {#if isHome}
    <div class="min-h-screen bg-gray-950 text-gray-100">
      {@render children()}
    </div>
  {:else if game}
    <a href="#main-content" class="skip-link">Skip to content</a>
    <div
      class="h-screen h-[100dvh] text-gray-100 flex flex-col themed-bg overflow-hidden"
      data-ornament={game.theme.ornament}
      style={themeStyle}
    >
      <Header onMenuClick={() => (sidebarOpen = !sidebarOpen)} />

      <div class="flex flex-1 min-h-0 overflow-hidden">
        {#if sidebarOpen}
          <button
            type="button"
            class="fixed inset-0 bg-black/50 z-40 lg:hidden"
            onclick={closeSidebar}
            aria-label="Close menu"
            tabindex="-1"
          ></button>
        {/if}

        <div
          class="fixed lg:static inset-y-0 left-0 z-50 flex flex-col lg:h-full lg:overflow-hidden transform-gpu will-change-transform transition-transform duration-150 ease-out motion-safe:transition-transform motion-reduce:transition-none
          {sidebarOpen ? 'translate-x-0' : '-translate-x-full lg:translate-x-0'}"
        >
          <Sidebar onclose={closeSidebar} />
        </div>

        <div class="flex min-w-0 min-h-0 flex-1 flex-col">
          <div
            class="shrink-0 border-b {$toolbarCount > 0 ? '' : 'hidden'}"
            style="background-color: var(--theme-bg-base); border-color: var(--theme-border); box-shadow: 0 10px 20px -12px rgba(0, 0, 0, 0.7);"
          >
            <div class="px-4 py-3 md:px-6 lg:px-8" bind:this={toolbarBar}></div>
          </div>
          <main
            id="main-content"
            tabindex="-1"
            class="min-h-0 min-w-0 flex-1 overflow-y-auto overscroll-contain safe-bottom p-4 md:p-6 lg:p-8"
          >
            {@render children()}
          </main>
        </div>
      </div>
    </div>
  {:else}
    <div class="min-h-screen bg-gray-950 text-gray-100">
      {@render children()}
    </div>
  {/if}
</QueryClientProvider>
