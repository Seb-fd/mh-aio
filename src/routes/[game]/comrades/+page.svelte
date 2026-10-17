<script lang="ts">
  import { selectedGame } from '$lib/stores/game'
  import { api, type ComradeWeapon, type ComradeArmor, type ComradeSkill } from '$lib/api'
  import type { BuddyWeapon, BuddyArmor } from '$lib/api'
  import { normKey } from '$lib/utils/norm'
  import Card from '$lib/components/ui/card.svelte'
  import Badge from '$lib/components/ui/badge.svelte'
  import FilterChip from '$lib/components/ui/filter-chip.svelte'
  import Skeleton from '$lib/components/ui/skeleton.svelte'
  import EmptyState from '$lib/components/ui/empty-state.svelte'
  import SearchField from '$lib/components/ui/search-field.svelte'
  import { createQuery } from '@tanstack/svelte-query'
  import { dbCache, dbRetry, dbRetryDelay, dbErrorText } from '$lib/query'

  const game = $derived($selectedGame)
  const dbId = $derived(game?.dbId)
  // Rise uses buddy gear (Palico/Palamute) instead of Felyne comrades.
  const isBuddy = $derived(game?.id === 'mhr')

  let kind = $state<'weapons' | 'armor' | 'skills'>('weapons')
  let buddyKind = $state<'palico' | 'palamute'>('palico')
  let searchTerm = $state('')

  const weaponsQuery = createQuery(() => ({
    queryKey: ['comrade-weapons', dbId ?? 0],
    queryFn: () => api.getComradeWeapons(dbId!),
    enabled: dbId != null && kind === 'weapons',
    ...dbCache,
    retry: dbRetry,
    retryDelay: dbRetryDelay,
  }))
  const armorQuery = createQuery(() => ({
    queryKey: ['comrade-armor', dbId ?? 0],
    queryFn: () => api.getComradeArmor(dbId!),
    enabled: dbId != null && kind === 'armor',
    ...dbCache,
    retry: dbRetry,
    retryDelay: dbRetryDelay,
  }))
  const skillsQuery = createQuery(() => ({
    queryKey: ['comrade-skills', dbId ?? 0],
    queryFn: () => api.getComradeSkills(dbId!),
    enabled: dbId != null && kind === 'skills',
    ...dbCache,
    retry: dbRetry,
    retryDelay: dbRetryDelay,
  }))

  const weapons = $derived<ComradeWeapon[]>(weaponsQuery.data ?? [])
  const armor = $derived<ComradeArmor[]>(armorQuery.data ?? [])
  const skills = $derived<ComradeSkill[]>(skillsQuery.data ?? [])

  const buddyWeaponsQuery = createQuery(() => ({
    queryKey: ['buddy-weapons', dbId ?? 0, buddyKind],
    queryFn: () => api.getBuddyWeapons(dbId!, buddyKind),
    enabled: dbId != null && isBuddy && kind === 'weapons',
    ...dbCache,
    retry: dbRetry,
    retryDelay: dbRetryDelay,
  }))
  const buddyArmorQuery = createQuery(() => ({
    queryKey: ['buddy-armor', dbId ?? 0, buddyKind],
    queryFn: () => api.getBuddyArmor(dbId!, buddyKind),
    enabled: dbId != null && isBuddy && kind === 'armor',
    ...dbCache,
    retry: dbRetry,
    retryDelay: dbRetryDelay,
  }))
  const buddyWeapons = $derived<BuddyWeapon[]>(buddyWeaponsQuery.data ?? [])
  const buddyArmor = $derived<BuddyArmor[]>(buddyArmorQuery.data ?? [])
  const loading = $derived(
    isBuddy
      ? buddyWeaponsQuery.isPending || buddyArmorQuery.isPending
      : weaponsQuery.isPending || armorQuery.isPending || skillsQuery.isPending,
  )
  const error = $derived(
    isBuddy
      ? dbErrorText(
          false,
          buddyWeaponsQuery.failureCount + buddyArmorQuery.failureCount,
          buddyWeaponsQuery.error ?? buddyArmorQuery.error,
        )
      : dbErrorText(
          false,
          weaponsQuery.failureCount + armorQuery.failureCount + skillsQuery.failureCount,
          weaponsQuery.error ?? armorQuery.error ?? skillsQuery.error,
        ),
  )

  const q = $derived(normKey(searchTerm.trim()))
  const filteredWeapons = $derived(weapons.filter((w) => !q || normKey(w.name_jp).includes(q)))
  const filteredArmor = $derived(armor.filter((a) => !q || normKey(a.name_jp).includes(q)))
  const filteredSkills = $derived(skills.filter((s) => !q || normKey(s.name_jp).includes(q)))
  const filteredBuddyWeapons = $derived(
    buddyWeapons.filter((w) => !q || normKey(w.name).includes(q)),
  )
  const filteredBuddyArmor = $derived(buddyArmor.filter((a) => !q || normKey(a.name).includes(q)))

  function buddyStats(w: BuddyWeapon): string {
    const parts: string[] = []
    if (w.attack_melee != null || w.attack_ranged != null)
      parts.push(`Atk ${w.attack_melee ?? '-'} / ${w.attack_ranged ?? '-'}`)
    const el = w.element_type ?? w.status_type
    const ev = w.element_type ? w.element_melee : w.status_melee
    if (el) parts.push(`${el}${ev != null ? ` ${ev}` : ''}`)
    if (w.affinity != null) parts.push(`${w.affinity > 0 ? '+' : ''}${w.affinity}%`)
    if (w.defense_bonus != null) parts.push(`Def +${w.defense_bonus}`)
    return parts.join(' · ')
  }
</script>

<div class="mx-auto w-full max-w-6xl px-4 py-6">
  <div class="mb-4">
    <h1 class="text-2xl font-bold" style="color: var(--theme-text-accent);">
      {isBuddy ? 'Buddy Gear' : 'Felyne Comrades'}
    </h1>
    <p class="text-sm text-[var(--theme-text-muted)]">
      {isBuddy
        ? 'Palico and Palamute weapons and armor. Stats split melee / ranged.'
        : 'Comrade weapons, armor and skills. Names in Japanese (no official English source exists).'}
    </p>
  </div>

  <div class="mb-4 flex flex-wrap items-center gap-2">
    {#if isBuddy}
      {#each ['palico', 'palamute'] as k}
        <FilterChip active={buddyKind === k} onclick={() => (buddyKind = k as typeof buddyKind)}
          >{k === 'palico' ? 'Palico' : 'Palamute'}</FilterChip
        >
      {/each}
    {/if}
    {#each isBuddy ? ['weapons', 'armor'] : ['weapons', 'armor', 'skills'] as k}
      <FilterChip active={kind === k} onclick={() => (kind = k as typeof kind)}
        >{k === 'weapons' ? 'Weapons' : k === 'armor' ? 'Armor' : 'Skills'}</FilterChip
      >
    {/each}
    <div class="min-w-48 flex-1"><SearchField bind:value={searchTerm} placeholder="Search…" /></div>
  </div>

  {#if loading}
    <div class="space-y-2">
      {#each Array(6) as _}
        <Skeleton class="h-16 w-full" />
      {/each}
    </div>
  {:else if error}
    <EmptyState title="Couldn't load comrades" hint={error} />
  {:else if isBuddy && kind === 'weapons'}
    {#if filteredBuddyWeapons.length === 0}
      <EmptyState title="No weapons found" hint="Try a different search." />
    {:else}
      <p class="mb-2 text-xs text-[var(--theme-text-muted)]">
        {filteredBuddyWeapons.length}
        {buddyKind === 'palico' ? 'Palico' : 'Palamute'} weapons
      </p>
      <div class="space-y-2">
        {#each filteredBuddyWeapons as w}
          <Card class="p-3">
            <div class="flex flex-wrap items-center gap-2">
              <span class="font-semibold text-gray-100">{w.name}</span>
              {#if w.attack_type}<Badge>{w.attack_type}</Badge>{/if}
              <span class="text-xs text-gray-400">{buddyStats(w)}</span>
            </div>
          </Card>
        {/each}
      </div>
    {/if}
  {:else if isBuddy}
    {#if filteredBuddyArmor.length === 0}
      <EmptyState title="No armor found" hint="Try a different search." />
    {:else}
      <p class="mb-2 text-xs text-[var(--theme-text-muted)]">
        {filteredBuddyArmor.length}
        {buddyKind === 'palico' ? 'Palico' : 'Palamute'} pieces
      </p>
      <div class="space-y-2">
        {#each filteredBuddyArmor as a}
          <Card class="p-3">
            <div class="flex flex-wrap items-center gap-2">
              <span class="font-semibold text-gray-100">{a.name}</span>
              {#if a.defense != null}<span class="text-xs text-gray-400">Def {a.defense}</span>{/if}
              <span class="text-xs text-gray-400">
                Res {[a.fire, a.water, a.thunder, a.ice, a.dragon].map((v) => v ?? '-').join(' / ')}
              </span>
            </div>
          </Card>
        {/each}
      </div>
    {/if}
  {:else if kind === 'weapons'}
    {#if filteredWeapons.length === 0}
      <EmptyState title="No weapons found" hint="Try a different search." />
    {:else}
      <p class="mb-2 text-xs text-[var(--theme-text-muted)]">{filteredWeapons.length} weapons</p>
      <div class="space-y-2">
        {#each filteredWeapons as w}
          <Card class="p-3">
            <div class="flex flex-wrap items-center gap-2">
              <span class="font-semibold text-gray-100">{w.name_jp}</span>
              {#if w.style}<Badge
                  >{w.style === '斬' ? 'Slash' : w.style === '打' ? 'Impact' : w.style}</Badge
                >{/if}
              {#if w.rarity != null}<span class="text-xs text-gray-400">R{w.rarity}</span>{/if}
              {#if w.attack != null}<span class="text-xs text-gray-400">Atk {w.attack}</span>{/if}
              {#if w.element}<span class="text-xs text-gray-400">{w.element}</span>{/if}
              {#if w.affinity}<span class="text-xs text-gray-400">{w.affinity}</span>{/if}
              {#if w.price != null}<span class="ml-auto text-xs text-gray-400">{w.price}z</span
                >{/if}
            </div>
          </Card>
        {/each}
      </div>
    {/if}
  {:else if kind === 'armor'}
    {#if filteredArmor.length === 0}
      <EmptyState title="No armor found" hint="Try a different search." />
    {:else}
      <p class="mb-2 text-xs text-[var(--theme-text-muted)]">{filteredArmor.length} pieces</p>
      <div class="space-y-2">
        {#each filteredArmor as a}
          <Card class="p-3">
            <div class="flex flex-wrap items-center gap-2">
              <span class="font-semibold text-gray-100">{a.name_jp}</span>
              <Badge>{a.slot === 'head' ? 'Head' : 'Body'}</Badge>
              {#if a.rarity != null}<span class="text-xs text-gray-400">R{a.rarity}</span>{/if}
              {#if a.defense != null}<span class="text-xs text-gray-400">Def {a.defense}</span>{/if}
              {#if a.materials_jp}<span class="text-xs text-gray-500">{a.materials_jp}</span>{/if}
              {#if a.price != null}<span class="ml-auto text-xs text-gray-400">{a.price}z</span
                >{/if}
            </div>
          </Card>
        {/each}
      </div>
    {/if}
  {:else if filteredSkills.length === 0}
    <EmptyState title="No skills found" hint="Try a different search." />
  {:else}
    <p class="mb-2 text-xs text-[var(--theme-text-muted)]">{filteredSkills.length} skills</p>
    <div class="space-y-2">
      {#each filteredSkills as s}
        <Card class="p-3">
          <div class="flex flex-wrap items-center gap-2">
            <span class="font-semibold text-gray-100">{s.name_jp}</span>
            {#if s.memory}<span class="text-xs text-gray-400">{s.memory}</span>{/if}
            {#if s.cost != null}<span class="text-xs text-gray-400">{s.cost} P</span>{/if}
            {#if s.learn_level}<span class="text-xs text-gray-400">Lv {s.learn_level}</span>{/if}
          </div>
          {#if s.effect_jp}<p class="mt-1 text-xs text-gray-400">{s.effect_jp}</p>{/if}
        </Card>
      {/each}
    </div>
  {/if}
</div>
