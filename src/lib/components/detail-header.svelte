<script lang="ts">
  import BackButton from './back-button.svelte'
  import Badge from './ui/badge.svelte'
  import FavoriteButton from './favorite-button.svelte'
  import type { FavoriteKind } from '$lib/api'
  import type { ComponentProps } from 'svelte'

  type BadgeTone = ComponentProps<typeof Badge>['tone']

  let {
    title,
    subtitle,
    icon,
    iconUrl,
    tags = [],
    favKind,
    favId,
  }: {
    title: string
    subtitle?: string
    icon?: string
    iconUrl?: string | null
    tags?: { label: string; color?: string; tone?: BadgeTone }[]
    favKind?: FavoriteKind
    favId?: number
  } = $props()

  let imgFailed = $state(false)
</script>

<header class="relative mb-5 md:mb-6 pb-5 md:pb-6 border-b border-[var(--theme-border)]">
  <div class="flex items-center gap-2 mb-3 md:mb-4">
    <BackButton />
  </div>

  <div class="flex items-start gap-3 md:gap-4 min-w-0">
    {#if iconUrl && !imgFailed}
      <div
        class="w-12 h-12 md:w-14 md:h-14 rounded-xl bg-gradient-to-br from-[var(--theme-banner-from)] to-[var(--theme-banner-to)] border border-[var(--theme-border-strong)] flex items-center justify-center shrink-0 shadow-lg p-1.5"
        style="box-shadow: 0 0 30px var(--theme-glow);"
      >
        <img
          src={iconUrl}
          alt=""
          class="w-9 h-9 md:w-10 md:h-10 object-contain"
          loading="lazy"
          decoding="async"
          onerror={() => (imgFailed = true)}
        />
      </div>
    {:else if icon}
      <div
        class="w-12 h-12 md:w-14 md:h-14 rounded-xl bg-gradient-to-br from-[var(--theme-banner-from)] to-[var(--theme-banner-to)] border border-[var(--theme-border-strong)] flex items-center justify-center shrink-0 shadow-lg"
        style="box-shadow: 0 0 30px var(--theme-glow);"
        aria-hidden="true"
      >
        <span class="text-xl md:text-2xl">{icon}</span>
      </div>
    {/if}
    <div class="min-w-0 flex-1">
      <div class="flex items-start gap-2">
        <h1 class="fluid-h1 font-bold text-gray-100 break-words flex-1">{title}</h1>
        {#if favKind && favId != null}
          <div class="pt-1 shrink-0">
            <FavoriteButton kind={favKind} id={favId} name={title} />
          </div>
        {/if}
      </div>
      {#if subtitle}
        <p class="text-sm text-[var(--theme-text-muted)] mt-1 break-words">{subtitle}</p>
      {/if}
      {#if tags.length > 0}
        <div class="flex flex-wrap gap-1.5 mt-2.5 md:mt-3">
          {#each tags as tag}
            {#if tag.tone}
              <Badge tone={tag.tone}>
                {tag.label}
              </Badge>
            {:else}
              <span
                class="text-[10px] uppercase tracking-wide px-2 py-1 rounded border whitespace-nowrap {tag.color ??
                  'bg-[var(--theme-bg-elevated)] text-gray-300 border-[var(--theme-border)]'}"
              >
                {tag.label}
              </span>
            {/if}
          {/each}
        </div>
      {/if}
    </div>
  </div>
</header>
