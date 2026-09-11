import type { Action } from 'svelte/action'
import { toolbarCount } from '$lib/stores/toolbar'

/**
 * Teleports the toolbar node into the layout's fixed sub-header bar.
 * A comment placeholder keeps the original position for clean teardown
 * (node is restored before Svelte removes it on page unmount).
 */
export const toolbarPortal: Action<HTMLElement, HTMLElement | null> = (node, target) => {
  const placeholder = document.createComment('toolbar-slot')
  node.before(placeholder)
  let mounted = false

  const mount = (el: HTMLElement | null) => {
    if (el && !mounted) {
      el.append(node)
      mounted = true
      toolbarCount.update((n) => n + 1)
    }
  }
  mount(target)

  return {
    update(newTarget: HTMLElement | null) {
      mount(newTarget)
    },
    destroy() {
      if (mounted) {
        toolbarCount.update((n) => Math.max(0, n - 1))
        mounted = false
      }
      if (placeholder.parentNode) placeholder.before(node)
    },
  }
}
