<script lang="ts">
  import { selectedGame } from '$lib/stores/game'
  import { favorites } from '$lib/stores/favorites'
  import type { FavoriteKind } from '$lib/api'
  import { Star } from '@lucide/svelte'

  interface Props {
    kind: FavoriteKind
    id: number
    name: string
    size?: 'sm' | 'md'
  }

  const { kind, id, name, size = 'md' }: Props = $props()
  const game = $derived($selectedGame)

  $effect(() => {
    if (game) void favorites.ensure(game.dbId)
  })

  const active = $derived(game ? favorites.isFav(game.dbId, kind, id) : null)
  const isActive = $derived(active ? $active : false)

  let busy = $state(false)
  async function onclick(e: MouseEvent) {
    e.stopPropagation()
    if (!game || busy) return
    busy = true
    try {
      await favorites.toggle(game.dbId, kind, id, name)
    } catch (err) {
      console.error('[favorites] toggle failed', err)
    } finally {
      busy = false
    }
  }

  const dims = $derived(size === 'sm' ? 'min-w-[32px] min-h-[32px]' : 'min-w-[36px] min-h-[36px]')
  const icon = $derived(size === 'sm' ? 'h-3.5 w-3.5' : 'h-4 w-4')
</script>

<button
  {onclick}
  disabled={busy || !game}
  aria-pressed={isActive}
  aria-label={isActive ? `Remove ${name} from favorites` : `Add ${name} to favorites`}
  title={isActive ? 'Favorited' : 'Add to favorites'}
  class="rounded-full border inline-flex items-center justify-center shrink-0 focus-visible:outline-none focus-visible:ring-2 disabled:opacity-40 {dims} {isActive
    ? 'border-[var(--theme-accent)]/50 bg-[var(--theme-accent)]/10'
    : 'border-[var(--theme-border)] bg-[var(--theme-bg-surface)] hover:border-[var(--theme-border-strong)]'}"
>
  <Star
    class="{icon} {isActive
      ? 'text-[var(--theme-accent)] fill-[var(--theme-accent)]'
      : 'text-gray-500'}"
    aria-hidden="true"
  />
</button>
