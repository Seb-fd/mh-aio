<script lang="ts">
  import type { Snippet } from 'svelte'
  import { TriangleAlert, RotateCcw } from '@lucide/svelte'
  import { cn } from '$lib/utils/index.js'

  let {
    title = 'Something went wrong',
    error,
    onRetry,
    class: className = '',
    children,
  }: {
    title?: string
    error?: string | null
    onRetry?: () => void
    class?: string
    children?: Snippet
  } = $props()
</script>

<div
  role="alert"
  class={cn('rounded-lg border border-red-900 bg-red-950/30 p-8 text-center', className)}
>
  <TriangleAlert class="mx-auto h-6 w-6 text-red-400" aria-hidden="true" />
  <p class="mt-3 font-medium text-red-300">{title}</p>
  {#if error}
    <p class="mt-2 text-sm text-[var(--theme-text-muted)]">{error}</p>
  {/if}
  {#if children}
    <div class="mt-4 flex justify-center">
      {@render children()}
    </div>
  {:else if onRetry}
    <button
      type="button"
      onclick={onRetry}
      class="mt-4 inline-flex items-center gap-1.5 rounded-full border border-red-900 px-4 min-h-[44px] text-sm text-red-300 hover:bg-red-950/50 focus-visible:outline-none focus-visible:ring-2"
    >
      <RotateCcw class="h-3.5 w-3.5" aria-hidden="true" /> Try again
    </button>
  {/if}
</div>
