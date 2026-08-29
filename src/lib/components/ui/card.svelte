<script lang="ts">
  import type { Snippet } from 'svelte'
  import type { HTMLAttributes } from 'svelte/elements'
  import { cn } from '$lib/utils/index.js'

  let {
    class: className,
    children,
    ...restProps
  }: HTMLAttributes<HTMLDivElement> & {
    children: Snippet
  } = $props()
</script>

<div
  class={cn('rounded-lg border bg-card text-card-foreground shadow-sm', className)}
  {...restProps}
>
  {@render children?.()}
</div>
