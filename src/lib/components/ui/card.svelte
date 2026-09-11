<script lang="ts">
  import type { Snippet } from 'svelte'
  import type { HTMLAttributes } from 'svelte/elements'
  import { tv, type VariantProps } from 'tailwind-variants'
  import { cn } from '$lib/utils/index.js'

  const cardTv = tv({
    base: 'rounded-lg border bg-card text-card-foreground shadow-sm',
    variants: {
      variant: {
        default: '',
        themed:
          'bg-[var(--theme-bg-surface)] border-[var(--theme-border)] hover:border-[var(--theme-border-strong)] hover:bg-[var(--theme-bg-elevated)] transition-colors motion-safe:transition-colors motion-reduce:transition-none',
        static: 'bg-[var(--theme-bg-surface)] border-[var(--theme-border)]',
      },
    },
    defaultVariants: { variant: 'default' },
  })

  let {
    class: className,
    variant = 'default',
    children,
    ...restProps
  }: HTMLAttributes<HTMLDivElement> &
    VariantProps<typeof cardTv> & {
      children: Snippet
    } = $props()
</script>

<div class={cn(cardTv({ variant }), className)} {...restProps}>
  {@render children?.()}
</div>
