<script lang="ts">
  import { goto } from '$app/navigation'
  import { selectedGame } from '$lib/stores/game'
  import Card from '$lib/components/ui/card.svelte'
  import ItemIcon from '$lib/components/item-icon.svelte'

  const game = $derived($selectedGame)

  const sections = [
    {
      href: '/monsters',
      label: 'Monsters',
      icon: '🐉',
      iconUrl: '/icons/mhfu/quests/hunt.png',
      iconName: 'Hunting',
      iconColor: 'Red',
      desc: 'Weaknesses, materials and tips',
    },
    {
      href: '/weapons',
      label: 'Weapons',
      icon: '⚔️',
      iconUrl: '/icons/mhfu/quests/slay.png',
      iconName: 'Slaying',
      iconColor: 'Orange',
      desc: 'Stats, elements and upgrade tree',
    },
    {
      href: '/armor',
      label: 'Armor',
      icon: '🛡️',
      iconUrl: '/icons/mhfu/armor/head.png',
      iconName: 'Head',
      iconColor: 'Gray',
      desc: 'Sets, skills and resistances',
    },
    {
      href: '/quests',
      label: 'Quests',
      icon: '📜',
      iconUrl: '/icons/mhfu/quests/event.png',
      iconName: 'Event',
      iconColor: 'Gray',
      desc: 'Key quests, rewards and drop rates',
    },
    {
      href: '/items',
      label: 'Items',
      icon: '🎒',
      iconUrl: '/icons/mhfu/MH4G-Medicine_Icon_Green.png',
      iconName: 'Medicine',
      iconColor: 'Green',
      desc: 'Materials, consumables and locations',
    },
    {
      href: '/skills',
      label: 'Skills',
      icon: '✨',
      iconUrl: '/icons/mhfu/skills/skills.png',
      iconName: 'Skills',
      iconColor: 'Violet',
      desc: 'Effects per level and synergies',
    },
    {
      href: '/decorations',
      label: 'Decorations',
      icon: '💎',
      iconUrl: '/icons/mhfu/decorations/ItemIcon017i.png',
      iconName: 'Attack',
      iconColor: 'Red',
      desc: 'Jewels, slots and crafting materials',
    },
    {
      href: '/builds',
      label: 'Builds',
      icon: '🔧',
      iconUrl: '/icons/mhfu/builds/forging.png',
      iconName: 'Forging',
      iconColor: 'Gray',
      desc: 'Suggestions and planner',
    },
  ]

  function navigate(href: string) {
    if (!game) return
    goto(`/${game.id}${href}`)
  }
</script>

{#if game}
  <div class="max-w-5xl mx-auto">
    <div class="mb-8 flex items-center gap-4">
      {#if game.iconUrl}
        <img src={game.iconUrl} alt={game.name} width="56" height="56" class="w-14 h-14 object-contain rounded-lg shrink-0" loading="lazy" />
      {/if}
      <div>
        <h1 class="text-3xl font-bold {game.color} mb-1">{game.name}</h1>
        <p class="text-gray-400">{game.platform} · {game.year}</p>
      </div>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each sections as section}
        <button onclick={() => navigate(section.href)} class="text-left">
          <Card
            class="p-5 border border-gray-800 hover:border-gray-700 hover:bg-gray-800/50 transition-all cursor-pointer h-full"
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
                <h2 class="font-semibold text-gray-100">{section.label}</h2>
                <p class="text-sm text-gray-500 mt-0.5">{section.desc}</p>
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
