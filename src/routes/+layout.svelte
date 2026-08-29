<script lang="ts">
  import '../app.css'
  import { page } from '$app/state'
  import { selectedGame } from '$lib/stores/game'
  import Sidebar from '$lib/components/sidebar.svelte'
  import Header from '$lib/components/header.svelte'

  let { children } = $props()

  let sidebarOpen = $state(false)

  const isHome = $derived(page.url.pathname === '/')
  const game = $derived($selectedGame)

  const themeStyle = $derived.by(() => {
    if (!game) return ''
    const t = game.theme
    return [
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
      `--theme-text-on-primary: ${t.textOnPrimary};`,
      `--theme-banner-from: ${t.bannerFrom};`,
      `--theme-banner-to: ${t.bannerTo};`,
      `--theme-glow: ${t.glow};`,
    ].join(' ')
  })
</script>

{#if isHome}
  <div class="min-h-screen bg-gray-950 text-gray-100">
    {@render children()}
  </div>
{:else if game}
  <div
    class="min-h-screen text-gray-100 flex flex-col themed-bg"
    data-ornament={game.theme.ornament}
    style={themeStyle}
  >
    <Header onMenuClick={() => (sidebarOpen = !sidebarOpen)} />

    <div class="flex flex-1 overflow-hidden">
      {#if sidebarOpen}
        <button
          type="button"
          class="fixed inset-0 bg-black/50 z-40 lg:hidden"
          onclick={() => (sidebarOpen = false)}
          aria-label="Close menu"
        ></button>
      {/if}

      <div
        class="fixed lg:static inset-y-0 left-0 z-50 transform-gpu will-change-transform transition-transform duration-150 ease-out
          {sidebarOpen ? 'translate-x-0' : '-translate-x-full lg:translate-x-0'}"
      >
        <Sidebar onclose={() => (sidebarOpen = false)} />
      </div>

      <main class="flex-1 overflow-auto p-6">
        {@render children()}
      </main>
    </div>
  </div>
{:else}
  <div class="min-h-screen bg-gray-950 text-gray-100">
    {@render children()}
  </div>
{/if}
