<script lang="ts">
  import { goto } from '$app/navigation'
  import ErrorState from '$lib/components/ui/error-state.svelte'
  import { selectedGame } from '$lib/stores/game'
  import { api, type Quest } from '$lib/api'
  import { normKey } from '$lib/utils/norm'
  import { rankTone, fallbackLabel } from '$lib/utils/mh'
  import Card from '$lib/components/ui/card.svelte'
  import Badge from '$lib/components/ui/badge.svelte'
  import Skeleton from '$lib/components/ui/skeleton.svelte'
  import EmptyState from '$lib/components/ui/empty-state.svelte'
  import ItemIcon from '$lib/components/item-icon.svelte'
  import SearchField from '$lib/components/ui/search-field.svelte'
  import FavoriteButton from '$lib/components/favorite-button.svelte'
  import { favorites } from '$lib/stores/favorites'
  import { ChevronRight, Star } from '@lucide/svelte'
  import { toolbarTarget } from '$lib/stores/toolbar'
  import { toolbarPortal } from '$lib/actions/toolbar-portal'
  import { captureScrollY, restoreScrollY } from '$lib/utils/scroll-restore'
  import { createQuery } from '@tanstack/svelte-query'
  import { dbCache, dbRetry, dbRetryDelay, dbErrorText } from '$lib/query'
  import type { Snapshot } from './$types.js'

  interface QuestsSnapshot {
    searchTerm: string
    hubFilter: string
    expandedCategories: string[]
    expandedStars: string[]
    userToggled: boolean
    scrollY: number
  }

  export const snapshot: Snapshot<QuestsSnapshot> = {
    capture: () => ({
      searchTerm,
      hubFilter,
      expandedCategories: [...expandedCategories],
      expandedStars: [...expandedStars],
      userToggled,
      scrollY: captureScrollY(),
    }),
    restore: (s) => {
      searchTerm = s.searchTerm
      hubFilter = s.hubFilter
      expandedCategories = new Set(s.expandedCategories)
      expandedStars = new Set(s.expandedStars)
      userToggled = s.userToggled
      pendingScrollY = s.scrollY
    },
  }

  let pendingScrollY = $state<number | null>(null)
  $effect(() => {
    if (!loading && pendingScrollY != null) {
      const y = pendingScrollY
      pendingScrollY = null
      restoreScrollY(y)
    }
  })

  const game = $derived($selectedGame)
  const dbId = $derived(game?.dbId)

  const questsQuery = createQuery(() => ({
    queryKey: ['quests', dbId ?? 0],
    queryFn: () => api.getQuests(dbId!),
    enabled: dbId != null,
    ...dbCache,
    retry: dbRetry,
    retryDelay: dbRetryDelay,
  }))

  const quests = $derived<Quest[]>(questsQuery.data ?? [])
  const loading = $derived(questsQuery.isPending)
  const error = $derived(
    dbErrorText(questsQuery.isPending, questsQuery.failureCount, questsQuery.error),
  )
  let hubFilter = $state<string>('elder')
  let searchTerm = $state('')
  let showFavsOnly = $state(false)

  $effect(() => {
    if (game) void favorites.ensure(game.dbId)
  })

  const favKeys = $derived(new Set(game ? [...($favorites.get(game.dbId)?.keys() ?? [])] : []))

  const hubMeta: Record<string, { label: string; sub: string }> = {
    low_high: { label: 'Low / High Rank', sub: '★1-9 · All quests' },
    master: { label: 'Master Rank', sub: 'MR★1-6 · Iceborne' },
    siege: { label: 'Siege', sub: 'Kulve Taroth · HR/Master' },
    // legacy / other games
    elder: { label: 'Village Elder', sub: 'Low Rank Village' },
    nekoto: { label: 'Village Nekoto', sub: 'High Rank Village' },
    guild_low: { label: 'Guild Low', sub: '★1-5' },
    guild_high: { label: 'Guild High', sub: '★6-8' },
    guild_g: { label: 'Guild G', sub: 'G★1-3' },
    event: { label: 'Event Quests', sub: 'Downloadable' },
    challenge: { label: 'Challenge Quests', sub: 'Arena challenges' },
    training: { label: 'Training School', sub: 'Learn the weapons' },
    treasure: { label: 'Treasure Hunt', sub: 'Gather & deliver' },
    hot_spring: { label: 'Hot Spring', sub: 'Bath quests' },
    drink: { label: 'Drink Quests', sub: 'Felyne bar' },
    nyanta: { label: 'Nyanta Quests', sub: 'Farm felyne' },
    other: { label: 'Other', sub: 'Misc' },
  }

  const categoryMeta: Record<string, { label: string; sub: string }> = {
    assigned: { label: 'Assignments', sub: 'Story' },
    optional: { label: 'Optional', sub: 'Repeatable' },
    event: { label: 'Events', sub: 'Collabs & festivals' },
    arena: { label: 'Arena', sub: 'Arena battles' },
    challenge: { label: 'Challenge', sub: 'Challenge quests' },
    special: { label: 'Special', sub: 'Special investigations' },
    siege: { label: 'Siege', sub: 'Siege quests' },
  }
  const categoryOrder = ['assigned', 'optional', 'event', 'arena', 'challenge', 'special', 'siege']

  const hubs = $derived(
    Array.from(new Set(quests.map((q) => q.hub).filter((h): h is string => !!h))).sort((a, b) => {
      const order = [
        'low_high',
        'master',
        'siege',
        'elder',
        'nekoto',
        'guild_low',
        'guild_high',
        'guild_g',
        'event',
        'challenge',
        'training',
        'treasure',
        'hot_spring',
        'drink',
        'nyanta',
        'other',
      ]
      return order.indexOf(a) - order.indexOf(b)
    }),
  )

  // Default to first hub when quests load
  $effect(() => {
    if (quests.length > 0 && !hubs.includes(hubFilter)) {
      hubFilter = hubs[0] ?? 'elder'
    }
  })

  const hubCounts = $derived(
    hubs.reduce(
      (acc, h) => {
        acc[h] = quests.filter((q) => q.hub === h).length
        return acc
      },
      {} as Record<string, number>,
    ),
  )

  const filtered = $derived(
    quests
      .filter((q) => q.hub === hubFilter)
      .filter((q) => searchTerm === '' || normKey(q.name).includes(normKey(searchTerm)))
      .filter((q) => !showFavsOnly || favKeys.has(`quest:${q.id}`)),
  )

  // Quest categories (assigned/optional/...) only exist for MHW. MH2G/MHP3rd
  // quests always have category NULL, so those hubs skip the category level.
  const hasCategories = $derived(filtered.some((q) => q.category != null && q.category !== ''))

  interface StarGroup {
    stars: number | null
    rank: string | null
    label: string
    items: Quest[]
  }
  interface CategoryGroup {
    category: string
    label: string
    sub: string
    items: Quest[]
    starGroups: StarGroup[]
  }

  let expandedCategories = $state<Set<string>>(new Set())
  let expandedStars = $state<Set<string>>(new Set())
  let userToggled = $state(false)

  // Shared rank+stars grouping for flat hubs (siege + games without quest
  // categories like MH2G/MHP3rd, where category is always NULL)
  function buildStarGroups(
    items: Quest[],
    label: (rank: string, stars: number | null) => string,
  ): StarGroup[] {
    const map = new Map<string, Quest[]>()
    for (const q of items) {
      const key = `${q.rank ?? ''}|${q.stars ?? ''}`
      const arr = map.get(key) ?? []
      arr.push(q)
      map.set(key, arr)
    }
    const starGroups: StarGroup[] = []
    for (const [key, gItems] of map.entries()) {
      const [rank, starsStr] = key.split('|')
      const stars = starsStr ? parseInt(starsStr) : null
      const rankLabel = rank || gItems[0]?.rank || ''
      starGroups.push({ stars, rank: rankLabel, label: label(rankLabel, stars), items: gItems })
    }
    starGroups.sort((a, b) => {
      const rankOrder: Record<string, number> = { Low: 0, High: 1, Master: 2 }
      const ra = rankOrder[a.rank ?? ''] ?? 99
      const rb = rankOrder[b.rank ?? ''] ?? 99
      if (ra !== rb) return ra - rb
      return (a.stars ?? -1) - (b.stars ?? -1)
    })
    return starGroups
  }

  // Category -> Stars (priority: category before stars)
  // For low_high, differentiate Low vs High with same stars (★6 appears as Low and High separately)
  // Hubs without categories (MH2G/MHP3rd) and siege skip this level (see flatStarGroups)
  const categoryGroups = $derived.by<CategoryGroup[]>(() => {
    if (hubFilter === 'siege' || !hasCategories) return []
    const map = new Map<string, Quest[]>()
    for (const q of filtered) {
      const cat = q.category ?? 'optional'
      const arr = map.get(cat) ?? []
      arr.push(q)
      map.set(cat, arr)
    }
    const groups: CategoryGroup[] = []
    for (const [cat, items] of map.entries()) {
      const meta = categoryMeta[cat] ?? { label: cat, sub: '' }
      // stars inside category — for low_high split by rank+stars to avoid mixing Low★6/High★6
      const starMap = new Map<string, Quest[]>()
      for (const q of items) {
        const k = `${q.rank ?? ''}|${q.stars ?? ''}`
        const a = starMap.get(k) ?? []
        a.push(q)
        starMap.set(k, a)
      }
      const starGroups: StarGroup[] = []
      for (const [key, sItems] of starMap.entries()) {
        const [rank, starsStr] = key.split('|')
        const stars = starsStr ? parseInt(starsStr) : null
        const rankLabel = rank || sItems[0]?.rank || ''
        let label: string
        if (stars == null) label = rankLabel || '★'
        else if (hubFilter === 'low_high') label = `${rankLabel} ★${stars}`
        else label = groupLabel(stars)
        starGroups.push({ stars, rank: rankLabel, label, items: sItems })
      }
      starGroups.sort((a, b) => {
        const rankOrder: Record<string, number> = { Low: 0, High: 1, Master: 2 }
        const ra = rankOrder[a.rank ?? ''] ?? 99
        const rb = rankOrder[b.rank ?? ''] ?? 99
        if (ra !== rb) return ra - rb
        return (a.stars ?? -1) - (b.stars ?? -1)
      })
      groups.push({ category: cat, label: meta.label, sub: meta.sub, items, starGroups })
    }
    groups.sort((a, b) => categoryOrder.indexOf(a.category) - categoryOrder.indexOf(b.category))
    return groups
  })

  // Siege flat: stars differentiating HR vs Master (rank+stars key)
  const siegeStarGroups = $derived.by<StarGroup[]>(() => {
    if (hubFilter !== 'siege') return []
    return buildStarGroups(filtered, (rank, stars) =>
      stars == null ? rank || '★' : siegeLabel(rank, stars),
    )
  })

  // Flat hubs without quest categories (MH2G/MHP3rd): stars directly, no category level
  const flatStarGroups = $derived.by<StarGroup[]>(() => {
    if (hubFilter === 'siege' || hasCategories) return []
    return buildStarGroups(filtered, (rank, stars) =>
      stars == null
        ? rank || '★'
        : hubFilter === 'low_high'
          ? `${rank} ★${stars}`
          : groupLabel(stars),
    )
  })

  const defaultOpenCategory = $derived(categoryGroups[0]?.category ?? null)
  const defaultOpenStars = $derived(
    categoryGroups[0]?.starGroups[0]?.stars ??
      siegeStarGroups[0]?.stars ??
      flatStarGroups[0]?.stars ??
      null,
  )

  function groupLabel(stars: number): string {
    if (hubFilter === 'guild_g') return `G★${stars}`
    if (hubFilter === 'master') return `MR★${stars}`
    if (hubFilter === 'low_high' && stars > 5) return `★${stars} (High)`
    if (hubFilter === 'low_high' && stars <= 5) return `★${stars} (Low)`
    return `★${stars}`
  }
  function siegeLabel(rank: string, stars: number): string {
    if (rank === 'Master') return `Master ★${stars}`
    if (rank === 'High') return `High ★${stars}`
    if (rank === 'Low') return `Low ★${stars}`
    return `★${stars}`
  }

  function isCategoryOpen(cat: string): boolean {
    if (!userToggled) return true
    return expandedCategories.has(cat)
  }
  function isStarsOpen(cat: string, stars: number | null, rank: string | null = null): boolean {
    if (!userToggled) return true
    return expandedStars.has(`${cat}|${rank ?? ''}|${stars}`)
  }
  function toggleCategory(cat: string) {
    const next = new Set(expandedCategories)
    if (next.has(cat)) next.delete(cat)
    else next.add(cat)
    expandedCategories = next
    userToggled = true
  }
  function toggleStars(cat: string, stars: number | null, rank: string | null = null) {
    const key = `${cat}|${rank ?? ''}|${stars}`
    const next = new Set(expandedStars)
    if (next.has(key)) next.delete(key)
    else next.add(key)
    expandedStars = next
    userToggled = true
  }
  // siege flat toggle uses stars key only
  function toggleSiegeStars(stars: number | null, rank: string | null) {
    const key = `siege|${rank}|${stars}`
    const next = new Set(expandedStars)
    if (next.has(key)) next.delete(key)
    else next.add(key)
    expandedStars = next
    userToggled = true
  }
  function isSiegeOpen(stars: number | null, rank: string | null): boolean {
    if (!userToggled) return true
    return expandedStars.has(`siege|${rank}|${stars}`)
  }

  function open(id: number) {
    if (!game) return
    goto(`/${game.id}/quests/${id}`)
  }

  function starsLabel(q: Quest): string {
    if (q.stars == null) return ''
    if (q.hub === 'guild_g') return `G★${q.stars}`
    if (q.hub === 'master') return `MR★${q.stars}`
    if (q.hub === 'siege') return siegeLabel(q.rank ?? '', q.stars)
    return `★${q.stars}`
  }

  // Hub icons live in per-game sets: MHW hubs under mhw, the classic
  // village/guild hubs under mhfu (unknown falls back to mhfu/unknown.png,
  // and ItemIcon renders initials if a file is still missing).
  function hubIconUrl(hub: string | null | undefined): string {
    const slug = (hub ?? 'unknown').replace('_', '-')
    const base =
      hub === 'low_high' || hub === 'master' || hub === 'siege'
        ? '/icons/mhw/quests/hubs'
        : '/icons/mhfu/quests/hubs'
    return `${base}/${slug}.png`
  }
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-4 md:mb-6">
    <h1 class="fluid-h2 font-bold text-gray-100">Quests</h1>
    <p class="text-sm text-[var(--theme-text-muted)] mt-1">
      {#if game}
        {game.shortName} · {quests.length} quests · {hubs.length} hubs · Key quests, rewards and drop
        rates
      {:else}
        Select a game first
      {/if}
    </p>
  </div>

  {#if loading}
    <div class="space-y-2" aria-busy="true">
      {#each Array(5) as _}
        <Skeleton lines={2} />
      {/each}
    </div>
  {:else if error}
    <ErrorState title="Failed to load quests" {error} />
  {:else if quests.length === 0}
    <EmptyState
      title="No quests found"
      hint={game ? `No quests seeded for ${game.shortName}.` : 'Select a game first.'}
    />
  {:else}
    <!-- Hub + search (fixed sub-header portal, like other sections) -->
    <div use:toolbarPortal={$toolbarTarget} class="flex flex-wrap gap-2 items-center">
      <SearchField
        bind:value={searchTerm}
        placeholder="Search quests..."
        label="Search quests"
        class="sm:w-56"
      />
      <select
        bind:value={hubFilter}
        aria-label="Filter by hub"
        class="px-3 rounded-full bg-[var(--theme-bg-surface)] border border-[var(--theme-border)] text-gray-300 focus:outline-none focus:border-[var(--theme-border-strong)] min-h-[44px] sm:min-h-[36px] text-base sm:text-xs"
      >
        {#each hubs as hub}
          {@const meta = hubMeta[hub] ?? { label: hub, sub: '' }}
          <option value={hub}>{meta.label} ({hubCounts[hub] ?? 0})</option>
        {/each}
      </select>
      <button
        type="button"
        onclick={() => (showFavsOnly = !showFavsOnly)}
        aria-pressed={showFavsOnly}
        title="Show favorites only"
        class="inline-flex items-center gap-1.5 px-4 min-h-[44px] sm:min-h-[36px] rounded-full border text-xs font-medium focus-visible:outline-none focus-visible:ring-2 {showFavsOnly
          ? 'border-[var(--theme-accent)]/50 bg-[var(--theme-accent)]/10 text-[var(--theme-accent)]'
          : 'border-[var(--theme-border)] bg-[var(--theme-bg-surface)] text-gray-400 hover:text-gray-200'}"
      >
        <Star
          class="h-3.5 w-3.5 {showFavsOnly ? 'fill-[var(--theme-accent)]' : ''}"
          aria-hidden="true"
        />
        Favorites
      </button>
    </div>

    {@const meta = hubMeta[hubFilter] ?? { label: fallbackLabel(hubFilter), sub: '' }}

    {#snippet questRow(quest: Quest, i: number)}
      <div class="relative">
        <button
          onclick={() => open(quest.id)}
          aria-label="Open {quest.name}"
          class="group w-full text-left rounded-lg focus-visible:outline-none focus-visible:ring-2"
          style="--i: {i};"
        >
          <Card variant="themed" class="p-3 cursor-pointer">
            <div class="flex items-center gap-3">
              <ItemIcon
                iconUrl={quest.icon_url}
                iconName={quest.icon_name}
                iconColor={quest.icon_color}
                size={32}
                alt=""
              />
              <div class="min-w-0 flex-1">
                <h3 class="font-semibold text-gray-100 text-sm truncate">{quest.name}</h3>
                <p class="text-[11px] text-[var(--theme-text-muted)] mt-0.5 truncate">
                  {[starsLabel(quest), quest.type ?? '', quest.category ?? '']
                    .filter((s) => s !== '')
                    .join(' · ')}
                </p>
              </div>
              {#if quest.is_key_quest}<Badge tone="key">Key</Badge>{/if}
              {#if quest.is_urgent}<Badge tone="urgent">Urgent</Badge>{/if}
              {#if quest.rank}
                <Badge tone={rankTone(quest.rank)}>{quest.rank}</Badge>
              {/if}
              <ChevronRight
                class="h-4 w-4 shrink-0 text-[var(--theme-text-muted)] opacity-0 transition-opacity group-hover:opacity-100 group-focus-visible:opacity-100"
                aria-hidden="true"
              />
            </div>
          </Card>
        </button>
        <div class="absolute top-2 right-2">
          <FavoriteButton kind="quest" id={quest.id} name={quest.name} size="sm" />
        </div>
      </div>
    {/snippet}

    <div class="mb-6">
      <div class="flex items-center gap-2 mb-3">
        <ItemIcon
          iconUrl={hubIconUrl(hubFilter)}
          iconName={hubFilter}
          iconColor="Gray"
          size={18}
          alt={meta.label}
        />
        <h2 class="text-sm font-semibold text-gray-200">{meta.label}</h2>
        <span class="text-[11px] text-[var(--theme-text-muted)]"
          >{meta.sub} · {filtered.length}</span
        >
        <div class="flex-1 h-px bg-[var(--theme-border)] ml-2"></div>
      </div>
      {#if filtered.length === 0}
        <EmptyState title="No quests in this hub" hint="Try another search term or hub." />
      {:else if hubFilter === 'siege' || !hasCategories}
        <!-- Flat by stars: siege (HR vs Master) + hubs without categories (MH2G/MHP3rd) -->
        <div class="space-y-4">
          {#each hubFilter === 'siege' ? siegeStarGroups : flatStarGroups as group}
            <section>
              <button
                onclick={() => toggleSiegeStars(group.stars, group.rank)}
                aria-expanded={isSiegeOpen(group.stars, group.rank)}
                aria-controls={'siege-group-' +
                  (group.rank ?? 'none') +
                  '-' +
                  (group.stars ?? 'none')}
                class="w-full flex items-center gap-2 px-1 min-h-[44px] py-1.5 text-left rounded-md focus-visible:outline-none focus-visible:ring-2"
              >
                <ChevronRight
                  class="h-3.5 w-3.5 shrink-0 text-[var(--theme-text-muted)] transition-transform {isSiegeOpen(
                    group.stars,
                    group.rank,
                  )
                    ? 'rotate-90'
                    : ''}"
                  aria-hidden="true"
                />
                <span class="text-sm font-semibold text-gray-100">{group.label}</span>
                <span class="text-[10px] text-[var(--theme-text-muted)]"
                  >({group.items.length})</span
                >
                <div class="flex-1 h-px bg-[var(--theme-border)] ml-1"></div>
              </button>
              {#if isSiegeOpen(group.stars, group.rank)}
                <div
                  id={'siege-group-' + (group.rank ?? 'none') + '-' + (group.stars ?? 'none')}
                  class="mt-2 space-y-2"
                >
                  {#each group.items as quest, i}
                    {@render questRow(quest, i)}
                  {/each}
                </div>
              {/if}
            </section>
          {/each}
        </div>
      {:else}
        <!-- low_high / master : category -> stars (flat headers, no nested boxes) -->
        <div class="space-y-4">
          {#each categoryGroups as catGroup}
            <section>
              <button
                onclick={() => toggleCategory(catGroup.category)}
                aria-expanded={isCategoryOpen(catGroup.category)}
                aria-controls={'cat-' + catGroup.category}
                class="w-full flex items-center gap-2 px-1 min-h-[44px] py-1.5 text-left rounded-md focus-visible:outline-none focus-visible:ring-2"
              >
                <ChevronRight
                  class="h-3.5 w-3.5 shrink-0 text-[var(--theme-text-muted)] transition-transform {isCategoryOpen(
                    catGroup.category,
                  )
                    ? 'rotate-90'
                    : ''}"
                  aria-hidden="true"
                />
                <span class="text-sm font-semibold text-gray-100">{catGroup.label}</span>
                <span class="text-[11px] text-[var(--theme-text-muted)]">{catGroup.sub}</span>
                <span class="text-[10px] text-[var(--theme-text-muted)]"
                  >({catGroup.items.length})</span
                >
                <div class="flex-1 h-px bg-[var(--theme-border)] ml-1"></div>
              </button>
              {#if isCategoryOpen(catGroup.category)}
                <div id={'cat-' + catGroup.category} class="mt-1 space-y-3">
                  {#each catGroup.starGroups as group}
                    <div>
                      <button
                        onclick={() => toggleStars(catGroup.category, group.stars, group.rank)}
                        aria-expanded={isStarsOpen(catGroup.category, group.stars, group.rank)}
                        aria-controls={'quest-group-' +
                          catGroup.category +
                          '-' +
                          (group.rank ?? 'none') +
                          '-' +
                          (group.stars ?? 'none')}
                        class="w-full flex items-center gap-2 px-1 min-h-[44px] py-1.5 text-left rounded-md focus-visible:outline-none focus-visible:ring-2"
                      >
                        <ChevronRight
                          class="h-3 w-3 shrink-0 text-[var(--theme-text-muted)] transition-transform {isStarsOpen(
                            catGroup.category,
                            group.stars,
                            group.rank,
                          )
                            ? 'rotate-90'
                            : ''}"
                          aria-hidden="true"
                        />
                        <span class="text-xs font-medium text-gray-200">{group.label}</span>
                        <span class="text-[10px] text-[var(--theme-text-muted)]"
                          >({group.items.length})</span
                        >
                      </button>
                      {#if isStarsOpen(catGroup.category, group.stars, group.rank)}
                        <div
                          id={'quest-group-' +
                            catGroup.category +
                            '-' +
                            (group.rank ?? 'none') +
                            '-' +
                            (group.stars ?? 'none')}
                          class="mt-1.5 space-y-2"
                        >
                          {#each group.items as quest, i}
                            {@render questRow(quest, i)}
                          {/each}
                        </div>
                      {/if}
                    </div>
                  {/each}
                </div>
              {/if}
            </section>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</div>
