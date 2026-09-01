<script lang="ts">
  let {
    iconUrl,
    iconName,
    iconColor,
    size = 24,
    alt = 'item icon',
  }: {
    iconUrl?: string | null
    iconName?: string | null
    iconColor?: string | null
    size?: number
    alt?: string
  } = $props()

  function colorCss(c?: string | null): string {
    const map: Record<string, string> = {
      Green: '#22c55e',
      DarkGreen: '#15803d',
      Blue: '#3b82f6',
      DarkBlue: '#1e40af',
      'Light Blue': '#7dd3fc',
      LightBeige: '#e7d6b8',
      Beige: '#c8b48a',
      DarkBeige: '#a8936b',
      Gold: '#fbbf24',
      Orange: '#f97316',
      Red: '#ef4444',
      DarkRed: '#991b1b',
      Cyan: '#06b6d4',
      Violet: '#a855f7',
      Pink: '#ec4899',
      Gray: '#9ca3af',
      DarkPurple: '#6b21a8',
      White: '#f8fafc',
      Yellow: '#eab308',
      Lime: '#84cc16',
    }
    if (!c) return '#9ca3af'
    if (c.startsWith('#')) return c
    return map[c] ?? '#9ca3af'
  }

  let failed = $state(false)
</script>

{#if iconUrl && !failed}
  <img
    src={iconUrl}
    {alt}
    width={size}
    height={size}
    class="shrink-0 object-contain"
    style="width: {size}px; height: {size}px;"
    loading="lazy"
    onerror={() => (failed = true)}
  />
{:else if iconName}
  <div
    class="shrink-0 rounded-md flex items-center justify-center border border-white/10"
    style="width: {size}px; height: {size}px; background-color: {colorCss(
      iconColor,
    )}22; border-color: {colorCss(iconColor)}55;"
    title={iconName + (iconColor ? ` (${iconColor})` : '')}
  >
    <span class="text-[9px] font-bold leading-none" style="color: {colorCss(iconColor)}"
      >{iconName.slice(0, 2).toUpperCase()}</span
    >
  </div>
{:else}
  <div
    class="shrink-0 rounded-md bg-[var(--theme-bg-elevated)] border border-[var(--theme-border)] flex items-center justify-center"
    style="width: {size}px; height: {size}px;"
  >
    <span class="text-xs">📦</span>
  </div>
{/if}
