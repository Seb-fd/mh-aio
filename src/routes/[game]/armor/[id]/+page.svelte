<script lang="ts">
  import { page } from '$app/state'
  import ErrorState from '$lib/components/ui/error-state.svelte'
  import { api, type ArmorDetail } from '$lib/api'
  import DetailHeader from '$lib/components/detail-header.svelte'
  import MaterialList from '$lib/components/material-list.svelte'
  import Skeleton from '$lib/components/ui/skeleton.svelte'
  import EmptyState from '$lib/components/ui/empty-state.svelte'
  import { rankTone } from '$lib/utils/mh'

  const id = $derived(Number(page.params.id))
  let armor = $state<ArmorDetail | null>(null)
  let loading = $state(true)
  let error = $state<string | null>(null)

  $effect(() => {
    if (!id || Number.isNaN(id)) return
    loading = true
    error = null
    api
      .getArmorDetail(id)
      .then((data) => {
        armor = data
      })
      .catch((e) => {
        error = String(e)
      })
      .finally(() => {
        loading = false
      })
  })

  const slotLabel: Record<string, string> = {
    head: 'Helm',
    chest: 'Mail',
    arms: 'Vambraces',
    waist: 'Coil',
    legs: 'Greaves',
  }
</script>

<div class="max-w-5xl mx-auto numbered-sections">
  {#if loading}
    <div class="space-y-3" aria-busy="true">
      <Skeleton lines={2} />
      <Skeleton lines={3} />
    </div>
  {:else if error}
    <ErrorState title="Failed to load armor" {error} />
  {:else if !armor}
    <EmptyState title="Armor not found" hint="It may belong to another game." />
  {:else}
    <DetailHeader
      title={armor.name}
      subtitle={slotLabel[armor.slot_type] ?? armor.slot_type}
      iconUrl={armor.icon_url}
      tags={[
        { label: armor.rank, tone: rankTone(armor.rank) },
        {
          label: `Rarity ${armor.rarity ?? 1}`,
          color: 'bg-[var(--theme-bg-elevated)] text-gray-300 border-[var(--theme-border)]',
        },
        ...(armor.armor_type
          ? [
              {
                label:
                  armor.armor_type === 'gunner'
                    ? 'Gunner'
                    : armor.armor_type === 'blade'
                      ? 'Blademaster'
                      : 'Any',
                color:
                  'bg-[var(--theme-bg-elevated)] text-[var(--theme-text-accent)] border-[var(--theme-border-strong)]',
              },
            ]
          : []),
      ]}
    />

    <div class="grid grid-cols-2 sm:grid-cols-3 gap-3 mb-8">
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-400">Defense</p>
        <p class="text-xl font-bold text-gray-100 mt-1">
          {armor.defense_base ?? 0}-{armor.defense_max ?? 0}
        </p>
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-400">Crafting Cost</p>
        <p class="text-xl font-bold mt-1" style="color: var(--theme-accent);">
          {armor.crafting_cost ?? 0}z
        </p>
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-400">Slots</p>
        <p class="text-xl font-bold text-gray-100 mt-1">{armor.slots ?? '0'}</p>
      </div>
    </div>

    {#if armor.skills}
      <section class="mb-8">
        <h2 class="section-title mb-3">Skills</h2>
        <div class="flex flex-wrap gap-2">
          {#each armor.skills
            .split(',')
            .map((s) => s.trim())
            .filter(Boolean) as skill}
            {@const parts = skill.trim().split(/\s+(?=[+-]\d)/)}
            <div
              class="px-3 min-h-[44px] py-1.5 rounded-md bg-[var(--theme-bg-elevated)] border border-[var(--theme-border)] flex items-center gap-2"
            >
              <span class="text-sm text-gray-200">{parts[0]}</span>
              {#if parts[1]}
                <span
                  class="text-xs font-semibold {Number(parts[1]) >= 0
                    ? 'text-emerald-300'
                    : 'text-red-300'}">{parts[1]}</span
                >
              {/if}
            </div>
          {/each}
        </div>
      </section>
    {/if}

    <section class="mb-8">
      <h2 class="section-title mb-3">Elemental Resistances</h2>
      <div class="rounded-lg border themed-card p-4">
        <div class="stat-grid text-center text-xs">
          <div
            class="rounded-md bg-[var(--theme-bg-elevated)] border border-[var(--theme-border)] py-2"
          >
            <p class="text-orange-300 font-semibold mb-1">Fire</p>
            <p class="text-lg font-bold text-gray-100 tabular-nums">{armor.resistance_fire ?? 0}</p>
          </div>
          <div
            class="rounded-md bg-[var(--theme-bg-elevated)] border border-[var(--theme-border)] py-2"
          >
            <p class="text-blue-300 font-semibold mb-1">Water</p>
            <p class="text-lg font-bold text-gray-100 tabular-nums">
              {armor.resistance_water ?? 0}
            </p>
          </div>
          <div
            class="rounded-md bg-[var(--theme-bg-elevated)] border border-[var(--theme-border)] py-2"
          >
            <p class="text-yellow-300 font-semibold mb-1">Thunder</p>
            <p class="text-lg font-bold text-gray-100 tabular-nums">
              {armor.resistance_thunder ?? 0}
            </p>
          </div>
          <div
            class="rounded-md bg-[var(--theme-bg-elevated)] border border-[var(--theme-border)] py-2"
          >
            <p class="text-cyan-300 font-semibold mb-1">Ice</p>
            <p class="text-lg font-bold text-gray-100 tabular-nums">{armor.resistance_ice ?? 0}</p>
          </div>
          <div
            class="rounded-md bg-[var(--theme-bg-elevated)] border border-[var(--theme-border)] py-2 col-span-2 sm:col-span-1"
          >
            <p class="text-purple-300 font-semibold mb-1">Dragon</p>
            <p class="text-lg font-bold text-gray-100 tabular-nums">
              {armor.resistance_dragon ?? 0}
            </p>
          </div>
        </div>
      </div>
    </section>

    {#if armor.description}
      <section class="mb-8">
        <h2 class="section-title mb-3">Description</h2>
        <div class="rounded-lg border themed-card p-5 leading-relaxed text-gray-200 text-[15px]">
          {armor.description}
        </div>
      </section>
    {/if}

    <section>
      <h2 class="section-title mb-3">Crafting Materials</h2>
      <MaterialList materials={armor.materials} />
    </section>
  {/if}
</div>
