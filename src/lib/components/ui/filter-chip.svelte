<script lang="ts">
  import { cn } from '$lib/utils/index.js'

  let {
    active = false,
    class: className = '',
    onclick,
    children,
    label,
  }: {
    active?: boolean
    class?: string
    onclick?: () => void
    children?: import('svelte').Snippet
    label?: string
  } = $props()
</script>

<button
  type="button"
  {onclick}
  aria-pressed={active}
  aria-label={label}
  class={cn(
    'inline-flex items-center justify-center gap-1.5 rounded-full border text-xs font-medium transition-colors motion-safe:transition-colors motion-reduce:transition-none min-h-[44px] px-4 py-2 sm:min-h-[36px] sm:px-3 sm:py-1.5 focus-visible:outline-none focus-visible:ring-2',
    active
      ? 'border-transparent text-[var(--theme-text-on-primary)]'
      : 'bg-[var(--theme-bg-surface)] text-[var(--theme-text-muted)] border-[var(--theme-border)] hover:text-gray-100 hover:border-[var(--theme-border-strong)]',
    className,
  )}
  style={active
    ? 'background-color: var(--theme-primary);'
    : 'background-color: var(--theme-bg-surface);'}
>
  {#if children}
    {@render children()}
  {:else if label}
    {label}
  {/if}
</button>
