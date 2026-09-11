<script lang="ts">
  import { page } from '$app/state'
  import ErrorState from '$lib/components/ui/error-state.svelte'
  import { api, type MhwMantle } from '$lib/api'
  import DetailHeader from '$lib/components/detail-header.svelte'
  import ItemIcon from '$lib/components/item-icon.svelte'
  import Skeleton from '$lib/components/ui/skeleton.svelte'
  import EmptyState from '$lib/components/ui/empty-state.svelte'
  import { Timer, RefreshCw } from '@lucide/svelte'
  const id = $derived(Number(page.params.id))
  let mantle = $state<MhwMantle | null>(null)
  let loading = $state(true)
  let error = $state<string | null>(null)
  $effect(() => {
    if (!id || Number.isNaN(id)) return
    loading = true
    api
      .getMhwMantleDetail(id)
      .then((d) => (mantle = d))
      .catch((e) => (error = String(e)))
      .finally(() => (loading = false))
  })
</script>

<div class="max-w-5xl mx-auto numbered-sections">
  {#if loading}
    <div class="space-y-3" aria-busy="true">
      <Skeleton lines={2} />
      <Skeleton lines={3} />
    </div>
  {:else if error}
    <ErrorState title="Something went wrong" {error} />
  {:else if !mantle}
    <EmptyState title="Not found" hint="It may belong to another game." />
  {:else}
    <DetailHeader
      title={mantle.name}
      subtitle={mantle.tool_type === 'booster' ? 'Booster' : 'Mantle'}
      iconUrl={mantle.icon_url}
      tags={[
        {
          label: mantle.tool_type,
          color: 'bg-[var(--theme-bg-elevated)] text-gray-300 border-[var(--theme-border)]',
        },
      ]}
    />
    {#if mantle.description}
      <section class="mb-6">
        <h2 class="section-title mb-2">Description</h2>
        <div class="rounded-lg border themed-card p-4 text-sm text-gray-200">
          {mantle.description}
        </div>
      </section>
    {/if}
    <section class="mb-6">
      <h2 class="section-title mb-2">Specific Effect</h2>
      <div class="rounded-lg border themed-card p-4 text-sm text-gray-200 leading-relaxed">
        {mantle.effect}
      </div>
      <div class="stat-grid gap-2 mt-3 text-xs">
        <div
          class="rounded border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] p-3 text-center"
        >
          <p class="text-gray-400">Duration</p>
          <p class="font-semibold text-gray-100 tabular-nums">{mantle.duration_sec ?? '-'} s</p>
        </div>
        <div
          class="rounded border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] p-3 text-center"
        >
          <p class="text-gray-400">Base Cooldown</p>
          <p class="font-semibold text-gray-100 tabular-nums">{mantle.cooldown_sec ?? '-'} s</p>
        </div>
        <div
          class="rounded border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] p-3 text-center col-span-2 sm:col-span-1"
        >
          <p class="text-gray-400">Upgraded Cooldown</p>
          <p class="font-semibold text-emerald-300 tabular-nums">
            {mantle.cooldown_upgraded_sec ?? '-'} s
          </p>
        </div>
      </div>
      {#if mantle.slots}<p class="text-xs text-gray-400 mt-2">
          Slots: <span class="text-gray-300">{mantle.slots}</span>
        </p>{/if}
    </section>
    <!-- Base vs Plus comparison -->
    <section class="mb-6">
      <h2 class="section-title mb-2">Appearance — Base vs Upgraded (+)</h2>
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
        <div
          class="rounded-lg border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] p-4 flex items-center gap-4"
        >
          <ItemIcon
            iconUrl={mantle.icon_url}
            iconName={mantle.icon_name}
            iconColor={mantle.icon_color}
            size={56}
            alt=""
          />
          <div>
            <p class="text-sm font-semibold text-gray-100">{mantle.name}</p>
            <p class="text-[11px] text-gray-400">Base — without star</p>
            <p
              class="text-[11px] text-gray-400 mt-1 inline-flex flex-wrap items-center gap-x-2 gap-y-0.5"
            >
              <span class="inline-flex items-center gap-1"
                ><Timer class="h-3 w-3 shrink-0" aria-hidden="true" />{mantle.duration_sec ??
                  '-'}s</span
              >
              <span class="inline-flex items-center gap-1"
                ><RefreshCw class="h-3 w-3 shrink-0" aria-hidden="true" />{mantle.cooldown_sec ??
                  '-'}s</span
              >
            </p>
          </div>
        </div>
        <div
          class="rounded-lg border border-amber-900/40 bg-[var(--theme-bg-surface)] p-4 flex items-center gap-4"
        >
          {#if mantle.icon_url_plus}
            <ItemIcon
              iconUrl={mantle.icon_url_plus}
              iconName={mantle.icon_name_plus}
              iconColor={mantle.icon_color_plus}
              size={56}
              alt={(mantle.name ?? '') + ' +'}
            />
          {/if}
          <div>
            <p class="text-sm font-semibold text-amber-300">{mantle.name} +</p>
            <p class="text-[11px] text-amber-400/70">Upgraded — with star (Master Rank)</p>
            <p class="text-[11px] text-gray-400 mt-1 inline-flex flex-wrap items-center gap-1">
              <RefreshCw
                class="h-3 w-3 shrink-0"
                aria-hidden="true"
              />{mantle.cooldown_upgraded_sec ?? '-'}s · {mantle.upgrade_effect ??
                'Improved slots/cooldown'}
            </p>
          </div>
        </div>
      </div>
    </section>
    <section class="mb-6">
      <h2 class="section-title mb-2">Acquisition</h2>
      <div class="rounded-lg border themed-card p-4 text-sm text-gray-200 leading-relaxed">
        {mantle.acquisition ?? 'Talk to the Armory after the corresponding quest.'}
      </div>
    </section>
    {#if mantle.upgrade_quest}
      <section class="mb-6">
        <h2 class="section-title mb-2">Iceborne Upgrade (+)</h2>
        <div class="rounded-lg border border-amber-900/50 bg-[var(--theme-bg-surface)] p-4">
          <p class="text-sm text-amber-300 font-medium">{mantle.upgrade_quest}</p>
          {#if mantle.upgrade_effect}<p class="text-sm text-gray-300 mt-2">
              {mantle.upgrade_effect}
            </p>{/if}
          <p class="text-[11px] text-gray-400 mt-2">
            Complete this optional Master Rank quest in Seliana to receive the upgraded (+) version
            with improved slots and reduced cooldown from the Armory.
          </p>
        </div>
      </section>
    {/if}
  {/if}
</div>
