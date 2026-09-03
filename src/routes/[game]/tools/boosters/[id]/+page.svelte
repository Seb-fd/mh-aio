<script lang="ts">
  import { page } from '$app/state'
  import { api, type MhwMantle } from '$lib/api'
  import DetailHeader from '$lib/components/detail-header.svelte'
  import ItemIcon from '$lib/components/item-icon.svelte'
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

<div class="max-w-5xl mx-auto">
  {#if loading}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Loading...</p>
    </div>
  {:else if error}
    <div class="bg-red-950/30 border border-red-900 rounded-lg p-8 text-center">
      <p class="text-red-400">{error}</p>
    </div>
  {:else if !mantle}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Not found</p>
    </div>
  {:else}
    <DetailHeader
      title={mantle.name}
      subtitle={mantle.tool_type === 'booster' ? 'Booster' : 'Mantle'}
      icon="🦺"
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
        <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-2">
          Description
        </h2>
        <div class="rounded-lg border themed-card p-4 text-sm text-gray-200">
          {mantle.description}
        </div>
      </section>
    {/if}
    <section class="mb-6">
      <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-2">
        Specific Effect
      </h2>
      <div class="rounded-lg border themed-card p-4 text-sm text-gray-200 leading-relaxed">
        {mantle.effect}
      </div>
      <div class="grid grid-cols-3 gap-2 mt-3 text-xs">
        <div
          class="rounded border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] p-3 text-center"
        >
          <p class="text-gray-500">Duration</p>
          <p class="font-semibold text-gray-100">{mantle.duration_sec ?? '-'} s</p>
        </div>
        <div
          class="rounded border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] p-3 text-center"
        >
          <p class="text-gray-500">Base Cooldown</p>
          <p class="font-semibold text-gray-100">{mantle.cooldown_sec ?? '-'} s</p>
        </div>
        <div
          class="rounded border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] p-3 text-center"
        >
          <p class="text-gray-500">Upgraded Cooldown</p>
          <p class="font-semibold text-emerald-400">{mantle.cooldown_upgraded_sec ?? '-'} s</p>
        </div>
      </div>
      {#if mantle.slots}<p class="text-xs text-gray-500 mt-2">
          Slots: <span class="text-gray-300">{mantle.slots}</span>
        </p>{/if}
    </section>
    <!-- Base vs Plus comparison -->
    <section class="mb-6">
      <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-2">
        Appearance — Base vs Upgraded (+)
      </h2>
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
        <div
          class="rounded-lg border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] p-4 flex items-center gap-4"
        >
          <ItemIcon
            iconUrl={mantle.icon_url}
            iconName={mantle.icon_name}
            iconColor={mantle.icon_color}
            size={56}
            alt={mantle.name}
          />
          <div>
            <p class="text-sm font-semibold text-gray-100">{mantle.name}</p>
            <p class="text-[11px] text-gray-500">Base — without star</p>
            <p class="text-[11px] text-gray-400 mt-1">
              ⏱ {mantle.duration_sec ?? '-'}s · ↻ {mantle.cooldown_sec ?? '-'}s
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
            <p class="text-[11px] text-gray-400 mt-1">
              ↻ {mantle.cooldown_upgraded_sec ?? '-'}s · {mantle.upgrade_effect ??
                'Improved slots/cooldown'}
            </p>
          </div>
        </div>
      </div>
    </section>
    <section class="mb-6">
      <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-2">Acquisition</h2>
      <div class="rounded-lg border themed-card p-4 text-sm text-gray-200 leading-relaxed">
        {mantle.acquisition ?? 'Talk to the Armory after the corresponding quest.'}
      </div>
    </section>
    {#if mantle.upgrade_quest}
      <section class="mb-6">
        <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-2">
          Iceborne Upgrade (+)
        </h2>
        <div class="rounded-lg border border-amber-900/50 bg-[var(--theme-bg-surface)] p-4">
          <p class="text-sm text-amber-300 font-medium">{mantle.upgrade_quest}</p>
          {#if mantle.upgrade_effect}<p class="text-sm text-gray-300 mt-2">
              {mantle.upgrade_effect}
            </p>{/if}
          <p class="text-[11px] text-gray-500 mt-2">
            Complete this optional Master Rank quest in Seliana to receive the upgraded (+) version
            with improved slots and reduced cooldown from the Armory.
          </p>
        </div>
      </section>
    {/if}
  {/if}
</div>
