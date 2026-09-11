<script lang="ts">
  import { cn } from '$lib/utils/index.js'

  let { class: className = '', lines = 3 }: { class?: string; lines?: number } = $props()
</script>

<div
  class={cn(
    'rounded-lg border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] p-4 space-y-3 motion-safe:animate-pulse',
    className,
  )}
  aria-hidden="true"
>
  {#each Array(lines) as _, i}
    <div
      class="h-3 rounded bg-[var(--theme-bg-elevated)]"
      style="width: {i === 0 ? '60%' : i === lines - 1 ? '40%' : '90%'}"
    ></div>
  {/each}
  <span class="sr-only">Loading…</span>
</div>
