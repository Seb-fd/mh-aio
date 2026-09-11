<script lang="ts">
  import type { Snippet } from 'svelte'
  import { tv, type VariantProps } from 'tailwind-variants'
  import { cn } from '$lib/utils/index.js'

  const badgeTv = tv({
    base: 'inline-flex items-center gap-1 rounded border px-2 py-0.5 text-[10px] font-medium uppercase tracking-wide whitespace-nowrap',
    variants: {
      tone: {
        neutral: 'bg-[var(--theme-bg-elevated)] text-gray-300 border-[var(--theme-border)]',
        accent:
          'bg-[var(--theme-bg-elevated)] text-[var(--theme-text-accent)] border-[var(--theme-border-strong)]',
        rankLow: 'bg-gray-700 text-gray-300 border-gray-600',
        rankHigh: 'bg-blue-900/40 text-blue-300 border-blue-800',
        rankMaster: 'bg-purple-900/40 text-purple-300 border-purple-800',
        rankG: 'bg-yellow-900/40 text-yellow-300 border-yellow-800',
        rankMasterRed: 'bg-red-900/40 text-red-300 border-red-800',
        key: 'bg-yellow-500/10 text-yellow-500 border-yellow-500/30',
        urgent: 'bg-red-500/10 text-red-400 border-red-500/30',
      },
    },
    defaultVariants: { tone: 'neutral' },
  })

  let {
    tone = 'neutral',
    class: className,
    children,
  }: VariantProps<typeof badgeTv> & { class?: string; children: Snippet } = $props()
</script>

<span class={cn(badgeTv({ tone }), className)}>
  {@render children()}
</span>
