<script lang="ts">
  import { page } from '$app/state'
  import { api, type PalicoGadgetDetail } from '$lib/api'
  import DetailHeader from '$lib/components/detail-header.svelte'
  import ItemIcon from '$lib/components/item-icon.svelte'
  const id = $derived(Number(page.params.id))
  let gadget = $state<PalicoGadgetDetail | null>(null)
  let loading = $state(true)
  let error = $state<string | null>(null)
  $effect(() => {
    if (!id || Number.isNaN(id)) return
    loading = true
    api
      .getPalicoGadgetDetail(id)
      .then((d) => (gadget = d))
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
  {:else if !gadget}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Not found</p>
    </div>
  {:else}
    <DetailHeader
      title={gadget.name}
      subtitle={gadget.tribe ?? ''}
      icon="🐾"
      iconUrl={gadget.icon_url}
      tags={[
        {
          label: gadget.gadget_type,
          color: 'bg-[var(--theme-bg-elevated)] text-gray-300 border-[var(--theme-border)]',
        },
      ]}
    />
    <section class="mb-6">
      <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-2">Icon</h2>
      <div class="flex items-center gap-4 rounded-lg border themed-card p-4">
        <ItemIcon
          iconUrl={gadget.icon_url}
          iconName={gadget.icon_name}
          iconColor={gadget.icon_color}
          size={48}
          alt={gadget.name}
        />
        <div>
          <p class="text-sm font-semibold text-gray-100">{gadget.name}</p>
          <p class="text-[11px] text-gray-500">{gadget.gadget_type} — {gadget.tribe ?? ''}</p>
        </div>
      </div>
    </section>
    {#if gadget.description}
      <section class="mb-6">
        <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-2">
          Description
        </h2>
        <div class="rounded-lg border themed-card p-4 text-sm text-gray-200">
          {gadget.description}
        </div>
      </section>
    {/if}
    {#if gadget.effect}
      <section class="mb-6">
        <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-2">
          Specific Effect
        </h2>
        <div class="rounded-lg border themed-card p-4 text-sm text-gray-200 leading-relaxed">
          {gadget.effect}
        </div>
      </section>
    {/if}
    <section class="mb-6">
      <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-2">
        Acquisition / Unlock
      </h2>
      <div
        class="rounded-lg border border-amber-900/40 bg-[var(--theme-bg-surface)] p-4 text-sm text-amber-200/90 leading-relaxed"
      >
        {gadget.acquisition ?? 'Complete the corresponding Grimalkyne tribe quest chain.'}
      </div>
      {#if gadget.gadget_type === 'gadget'}
        <p class="text-[11px] text-gray-500 mt-2">
          All 6 gadgets are unlocked by befriending each Lynian tribe (Bugtrappers, Protectors,
          Troupers, Plunderers, Gajalaka). Talk to the Lynian Researcher after gaining their trust.
        </p>
      {:else if gadget.gadget_type === 'tailraider'}
        <p class="text-[11px] text-gray-500 mt-2">
          Tailraider Signal is Iceborne-exclusive (Boaboa in Hoarfrost Reach). Requires Master Rank
          and completing "By Our Powers Combined".
        </p>
      {:else}
        <p class="text-[11px] text-gray-500 mt-2">
          Safari is managed from Astera/Seliana via the Housekeeper. Each tribe befriended unlocks
          an additional simultaneous expedition slot.
        </p>
      {/if}
    </section>
    {#if gadget.levels.length > 0}
      <section class="mb-6">
        <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-2">
          Proficiency / Upgrades ({gadget.levels.length} levels)
        </h2>
        <p class="text-[11px] text-gray-500 mb-3">
          Level up by using the gadget on hunts. Each level unlocks a more powerful ability. Level
          10 = mastery.
        </p>
        <div class="space-y-2">
          {#each gadget.levels as lv (lv.id)}
            <div
              class="rounded-lg border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] p-3 flex gap-3"
            >
              <span
                class="shrink-0 w-10 h-10 rounded-full bg-[var(--theme-bg-elevated)] border border-[var(--theme-border)] flex items-center justify-center text-xs font-bold text-[var(--theme-accent)]"
                >{lv.proficiency}</span
              >
              <div class="min-w-0 flex-1">
                <p class="text-sm font-medium text-gray-100">{lv.ability_name}</p>
                {#if lv.description}<p class="text-xs text-gray-400 mt-0.5">
                    {lv.description}
                  </p>{/if}
                {#if lv.unlock_condition}<p class="text-[11px] text-emerald-400/80 mt-1">
                    🔓 {lv.unlock_condition}
                  </p>{/if}
              </div>
            </div>
          {/each}
        </div>
      </section>
    {/if}
  {/if}
</div>
