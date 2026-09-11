<script lang="ts">
  import type { Snippet } from 'svelte'
  import { cn } from '$lib/utils/index.js'

  let {
    title = 'Nothing here yet',
    hint,
    class: className = '',
    children,
  }: { title?: string; hint?: string; class?: string; children?: Snippet } = $props()
</script>

<div
  class={cn(
    'rounded-lg border border-dashed border-[var(--theme-border)] bg-[var(--theme-bg-surface)] p-8 text-center',
    className,
  )}
  role="status"
>
  <p class="text-gray-300 font-medium">{title}</p>
  {#if hint}
    <p class="text-[var(--theme-text-muted)] text-sm mt-2">{hint}</p>
  {/if}
  {#if children}
    <div class="mt-4 flex justify-center">
      {@render children()}
    </div>
  {/if}
</div>
