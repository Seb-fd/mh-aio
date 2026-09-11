import { writable } from 'svelte/store'

/**
 * Fixed sub-header portal: the app layout renders an empty bar directly under
 * the global header (outside the scroll container). List pages teleport their
 * toolbar controls into it via the `toolbarPortal` action, so the bar sits
 * flush under the header and content can never slip between them.
 */
export const toolbarTarget = writable<HTMLElement | null>(null)
/** Number of toolbars currently teleported (drives bar visibility). */
export const toolbarCount = writable(0)
