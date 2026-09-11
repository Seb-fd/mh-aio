<script lang="ts">
  import { page } from '$app/state'
  import { goto } from '$app/navigation'
  import { selectedGame } from '$lib/stores/game'

  let { children } = $props()
  const game = $derived($selectedGame)
  const currentPath = $derived(page.url.pathname)

  function isActive(prefix: string): boolean {
    if (!game) return false
    return currentPath.includes(`/tools/${prefix}`)
  }

  function go(tab: string) {
    if (!game) return
    goto(`/${game.id}/tools/${tab}`)
  }
</script>

<div class="max-w-6xl mx-auto">
  {#if game && game.id !== 'mhw'}
    <div class="border rounded-lg p-8 text-center themed-card mb-6">
      <p class="text-gray-400">
        The Tools section is exclusive to <span class="text-blue-400"
          >Monster Hunter World + Iceborne</span
        >.
      </p>
      <p class="text-xs text-gray-400 mt-1">
        Switch to MHW in the game selector to see this section.
      </p>
    </div>
  {:else}
    <div class="mb-4 md:mb-6">
      <h1 class="fluid-h2 font-bold text-gray-100">Tools</h1>
      <p class="text-sm text-gray-400 mt-1">
        MHW · Mantles · Boosters · Palico (Gadgets + Safari) — World + Iceborne
      </p>
      <div class="flex gap-2 mt-4 flex-wrap" role="group" aria-label="Tool categories">
        <button
          onclick={() => go('mantles')}
          aria-current={isActive('mantles') ? 'page' : undefined}
          class="px-4 min-h-[44px] rounded-full text-xs font-medium border transition-colors motion-safe:transition-colors motion-reduce:transition-none focus-visible:outline-none focus-visible:ring-2 {isActive(
            'mantles',
          )
            ? 'bg-[var(--theme-primary)] text-[var(--theme-text-on-primary)] border-transparent'
            : 'bg-[var(--theme-bg-surface)] text-gray-400 border-[var(--theme-border)] hover:text-gray-200'}"
          >Mantles (17)</button
        >
        <button
          onclick={() => go('boosters')}
          aria-current={isActive('boosters') ? 'page' : undefined}
          class="px-4 min-h-[44px] rounded-full text-xs font-medium border transition-colors motion-safe:transition-colors motion-reduce:transition-none focus-visible:outline-none focus-visible:ring-2 {isActive(
            'boosters',
          )
            ? 'bg-[var(--theme-primary)] text-[var(--theme-text-on-primary)] border-transparent'
            : 'bg-[var(--theme-bg-surface)] text-gray-400 border-[var(--theme-border)] hover:text-gray-200'}"
          >Boosters (3)</button
        >
        <button
          onclick={() => go('palico')}
          aria-current={isActive('palico') ? 'page' : undefined}
          class="px-4 min-h-[44px] rounded-full text-xs font-medium border transition-colors motion-safe:transition-colors motion-reduce:transition-none focus-visible:outline-none focus-visible:ring-2 {isActive(
            'palico',
          )
            ? 'bg-[var(--theme-primary)] text-[var(--theme-text-on-primary)] border-transparent'
            : 'bg-[var(--theme-bg-surface)] text-gray-400 border-[var(--theme-border)] hover:text-gray-200'}"
          >Palico</button
        >
      </div>
    </div>
  {/if}
  {@render children()}
</div>
