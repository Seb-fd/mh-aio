<script lang="ts">
  import type { Snippet } from 'svelte'
  import type { HTMLButtonAttributes } from 'svelte/elements'
  import { tv, type VariantProps } from 'tailwind-variants'
  import { cn } from '$lib/utils/index.js'

  const buttonTv = tv({
    base: 'inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors motion-safe:transition-colors motion-reduce:transition-none focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:size-4 [&_svg]:shrink-0',
    variants: {
      variant: {
        default: 'bg-primary text-primary-foreground hover:bg-primary/90',
        themed:
          'border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] text-gray-200 hover:border-[var(--theme-border-strong)] hover:bg-[var(--theme-bg-elevated)]',
        themedPrimary:
          'bg-[var(--theme-primary)] text-[var(--theme-text-on-primary)] hover:brightness-110 border border-transparent',
        destructive: 'bg-destructive text-destructive-foreground hover:bg-destructive/90',
        outline: 'border border-input bg-background hover:bg-accent hover:text-accent-foreground',
        secondary: 'bg-secondary text-secondary-foreground hover:bg-secondary/80',
        ghost: 'hover:bg-accent hover:text-accent-foreground',
        link: 'text-primary underline-offset-4 hover:underline',
      },
      size: {
        default: 'h-10 px-4 py-2 min-h-[44px]',
        sm: 'h-9 rounded-md px-3 min-h-[44px] sm:min-h-0 sm:h-9',
        lg: 'h-11 rounded-md px-8 min-h-[48px]',
        icon: 'h-11 w-11 min-w-[44px] min-h-[44px]',
        chip: 'h-11 px-4 py-2 rounded-full text-xs font-medium min-h-[44px] sm:h-9 sm:min-h-[36px]',
      },
    },
    defaultVariants: { variant: 'default', size: 'default' },
  })

  type ButtonVariants = VariantProps<typeof buttonTv>

  let {
    class: className,
    variant = 'default',
    size = 'default',
    children,
    ...restProps
  }: HTMLButtonAttributes &
    ButtonVariants & {
      children: Snippet
    } = $props()
</script>

<button class={cn(buttonTv({ variant, size }), className)} {...restProps}>
  {@render children?.()}
</button>
