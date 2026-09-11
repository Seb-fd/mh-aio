<script lang="ts">
  import { goto } from '$app/navigation'
  import { selectedGame } from '$lib/stores/game'
  import { prefetchSection } from '$lib/prefetch'
  import Card from '$lib/components/ui/card.svelte'
  import ItemIcon from '$lib/components/item-icon.svelte'

  const game = $derived($selectedGame)

  interface Section {
    href: string
    label: string
    iconUrl: string
    iconName: string
    iconColor: string
    desc: string
    gameOnly?: string
    /** Bento feature tile: spans 2 columns on sm+ screens. */
    wide?: boolean
  }

  const allSections: Section[] = [
    {
      href: '/monsters',
      label: 'Monsters',
      iconUrl: '/icons/mhfu/quests/hunt.png',
      iconName: 'Hunting',
      iconColor: 'Red',
      desc: 'Weaknesses, materials and tips',
      wide: true,
    },
    {
      href: '/weapons',
      label: 'Weapons',
      iconUrl: '/icons/mhfu/quests/slay.png',
      iconName: 'Slaying',
      iconColor: 'Orange',
      desc: 'Stats, elements and upgrade tree',
    },
    {
      href: '/armor',
      label: 'Armor',
      iconUrl: '/icons/mhfu/armor/head.png',
      iconName: 'Head',
      iconColor: 'Gray',
      desc: 'Sets, skills and resistances',
    },
    {
      href: '/quests',
      label: 'Quests',
      iconUrl: '/icons/mhfu/quests/event.png',
      iconName: 'Event',
      iconColor: 'Gray',
      desc: 'Key quests, rewards and drop rates',
    },
    {
      href: '/items',
      label: 'Items',
      iconUrl: '/icons/mhfu/MH4G-Medicine_Icon_Green.png',
      iconName: 'Medicine',
      iconColor: 'Green',
      desc: 'Materials, consumables and locations',
    },
    {
      href: '/skills',
      label: 'Skills',
      iconUrl: '/icons/mhfu/skills/skills.png',
      iconName: 'Skills',
      iconColor: 'Violet',
      desc: 'Effects per level and synergies',
    },
    {
      href: '/decorations',
      label: 'Decorations',
      iconUrl: '/icons/mhfu/decorations/ItemIcon017i.png',
      iconName: 'Attack',
      iconColor: 'Red',
      desc: 'Jewels, slots and crafting materials',
    },
    {
      href: '/builds',
      label: 'Builds',
      iconUrl: '/icons/mhfu/builds/forging.png',
      iconName: 'Forging',
      iconColor: 'Gray',
      desc: 'Suggestions and planner',
    },
    {
      href: '/tools',
      label: 'Tools',
      iconUrl: '/icons/mhw/tools/Tools.png',
      iconName: 'Ghillie Mantle',
      iconColor: '#3D7F3A',
      desc: 'Mantles, Boosters & Palico — World+Iceborne',
      gameOnly: 'mhw',
    },
  ]
  const sections = $derived(allSections.filter((s) => !s.gameOnly || s.gameOnly === game?.id))

  function navigate(href: string) {
    if (!game) return
    goto(`/${game.id}${href}`)
  }
</script>

{#if game}
  <div class="max-w-5xl mx-auto">
    <div
      class="themed-hero mb-6 md:mb-8 flex items-center gap-4 rounded-2xl border p-5 sm:p-6 overflow-hidden relative"
    >
      <div
        class="pointer-events-none absolute inset-0 opacity-60"
        aria-hidden="true"
        style="background-image: radial-gradient(ellipse 80% 90% at 100% 0%, var(--theme-glow), transparent);"
      ></div>
      <span
        aria-hidden="true"
        class="font-display pointer-events-none absolute -right-2 -bottom-7 text-8xl sm:text-9xl font-bold opacity-15 select-none"
        style="color: var(--theme-primary);"
      >
        {game.theme.sigil}
      </span>
      {#if game.iconUrl}
        <img
          src={game.iconUrl}
          alt=""
          width="56"
          height="56"
          class="w-14 h-14 object-contain rounded-lg shrink-0 relative"
          loading="lazy"
        />
      {:else}
        <div
          class="w-14 h-14 rounded-xl flex items-center justify-center shrink-0 relative border border-[var(--theme-border-strong)] bg-[var(--theme-bg-elevated)]"
          aria-hidden="true"
        >
          <span class="font-display text-2xl font-bold" style="color: var(--theme-text-accent);">
            {game.shortName.charAt(0)}
          </span>
        </div>
      {/if}
      <div class="min-w-0 relative">
        <p
          class="font-display text-[11px] font-bold uppercase tracking-[0.25em] mb-1.5"
          style="color: var(--theme-text-muted);"
        >
          Guild Record · {game.year}
        </p>
        <h1 class="fluid-h2 font-bold mb-1 break-words" style="color: var(--theme-text-accent);">
          {game.name}
        </h1>
        <p class="text-[var(--theme-text-muted)] text-sm sm:text-base">{game.platform}</p>
      </div>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3 sm:gap-4">
      {#each sections as section, i}
        <button
          onclick={() => navigate(section.href)}
          onmouseenter={() => prefetchSection(section.href, game?.dbId)}
          onfocusin={() => prefetchSection(section.href, game?.dbId)}
          aria-label="Open {section.label}"
          class="reveal text-left rounded-xl min-h-[44px] focus-visible:outline-none focus-visible:ring-2 {section.wide
            ? 'sm:col-span-2'
            : ''}"
          style="--i: {i};"
        >
          <Card
            variant="themed"
            class="p-5 transition-colors motion-safe:transition-colors motion-reduce:transition-none cursor-pointer h-full"
          >
            <div class="flex items-start gap-3">
              <ItemIcon
                iconUrl={section.iconUrl}
                iconName={section.iconName}
                iconColor={section.iconColor}
                size={32}
                alt={section.label}
              />
              <div>
                <h2 class="font-display font-bold text-gray-100">{section.label}</h2>
                <p class="text-sm text-[var(--theme-text-muted)] mt-0.5">{section.desc}</p>
              </div>
            </div>
          </Card>
        </button>
      {/each}
    </div>
  </div>
{:else}
  <div class="text-center py-20">
    <p class="text-gray-400">No game selected</p>
    <a href="/" class="text-yellow-500 hover:underline mt-2 inline-block">Back to selector</a>
  </div>
{/if}
