<script lang="ts">
  import { goto } from '$app/navigation'
  import { GAMES, selectedGame, type Game } from '$lib/stores/game'
  import Card from '$lib/components/ui/card.svelte'
  import { Swords, ChevronRight } from '@lucide/svelte'

  function selectGame(game: Game) {
    selectedGame.select(game)
    goto(`/${game.id}`)
  }
</script>

<div
  class="min-h-screen min-h-[100dvh] flex flex-col items-center p-4 sm:p-8 bg-gray-950 safe-bottom"
>
  <div
    class="w-full max-w-4xl rounded-2xl border border-gray-800 bg-gradient-to-br from-gray-900 via-gray-950 to-black px-6 py-8 sm:px-10 sm:py-12 text-center mb-6 sm:mb-8 overflow-hidden relative"
  >
    <div
      class="pointer-events-none absolute inset-0 opacity-40"
      aria-hidden="true"
      style="background-image: radial-gradient(ellipse 70% 60% at 50% 0%, rgba(251,191,36,0.18), transparent);"
    ></div>
    <div class="relative">
      <p
        class="font-display mb-3 text-[11px] font-bold uppercase tracking-[0.3em] text-yellow-500/80"
      >
        Guild Archive · Offline
      </p>
      <div
        class="mx-auto mb-4 flex h-14 w-14 items-center justify-center rounded-2xl border border-yellow-500/40 bg-yellow-500/10"
      >
        <Swords class="h-7 w-7 text-yellow-500" aria-hidden="true" />
      </div>
      <h1 class="fluid-h1 font-bold text-gray-50 mb-2">MH-AIO</h1>
      <p class="text-base sm:text-lg text-gray-300">Monster Hunter All-in-One Encyclopedia</p>
      <p class="text-sm text-gray-400 mt-2">
        World · Rise · Wilds · Portable 3rd · Freedom Unite — offline, always with you
      </p>
    </div>
  </div>

  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3 sm:gap-4 max-w-4xl w-full">
    {#each GAMES as game, i}
      <button
        onclick={() => selectGame(game)}
        aria-label="Select {game.name}"
        class="reveal text-left rounded-xl min-h-[44px] focus-visible:outline-none focus-visible:ring-2"
        style="--i: {i};"
      >
        <Card
          class="h-full overflow-hidden border-2 transition-colors motion-safe:transition-colors motion-reduce:transition-none duration-200 cursor-pointer hover:scale-[1.02] motion-safe:hover:scale-[1.02]"
          style="border-color: {game.theme.borderStrong}; background-color: {game.theme.bgSurface};"
        >
          <div
            class="h-1 w-full"
            aria-hidden="true"
            style="background: linear-gradient(90deg, {game.theme.bannerFrom}, {game.theme
              .primary});"
          ></div>
          <div class="relative flex flex-col h-full p-5 overflow-hidden">
            <span
              aria-hidden="true"
              class="font-display pointer-events-none absolute -right-1 -bottom-5 text-7xl font-bold opacity-10 select-none"
              style="color: {game.theme.textAccent};"
            >
              {game.theme.sigil}
            </span>
            <div class="relative flex items-center gap-3 mb-3">
              <div
                class="w-10 h-10 rounded-lg flex items-center justify-center overflow-hidden shrink-0"
                style="background-color: {game.theme.bgElevated}; border: 1px solid {game.theme
                  .borderStrong};"
              >
                {#if game.iconUrl}
                  <img
                    src={game.iconUrl}
                    alt=""
                    width="40"
                    height="40"
                    class="w-10 h-10 object-contain"
                    loading="lazy"
                  />
                {:else}
                  <span
                    class="font-display text-lg font-bold"
                    style="color: {game.theme.textAccent};"
                    aria-hidden="true"
                  >
                    {game.shortName.charAt(0)}
                  </span>
                {/if}
              </div>
              <div class="min-w-0">
                <h2
                  class="font-display text-base font-bold truncate"
                  style="color: {game.theme.textAccent};"
                >
                  {game.shortName}
                </h2>
                <p class="text-xs text-gray-400">{game.year} · {game.platform}</p>
              </div>
              <ChevronRight class="ml-auto h-4 w-4 shrink-0 text-gray-400" aria-hidden="true" />
            </div>
            <p class="relative text-sm text-gray-200">{game.name}</p>
          </div>
        </Card>
      </button>
    {/each}
  </div>
  <p class="mt-6 text-xs text-gray-500">Your data lives on-device — no account, no tracking.</p>
</div>
