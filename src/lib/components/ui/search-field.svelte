<script lang="ts">
  import { cn } from '$lib/utils/index.js'

  let {
    value = $bindable(''),
    placeholder = 'Search...',
    label = placeholder,
    class: className = '',
    id,
  }: {
    value?: string
    placeholder?: string
    label?: string
    class?: string
    id?: string
  } = $props()
</script>

<!-- F4: single standardized list search. Width override via class (e.g. "sm:w-72"). -->
<input
  type="search"
  bind:value
  {placeholder}
  aria-label={label}
  {id}
  class={cn(
    'w-full sm:w-56 px-3 rounded-lg bg-[var(--theme-bg-surface)] border border-[var(--theme-border)] text-gray-100 placeholder-gray-500 focus:outline-none focus:border-[var(--theme-border-strong)] min-h-[44px] text-base sm:text-sm',
    className,
  )}
/>
