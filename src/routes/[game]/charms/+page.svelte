<script lang="ts">
  import { page } from '$app/state'
  import { selectedGame } from '$lib/stores/game'
  import { api, type CharmView, type Skill, type HunterCharmView } from '$lib/api'
  import { normKey } from '$lib/utils/norm'
  import Card from '$lib/components/ui/card.svelte'
  import Badge from '$lib/components/ui/badge.svelte'
  import FilterChip from '$lib/components/ui/filter-chip.svelte'
  import Skeleton from '$lib/components/ui/skeleton.svelte'
  import EmptyState from '$lib/components/ui/empty-state.svelte'
  import SearchField from '$lib/components/ui/search-field.svelte'
  import MaterialList from '$lib/components/material-list.svelte'
  import { createQuery } from '@tanstack/svelte-query'
  import { dbCache, dbRetry, dbRetryDelay, dbErrorText } from '$lib/query'

  const game = $derived($selectedGame)
  const dbId = $derived(game?.dbId)
  const isMhw = $derived(game?.id === 'mhw')

  const ROMAN = ['', 'I', 'II', 'III', 'IV', 'V']
  const familyStem = (name: string) => name.replace(/\s+(I{1,3}|IV|V)$/, '')

  const skillsQuery = createQuery(() => ({
    queryKey: ['skills', dbId ?? 0],
    queryFn: () => api.getSkills(dbId!),
    enabled: dbId != null,
    ...dbCache,
    retry: dbRetry,
    retryDelay: dbRetryDelay,
  }))
  const skills = $derived<Skill[]>(skillsQuery.data ?? [])

  const initialSkill = $derived.by(() => {
    const raw = page.url.searchParams.get('skill')
    const id = raw == null ? NaN : Number(raw)
    return Number.isFinite(id) ? id : null
  })

  let skillId = $state<number | null>(null)
  let minPoints = $state(10)
  let searchTerm = $state('')
  let typeFilter = $state<string>('all')

  $effect(() => {
    if (skillId == null && initialSkill != null) skillId = initialSkill
  })

  const charmsQuery = createQuery(() => ({
    queryKey: ['charms', dbId ?? 0, skillId, minPoints],
    queryFn: () => api.getCharms(dbId!, skillId, minPoints),
    enabled: dbId != null && skillId != null,
    ...dbCache,
    retry: dbRetry,
    retryDelay: dbRetryDelay,
  }))
  const charms = $derived<CharmView[]>(charmsQuery.data ?? [])
  const loading = $derived(charmsQuery.isPending)
  const error = $derived(
    dbErrorText(charmsQuery.isPending, charmsQuery.failureCount, charmsQuery.error),
  )

  const types = $derived(['all', ...Array.from(new Set(charms.map((c) => c.charm_type))).sort()])
  const q = $derived(normKey(searchTerm.trim()))
  const filtered = $derived(
    charms.filter((c) => {
      if (typeFilter !== 'all' && c.charm_type !== typeFilter) return false
      if (q && !normKey(c.skills.map((s) => s.skill_name).join(' ')).includes(q)) return false
      return true
    }),
  )

  // MHW hunter charms rank tree (I->V + craft materials)
  const hunterQuery = createQuery(() => ({
    queryKey: ['hunter-charms', dbId ?? 0],
    queryFn: () => api.getHunterCharms(dbId!),
    enabled: dbId != null && isMhw,
    ...dbCache,
    retry: dbRetry,
    retryDelay: dbRetryDelay,
  }))
  const hunterCharms = $derived<HunterCharmView[]>(hunterQuery.data ?? [])
  const hunterLoading = $derived(hunterQuery.isPending)
  const hunterError = $derived(
    dbErrorText(hunterQuery.isPending, hunterQuery.failureCount, hunterQuery.error),
  )
  let hunterLevel = $state<number | null>(null)
  let hunterSkill = $state('')
  let hunterSearch = $state('')
  const hq = $derived(normKey(hunterSearch.trim()))

  const hunterSkills = $derived(
    Array.from(
      new Set(
        hunterCharms.flatMap((c) =>
          [c.skill1_name, c.skill2_name].filter((s): s is string => s != null),
        ),
      ),
    ).sort(),
  )
  const hunterFiltered = $derived(
    hunterCharms.filter((c) => {
      if (hunterLevel != null && c.charm_level !== hunterLevel) return false
      if (hunterSkill && c.skill1_name !== hunterSkill && c.skill2_name !== hunterSkill)
        return false
      if (hq && !normKey([c.name, c.skill1_name ?? '', c.skill2_name ?? ''].join(' ')).includes(hq))
        return false
      return true
    }),
  )
  // Group into upgrade families (roots first, ranks ascending)
  const hunterFamilies = $derived.by(() => {
    const byPrev = new Map<number, HunterCharmView[]>()
    for (const c of hunterFiltered) {
      if (c.previous_id != null) {
        const arr = byPrev.get(c.previous_id) ?? []
        arr.push(c)
        byPrev.set(c.previous_id, arr)
      }
    }
    const ids = new Set(hunterFiltered.map((c) => c.id))
    const roots = hunterFiltered
      .filter((c) => c.previous_id == null || !ids.has(c.previous_id))
      .sort((a, b) => a.name.localeCompare(b.name))
    return roots.map((root) => {
      const chain = [root]
      let cur = root
      for (;;) {
        const next = (byPrev.get(cur.id) ?? []).sort((a, b) => a.charm_level - b.charm_level)[0]
        if (!next || chain.some((m) => m.id === next.id)) break
        chain.push(next)
        cur = next
      }
      return { stem: familyStem(root.name), chain }
    })
  })
</script>

<div class="mx-auto w-full max-w-6xl px-4 py-6">
  <div class="mb-4">
    <h1 class="text-2xl font-bold" style="color: var(--theme-text-accent);">Charms</h1>
    <p class="text-sm text-[var(--theme-text-muted)]">
      {#if isMhw}
        Hunter charm rank tree — forge and upgrade from I to V.
      {:else}
        Excavated charm rolls — pick a skill to see matching charms.
      {/if}
    </p>
  </div>

  {#if isMhw}
    <Card class="mb-4 p-4">
      <div class="flex flex-wrap items-end gap-3">
        <label class="flex min-w-52 flex-1 flex-col gap-1 text-xs text-gray-400">
          Skill
          <select
            class="rounded-md border border-gray-700 bg-gray-900 px-2 py-2 text-sm text-gray-100"
            value={hunterSkill}
            onchange={(e) => {
              hunterSkill = (e.target as HTMLSelectElement).value
            }}
          >
            <option value="">All skills</option>
            {#each hunterSkills as s}
              <option value={s}>{s}</option>
            {/each}
          </select>
        </label>
        <div class="flex flex-1 flex-col gap-1">
          <span class="text-xs text-gray-400">Search</span>
          <SearchField bind:value={hunterSearch} placeholder="Filter by name or skill…" />
        </div>
      </div>
      <div class="mt-3 flex flex-wrap gap-2">
        <FilterChip active={hunterLevel == null} onclick={() => (hunterLevel = null)}
          >All ranks</FilterChip
        >
        {#each [1, 2, 3, 4, 5] as lv}
          <FilterChip active={hunterLevel === lv} onclick={() => (hunterLevel = lv)}
            >{ROMAN[lv]}</FilterChip
          >
        {/each}
      </div>
    </Card>

    {#if hunterLoading}
      <div class="space-y-2">
        {#each Array(6) as _}
          <Skeleton class="h-16 w-full" />
        {/each}
      </div>
    {:else if hunterError}
      <EmptyState title="Couldn't load charms" hint={hunterError} />
    {:else if hunterFamilies.length === 0}
      <EmptyState title="No charms found" hint="Try clearing the filters." />
    {:else}
      <p class="mb-2 text-xs text-[var(--theme-text-muted)]">
        {hunterFiltered.length} charms in {hunterFamilies.length} families
      </p>
      <div class="space-y-3">
        {#each hunterFamilies as fam}
          <Card class="p-4">
            <h2 class="mb-2 text-sm font-semibold text-gray-100">{fam.stem}</h2>
            <div class="space-y-2">
              {#each fam.chain as c}
                <div class="rounded-lg border border-gray-800 p-3">
                  <div class="flex flex-wrap items-center gap-2">
                    <span class="text-sm font-medium text-gray-100">{c.name}</span>
                    {#if c.charm_level > 0}
                      <Badge>{ROMAN[c.charm_level]}</Badge>
                    {/if}
                    <Badge>Rarity {c.rarity}</Badge>
                    {#if c.previous_name}
                      <span class="text-xs text-gray-400">← {c.previous_name}</span>
                    {/if}
                    <span class="ml-auto flex flex-wrap gap-2">
                      {#if c.skill1_name}
                        <span class="text-xs text-gray-200"
                          >{c.skill1_name}
                          <span class="font-semibold text-emerald-400">+{c.skill1_level}</span
                          ></span
                        >
                      {/if}
                      {#if c.skill2_name}
                        <span class="text-xs text-gray-200"
                          >{c.skill2_name}
                          <span class="font-semibold text-emerald-400">+{c.skill2_level}</span
                          ></span
                        >
                      {/if}
                    </span>
                  </div>
                  {#if c.materials.length > 0}
                    {@const create = c.materials.filter((m) => m.craft_kind === 'Create')}
                    {@const upgrade = c.materials.filter((m) => m.craft_kind !== 'Create')}
                    {#if create.length > 0}
                      <p class="mb-1 mt-2 text-xs text-gray-400">Forge</p>
                      <MaterialList materials={create} />
                    {/if}
                    {#if upgrade.length > 0}
                      <p class="mb-1 mt-2 text-xs text-gray-400">Upgrade</p>
                      <MaterialList materials={upgrade} />
                    {/if}
                  {/if}
                </div>
              {/each}
            </div>
          </Card>
        {/each}
      </div>
    {/if}
  {:else}
    <Card class="mb-4 p-4">
      <div class="flex flex-wrap items-end gap-3">
        <label class="flex min-w-52 flex-1 flex-col gap-1 text-xs text-gray-400">
          Skill
          <select
            class="rounded-md border border-gray-700 bg-gray-900 px-2 py-2 text-sm text-gray-100"
            value={skillId ?? ''}
            onchange={(e) => {
              const v = (e.target as HTMLSelectElement).value
              skillId = v === '' ? null : Number(v)
            }}
          >
            <option value="">Select a skill…</option>
            {#each skills as s}
              <option value={s.id}>{s.name}</option>
            {/each}
          </select>
        </label>
        <label class="flex flex-col gap-1 text-xs text-gray-400">
          Min points
          <select
            class="rounded-md border border-gray-700 bg-gray-900 px-2 py-2 text-sm text-gray-100"
            value={minPoints}
            onchange={(e) => {
              minPoints = Number((e.target as HTMLSelectElement).value)
            }}
          >
            {#each [5, 8, 10, 13, 15] as p}
              <option value={p}>+{p}</option>
            {/each}
          </select>
        </label>
        <div class="flex flex-1 flex-col gap-1">
          <span class="text-xs text-gray-400">Filter results</span>
          <SearchField bind:value={searchTerm} placeholder="Filter by skill…" />
        </div>
      </div>
      {#if types.length > 2}
        <div class="mt-3 flex flex-wrap gap-2">
          {#each types as t}
            <FilterChip active={typeFilter === t} onclick={() => (typeFilter = t)}>{t}</FilterChip>
          {/each}
        </div>
      {/if}
    </Card>

    {#if skillId == null}
      <EmptyState
        title="Pick a skill"
        hint="Choose a skill above to list excavated charms granting it."
      />
    {:else if loading}
      <div class="space-y-2">
        {#each Array(6) as _}
          <Skeleton class="h-16 w-full" />
        {/each}
      </div>
    {:else if error}
      <EmptyState title="Couldn't load charms" hint={error} />
    {:else if filtered.length === 0}
      <EmptyState title="No charms found" hint="Try lowering min points." />
    {:else}
      <p class="mb-2 text-xs text-[var(--theme-text-muted)]">{filtered.length} charms</p>
      <div class="space-y-2">
        {#each filtered as c}
          <Card class="p-3">
            <div class="flex flex-wrap items-center gap-2">
              <Badge>{c.charm_type}</Badge>
              {#if c.charm_table != null}
                <span class="text-xs text-gray-400">table {c.charm_table}</span>
              {/if}
              <span class="text-xs text-gray-400">{c.slots} slot{c.slots === 1 ? '' : 's'}</span>
              <span class="ml-auto flex flex-wrap gap-2">
                {#each c.skills as s}
                  <span class="text-xs text-gray-200"
                    >{s.skill_name}
                    <span class="font-semibold text-emerald-400">+{s.points}</span></span
                  >
                {/each}
              </span>
            </div>
          </Card>
        {/each}
      </div>
    {/if}
  {/if}
</div>
