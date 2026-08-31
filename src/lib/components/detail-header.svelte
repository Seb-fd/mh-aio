<script lang="ts">
  import BackButton from './back-button.svelte'

  let {
    title,
    subtitle,
    icon,
    iconUrl,
    tags = [],
  }: {
    title: string
    subtitle?: string
    icon?: string
    iconUrl?: string | null
    tags?: { label: string; color?: string }[]
  } = $props()

  let imgFailed = $state(false)
</script>

<header class="relative mb-6 pb-6 border-b border-[var(--theme-border)]">
  <div class="flex items-center gap-2 mb-4">
    <BackButton />
  </div>

  <div class="flex items-start gap-4">
    {#if iconUrl && !imgFailed}
      <div
        class="w-14 h-14 rounded-xl bg-gradient-to-br from-[var(--theme-banner-from)] to-[var(--theme-banner-to)] border border-[var(--theme-border-strong)] flex items-center justify-center shrink-0 shadow-lg p-1.5"
        style="box-shadow: 0 0 30px var(--theme-glow);"
      >
        <img
          src={iconUrl}
          alt={title}
          class="w-10 h-10 object-contain"
          loading="lazy"
          onerror={() => (imgFailed = true)}
        />
      </div>
    {:else if icon}
      <div
        class="w-14 h-14 rounded-xl bg-gradient-to-br from-[var(--theme-banner-from)] to-[var(--theme-banner-to)] border border-[var(--theme-border-strong)] flex items-center justify-center shrink-0 shadow-lg"
        style="box-shadow: 0 0 30px var(--theme-glow);"
      >
        <span class="text-2xl">{icon}</span>
      </div>
    {/if}
    <div class="min-w-0 flex-1">
      <h1 class="text-3xl font-bold text-gray-100 leading-tight">{title}</h1>
      {#if subtitle}
        <p class="text-sm text-gray-400 mt-1">{subtitle}</p>
      {/if}
      {#if tags.length > 0}
        <div class="flex flex-wrap gap-1.5 mt-3">
          {#each tags as tag}
            <span
              class="text-[10px] uppercase tracking-wide px-2 py-0.5 rounded border {tag.color ??
                'bg-[var(--theme-bg-elevated)] text-gray-300 border-[var(--theme-border)]'}"
            >
              {tag.label}
            </span>
          {/each}
        </div>
      {/if}
    </div>
  </div>
</header>
