import { tick } from 'svelte'

function mainEl(): HTMLElement | null {
  return document.getElementById('main-content')
}

/** Current scroll position of the app scroll container ( NOT window ). */
export function captureScrollY(): number {
  return mainEl()?.scrollTop ?? 0
}

/**
 * Restore a saved position once layout settled. Call after data finished
 * loading (post-tick); the rAF waits out the DOM update so the target offset
 * exists instead of clamping to the skeleton height.
 */
export function restoreScrollY(y: number): void {
  tick().then(() => {
    requestAnimationFrame(() => mainEl()?.scrollTo(0, y))
  })
}
