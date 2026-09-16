<script lang="ts">
  import { goto } from '$app/navigation'
  import ErrorState from '$lib/components/ui/error-state.svelte'
  import { selectedGame } from '$lib/stores/game'
  import { api, type Skill } from '$lib/api'
  import { normKey } from '$lib/utils/norm'
  import Card from '$lib/components/ui/card.svelte'
  import Badge from '$lib/components/ui/badge.svelte'
  import Skeleton from '$lib/components/ui/skeleton.svelte'
  import EmptyState from '$lib/components/ui/empty-state.svelte'
  import SearchField from '$lib/components/ui/search-field.svelte'
  import FavoriteButton from '$lib/components/favorite-button.svelte'
  import { favorites } from '$lib/stores/favorites'
  import { Star } from '@lucide/svelte'
  import { toolbarTarget } from '$lib/stores/toolbar'
  import { toolbarPortal } from '$lib/actions/toolbar-portal'
  import { captureScrollY, restoreScrollY } from '$lib/utils/scroll-restore'
  import { createQuery } from '@tanstack/svelte-query'
  import { dbCache, dbRetry, dbRetryDelay, dbErrorText } from '$lib/query'
  import type { Snapshot } from './$types.js'

  interface SkillsSnapshot {
    searchTerm: string
    scrollY: number
  }

  export const snapshot: Snapshot<SkillsSnapshot> = {
    capture: () => ({ searchTerm, scrollY: captureScrollY() }),
    restore: (s) => {
      searchTerm = s.searchTerm
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

  const skillsQuery = createQuery(() => ({
    queryKey: ['skills', dbId ?? 0],
    queryFn: () => api.getSkills(dbId!),
    enabled: dbId != null,
    ...dbCache,
    retry: dbRetry,
    retryDelay: dbRetryDelay,
  }))

  const skills = $derived<Skill[]>(skillsQuery.data ?? [])
  const loading = $derived(skillsQuery.isPending)
  const error = $derived(
    dbErrorText(skillsQuery.isPending, skillsQuery.failureCount, skillsQuery.error),
  )
  let searchTerm = $state('')
  let showFavsOnly = $state(false)

  $effect(() => {
    if (game) void favorites.ensure(game.dbId)
  })

  const favKeys = $derived(new Set(game ? [...($favorites.get(game.dbId)?.keys() ?? [])] : []))

  function open(id: number) {
    if (!game) return
    goto(`/${game.id}/skills/${id}`)
  }

  const filtered = $derived(
    skills
      .filter((s) => searchTerm === '' || normKey(s.name).includes(normKey(searchTerm)))
      .filter((s) => !showFavsOnly || favKeys.has(`skill:${s.id}`)),
  )
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-4 md:mb-6">
    <h1 class="fluid-h2 font-bold text-gray-100">Skills</h1>
    <p class="text-sm text-gray-400 mt-1">
      {#if game}
        {game.shortName} · {filtered.length} / {skills.length} skills · Effects per level and synergies
      {:else}
        Select a game first
      {/if}
    </p>
  </div>

  {#if loading}
    <div class="grid grid-cols-1 sm:grid-cols-2 gap-3" aria-busy="true">
      {#each Array(6) as _}
        <Skeleton lines={2} />
      {/each}
    </div>
  {:else if error}
    <ErrorState title="Failed to load skills" {error} />
  {:else if skills.length === 0}
    <EmptyState
      title="No skills found"
      hint={game ? `No skills seeded for ${game.shortName}.` : 'Select a game first.'}
    />
  {:else}
    <div use:toolbarPortal={$toolbarTarget} class="flex flex-col gap-2">
      <div class="flex flex-wrap gap-2 items-center">
        <SearchField
          bind:value={searchTerm}
          placeholder="Search skills..."
          label="Search skills"
          class="sm:w-64"
        />
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
    </div>

    {#if filtered.length === 0}
      <div class="mt-4">
        <EmptyState title="No matches" hint="Try another search term.">
          <button
            type="button"
            onclick={() => (searchTerm = '')}
            class="text-xs px-4 min-h-[44px] rounded-full border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] text-gray-200 hover:border-[var(--theme-border-strong)]"
          >
            Clear search
          </button>
        </EmptyState>
      </div>
    {:else}
      <div class="mt-4 grid grid-cols-1 sm:grid-cols-2 gap-3">
        {#each filtered as skill (skill.id)}
          <div class="relative">
            <button
              onclick={() => open(skill.id)}
              aria-label="Open {skill.name}"
              class="text-left rounded-lg min-h-[44px] w-full focus-visible:outline-none focus-visible:ring-2"
            >
              <Card variant="themed" class="p-4 cursor-pointer">
                <div class="flex items-start justify-between gap-2 mb-1">
                  <h3 class="font-semibold text-gray-100">{skill.name}</h3>
                  {#if skill.max_level}
                    <Badge tone="neutral">Lv 1-{skill.max_level}</Badge>
                  {/if}
                </div>
                {#if skill.description}
                  <p class="text-xs text-gray-400 mt-2 line-clamp-2">{skill.description}</p>
                {/if}
              </Card>
            </button>
            <div class="absolute top-2 right-2">
              <FavoriteButton kind="skill" id={skill.id} name={skill.name} size="sm" />
            </div>
          </div>
        {/each}
      </div>
    {/if}
  {/if}
</div>
