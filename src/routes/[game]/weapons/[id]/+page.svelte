<script lang="ts">
  import { page } from '$app/state'
  import ErrorState from '$lib/components/ui/error-state.svelte'
  import { goto } from '$app/navigation'
  import { selectedGame } from '$lib/stores/game'
  import { api, type WeaponDetail, type Weapon } from '$lib/api'
  import DetailHeader from '$lib/components/detail-header.svelte'
  import ItemIcon from '$lib/components/item-icon.svelte'
  import MaterialList from '$lib/components/material-list.svelte'
  import Skeleton from '$lib/components/ui/skeleton.svelte'
  import EmptyState from '$lib/components/ui/empty-state.svelte'
  import {
    elementColor,
    sharpnessValues,
    SHARP_COLORS_ARR as SHARP_COLORS,
    SHARP_LABELS,
    parseUpgradeParent,
    buildWeaponForest,
  } from '$lib/utils/mh'
  import { Hammer, ArrowUp, CornerDownRight } from '@lucide/svelte'

  const id = $derived(Number(page.params.id))
  let weapon = $state<WeaponDetail | null>(null)
  let allWeapons = $state<Weapon[]>([])
  let loading = $state(true)
  let error = $state<string | null>(null)

  $effect(() => {
    if (!id || Number.isNaN(id)) return
    loading = true
    error = null
    let done = 0
    const finish = () => {
      if (++done >= 2) loading = false
    }
    api
      .getWeaponDetail(id)
      .then((data) => {
        weapon = data
      })
      .catch((e) => {
        error = String(e)
      })
      .finally(finish)
    const g = $selectedGame
    if (g?.dbId != null) {
      api
        .getWeapons(g.dbId)
        .then((data) => {
          allWeapons = data
        })
        .catch(() => {})
        .finally(finish)
    } else {
      finish()
    }
  })

  // ID-keyed forest shared by the walk-up and the subtree (names repeat
  // across variants, e.g. Wilds Artian x3 — see mh.ts buildWeaponForest).
  const forest = $derived(
    buildWeaponForest(allWeapons, (a, b) => (a.attack ?? 0) - (b.attack ?? 0)),
  )

  const parentName = $derived(parseUpgradeParent(weapon?.upgrade_path))

  const baseWeapon = $derived<Weapon | null>(
    weapon ? (forest.byId.get(forest.parentOf.get(weapon.id) ?? -1) ?? null) : null,
  )

  // Flat rows, built iteratively — the subtree from the furthest ancestor
  // is rendered without recursion (no call-stack overflow at any depth).
  interface DetailRow {
    weapon: Weapon
    depth: number
  }

  const MAX_INDENT = 8

  // Full subtree rooted at the current weapon's furthest ancestor (tree-style like weapon trees)
  const treeRows = $derived.by<DetailRow[]>(() => {
    if (!weapon) return []
    // walk up to the root (visited-set + step cap: stale rows can form cycles)
    let root: Weapon = weapon
    const seenUp = new Set<number>([weapon.id])
    let guard = 0
    while (guard++ < 40) {
      const pid = forest.parentOf.get(root.id)
      if (pid == null) break
      const parent = forest.byId.get(pid)
      if (!parent || seenUp.has(parent.id)) break
      seenUp.add(parent.id)
      root = parent
    }
    // iterative DFS from root (no recursion anywhere)
    const rows: DetailRow[] = []
    const emitted = new Set<number>()
    const stack: { w: Weapon; depth: number; childIdx: number; kids: Weapon[] }[] = []
    const push = (w: Weapon, depth: number) => {
      const kids = (forest.childrenOf.get(w.id) ?? []).filter((c) => !emitted.has(c.id))
      emitted.add(w.id)
      rows.push({ weapon: w, depth })
      stack.push({ w, depth, childIdx: 0, kids })
    }
    push(root, 0)
    while (stack.length > 0) {
      const top = stack[stack.length - 1]
      if (top.childIdx >= top.kids.length) {
        stack.pop()
        continue
      }
      const child = top.kids[top.childIdx++]
      if (emitted.has(child.id)) continue
      push(child, top.depth + 1)
    }
    return rows
  })

  const isUpgrade = $derived(parentName != null)

  // Helpers now from $lib/utils/mh (DRY)
  const sharpnessSegments = sharpnessValues
  const sharpTotal = $derived(sharpnessSegments(weapon?.sharpness).reduce((a, b) => a + b, 0))
  const sharpA11y = $derived(
    sharpnessSegments(weapon?.sharpness)
      .map((seg, i) => (seg > 0 ? `${SHARP_LABELS[i] ?? `level ${i + 1}`}: ${seg}` : null))
      .filter((x): x is string => x !== null)
      .join(', '),
  )

  function openWeapon(wid: number | null | undefined) {
    if (wid == null) return
    const gameId = page.params.game
    if (gameId) goto(`/${gameId}/weapons/${wid}`)
  }
</script>

<div class="max-w-5xl mx-auto numbered-sections">
  {#if loading}
    <div class="space-y-3" aria-busy="true">
      <Skeleton lines={2} />
      <Skeleton lines={3} />
    </div>
  {:else if error}
    <ErrorState title="Failed to load weapon" {error} />
  {:else if !weapon}
    <EmptyState title="Weapon not found" hint="It may belong to another game." />
  {:else}
    <DetailHeader
      title={weapon.name}
      subtitle={weapon.weapon_type}
      iconUrl={weapon.icon_url}
      favKind="weapon"
      favId={weapon.id}
      tags={[
        {
          label: `Rarity ${weapon.rarity ?? 1}`,
          color: 'bg-[var(--theme-bg-elevated)] text-gray-300 border-[var(--theme-border)]',
        },
        ...(weapon.element_type && weapon.element_type !== ''
          ? [
              {
                label: `${weapon.element_type} ${weapon.element_value ?? 0}`,
                color: `bg-[var(--theme-bg-elevated)] ${elementColor(weapon.element_type)} border-[var(--theme-border-strong)]`,
              },
            ]
          : []),
      ]}
    />

    <div class="stat-grid gap-3 mb-8">
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-400">Attack</p>
        <p class="text-2xl font-bold text-gray-100 mt-1">{weapon.attack ?? 0}</p>
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-400">Affinity</p>
        <p
          class="text-2xl font-bold mt-1"
          class:text-emerald-400={(weapon.affinity ?? 0) > 0}
          class:text-red-400={(weapon.affinity ?? 0) < 0}
          class:text-gray-100={(weapon.affinity ?? 0) === 0}
        >
          {weapon.affinity ?? 0}%
        </p>
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-400">Element</p>
        <p class="text-lg font-bold mt-1 {elementColor(weapon.element_type)}">
          {weapon.element_type ? `${weapon.element_type} ${weapon.element_value ?? 0}` : '—'}
        </p>
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-400">Status</p>
        <p
          class="text-lg font-bold mt-1 {weapon.status_type ? 'text-fuchsia-300' : 'text-gray-400'}"
        >
          {weapon.status_type ? `${weapon.status_type} ${weapon.status_value ?? 0}` : '—'}
        </p>
      </div>
    </div>

    <div class="stat-grid gap-3 mb-8">
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-400">Cost</p>
        <p class="text-lg font-bold mt-1" style="color: var(--theme-accent);">
          {weapon.crafting_cost ?? 0}z
        </p>
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-400">Slots</p>
        <p class="text-lg font-bold text-gray-100 mt-1">{weapon.slots ?? '0'}</p>
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-400">Defense Bonus</p>
        <p class="text-lg font-bold text-gray-100 mt-1">{weapon.defense_bonus ?? 0}</p>
      </div>
      {#if parentName}
        <div
          class="rounded-lg border themed-card p-3 text-center col-span-2 sm:col-span-1 flex flex-col justify-center overflow-hidden"
        >
          <p class="text-[10px] uppercase tracking-wide text-gray-400">Upgraded From</p>
          <p class="text-xs font-semibold text-gray-200 mt-1 truncate px-2">
            {parentName}
          </p>
        </div>
      {/if}
    </div>

    <section class="mb-8">
      <h2 class="section-title mb-3">Upgrade Tree</h2>
      <div class="rounded-lg border themed-card p-4">
        <div class="flex flex-wrap items-center gap-2 mb-4">
          {#if weapon.is_forgeable}
            <span
              class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-md text-xs font-medium border min-h-[36px]"
              style="background-color: color-mix(in oklab, #22c55e 16%, var(--theme-bg-elevated)); border-color: color-mix(in oklab, #22c55e 45%, transparent); color: #4ade80;"
            >
              <Hammer class="h-3.5 w-3.5" aria-hidden="true" /> Forgeable directly
            </span>
          {/if}
          {#if isUpgrade}
            <button
              onclick={() => openWeapon(baseWeapon?.id)}
              class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-md text-xs font-medium border cursor-pointer transition-colors motion-safe:transition-colors motion-reduce:transition-none hover:border-[var(--theme-border-strong)] min-h-[36px] focus-visible:outline-none focus-visible:ring-2"
              style="background-color: color-mix(in oklab, var(--theme-accent) 16%, var(--theme-bg-elevated)); border-color: color-mix(in oklab, var(--theme-accent) 45%, transparent); color: var(--theme-accent);"
            >
              <ArrowUp class="h-3.5 w-3.5" aria-hidden="true" /> Crafted from {baseWeapon?.name ??
                parentName ??
                ''}
            </button>
          {/if}
        </div>
        {#each treeRows as row (row.weapon.id)}
          {@render detailRow(row)}
        {/each}
      </div>
    </section>

    {#if sharpTotal > 0}
      <section class="mb-8">
        <h2 class="section-title mb-3">Sharpness</h2>
        <div class="rounded-lg border themed-card p-4">
          <div class="overflow-x-auto -mx-1 px-1" role="img" aria-label="Sharpness: {sharpA11y}">
            <div class="flex items-center gap-[2px] h-4 w-max max-w-none" aria-hidden="true">
              {#each sharpnessSegments(weapon.sharpness) as seg, i}
                {#if seg > 0}
                  <div
                    class="rounded-[2px] shrink-0"
                    style="height: 14px; width: {seg * 3}px; background: {SHARP_COLORS[
                      i
                    ]}; box-shadow: inset 0 -3px 0 rgba(0,0,0,0.25);"
                  ></div>
                {/if}
              {/each}
            </div>
          </div>
          <div class="flex flex-wrap gap-x-4 gap-y-1 mt-3">
            {#each sharpnessSegments(weapon.sharpness) as seg, i}
              {#if seg > 0}
                <div class="flex items-center gap-1.5">
                  <span
                    class="inline-block w-2.5 h-2.5 rounded-sm"
                    style="background: {SHARP_COLORS[i]};"
                  ></span>
                  <span class="text-xs text-gray-300">{SHARP_LABELS[i]}</span>
                  <span class="text-xs text-gray-400 tabular-nums">{seg}</span>
                </div>
              {/if}
            {/each}
          </div>
        </div>
      </section>
    {/if}

    {#if weapon.description}
      <section class="mb-8">
        <h2 class="section-title mb-3">Description</h2>
        <div class="rounded-lg border themed-card p-5 leading-relaxed text-gray-200 text-[15px]">
          {weapon.description}
        </div>
      </section>
    {/if}

    {#if weapon.forge_materials.length > 0}
      <section class="mb-8">
        <h2 class="section-title mb-3">Forge (Direct) Materials</h2>
        <div class="rounded-lg border themed-card p-4">
          <MaterialList materials={weapon.forge_materials} showCraftingCost={false} />
        </div>
      </section>
    {/if}

    {#if weapon.upgrade_materials.length > 0}
      <section class="mb-8">
        <h2 class="section-title mb-3">
          Upgrade Materials {isUpgrade
            ? '(from ' + (baseWeapon?.name ?? parentName ?? '') + ')'
            : ''}
        </h2>
        <div class="rounded-lg border themed-card p-4">
          <MaterialList materials={weapon.upgrade_materials} showCraftingCost={false} />
        </div>
      </section>
    {/if}

    {#if weapon.forge_materials.length === 0 && weapon.upgrade_materials.length === 0}
      <section>
        <h2 class="section-title mb-3">Crafting Materials</h2>
        <MaterialList materials={weapon.materials} showCraftingCost={false} />
      </section>
    {/if}
  {/if}
</div>

{#snippet detailRow(row: DetailRow)}
  {@const w = row.weapon}
  <div
    class="min-w-0 {row.depth > 0 ? 'ml-3 sm:ml-4 pl-2 border-l border-[var(--theme-border)]' : ''}"
    style={row.depth > 1
      ? `margin-left: calc(0.75rem + ${(Math.min(row.depth, MAX_INDENT) - 1) * 0.5}rem);`
      : ''}
  >
    <div class="flex items-center gap-1.5 mb-1.5 min-w-0">
      {#if row.depth > 0}
        <CornerDownRight class="h-3.5 w-3.5 shrink-0 text-gray-600" aria-hidden="true" />
      {/if}
      <button
        onclick={() => openWeapon(w.id)}
        aria-current={w.id === weapon?.id ? 'page' : undefined}
        class="flex-1 min-w-0 text-left px-3 min-h-[48px] py-2 rounded-lg border transition-colors motion-safe:transition-colors motion-reduce:transition-none cursor-pointer focus-visible:outline-none focus-visible:ring-2"
        style={w.id === weapon?.id
          ? 'background-color: color-mix(in oklab, var(--theme-accent) 20%, var(--theme-bg-elevated)); border-color: color-mix(in oklab, var(--theme-accent) 60%, transparent); color: var(--theme-accent); font-weight: 600;'
          : 'border-color: var(--theme-border); background-color: var(--theme-bg-surface); color: rgb(209 213 219);'}
      >
        <div class="flex items-center gap-2 min-w-0">
          <ItemIcon
            iconUrl={w.icon_url}
            iconName={w.icon_name}
            iconColor={w.icon_color}
            size={20}
            alt=""
          />
          <span
            class="text-[10px] shrink-0 w-9 text-center rounded py-0.5 border border-[var(--theme-border)] tabular-nums"
            style="color: rgb(156 163 175);">R{w.rarity ?? 1}</span
          >
          {#if w.is_forgeable}
            <Hammer class="h-3 w-3 shrink-0 text-gray-400" aria-label="Forgeable directly" />
          {/if}
          <span class="text-sm font-medium truncate min-w-0">{w.attack ?? 0} · {w.name}</span>
          {#if w.element_type}
            <span class="text-[11px] {elementColor(w.element_type)} shrink-0 hidden sm:inline"
              >{w.element_type} {w.element_value ?? 0}</span
            >
          {/if}
        </div>
      </button>
    </div>
  </div>
{/snippet}
