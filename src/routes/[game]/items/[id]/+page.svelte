<script lang="ts">
  import { page } from '$app/state'
  import { goto } from '$app/navigation'
  import { api, type ItemDetail } from '$lib/api'
  import DetailHeader from '$lib/components/detail-header.svelte'
  import DropTable from '$lib/components/drop-table.svelte'
  import ItemIcon from '$lib/components/item-icon.svelte'
  import { selectedGame } from '$lib/stores/game'

  const id = $derived(Number(page.params.id))
  const game = $derived($selectedGame)
  let item = $state<ItemDetail | null>(null)
  let loading = $state(true)
  let error = $state<string | null>(null)

  $effect(() => {
    if (!id || Number.isNaN(id)) return
    loading = true
    error = null
    api
      .getItemDetail(id)
      .then((data) => {
        item = data
      })
      .catch((e) => {
        error = String(e)
      })
      .finally(() => {
        loading = false
      })
  })

  function goToItem(itemId: number) {
    if (!game) return
    goto(`/${game.id}/items/${itemId}`)
  }

  const categoryColor: Record<string, string> = {
    Consumable: 'bg-emerald-900/40 text-emerald-300',
    Material: 'bg-purple-900/40 text-purple-300',
    Ammo: 'bg-orange-900/40 text-orange-300',
  }

  // Detect whether a description is (partly) Japanese. MHP3rd material
  // descriptions sourced from the Monster Item List are in Japanese; these are
  // kept faithfully but flagged so the English UI doesn't look broken.
  const hasCJK = $derived(
    item?.description ? /[\u3040-\u30ff\u3400-\u4dbf\u4e00-\u9fff]/u.test(item.description) : false,
  )
</script>

<div class="max-w-5xl mx-auto">
  {#if loading}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Loading item...</p>
    </div>
  {:else if error}
    <div class="bg-red-950/30 border border-red-900 rounded-lg p-8 text-center">
      <p class="text-red-400">Failed to load item</p>
      <p class="text-gray-500 text-sm mt-2">{error}</p>
    </div>
  {:else if !item}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Item not found</p>
    </div>
  {:else}
    <DetailHeader
      title={item.name}
      subtitle={item.subcategory
        ? `${item.category ?? ''} • ${item.subcategory}`
        : (item.category ?? '')}
      icon={item.category === 'Ammo'
        ? '🏹'
        : item.category === 'Consumable'
          ? '🧪'
          : item.subcategory === 'Charm'
            ? '✨'
            : '📦'}
      iconUrl={item.icon_url}
      tags={[
        {
          label: item.category ?? 'Unknown',
          color: categoryColor[item.category ?? ''] ?? 'bg-gray-800 text-gray-300',
        },
        ...(item.subcategory && item.subcategory !== item.category
          ? [
              {
                label: item.subcategory,
                color: 'bg-[var(--theme-bg-elevated)] text-gray-300 border-[var(--theme-border)]',
              },
            ]
          : []),
        {
          label: `Rarity ${item.rarity ?? 1}`,
          color: 'bg-[var(--theme-bg-elevated)] text-gray-300 border-[var(--theme-border)]',
        },
      ]}
    />

    <div class="mb-6 flex flex-wrap gap-2">
      {#if item.sell_price !== null && item.sell_price !== undefined}
        <div class="inline-flex items-center gap-2 px-4 py-2 rounded-lg border themed-card">
          <span class="text-xs uppercase tracking-wide text-gray-500">Sell</span>
          <span class="text-sm font-semibold" style="color: var(--theme-accent);"
            >{item.sell_price}z</span
          >
        </div>
      {/if}
      {#if item.buy_price !== null && item.buy_price !== undefined}
        <div class="inline-flex items-center gap-2 px-4 py-2 rounded-lg border themed-card">
          <span class="text-xs uppercase tracking-wide text-gray-500">Buy</span>
          <span class="text-sm font-semibold" style="color: var(--theme-accent);"
            >{item.buy_price}z</span
          >
        </div>
      {/if}
      {#if item.carry_limit !== null && item.carry_limit !== undefined}
        <div class="inline-flex items-center gap-2 px-4 py-2 rounded-lg border themed-card">
          <span class="text-xs uppercase tracking-wide text-gray-500">Carry</span>
          <span class="text-sm font-semibold text-gray-200">x{item.carry_limit}</span>
        </div>
      {/if}
      {#if item.icon_url || item.icon_name}
        <div class="inline-flex items-center gap-2 px-3 py-1.5 rounded-lg border themed-card">
          <ItemIcon iconUrl={item.icon_url} iconName={item.icon_name} iconColor={item.icon_color} size={24} alt={item.name} />
          <span class="text-xs text-gray-500">{item.icon_name ?? 'Icon'}</span>
          {#if item.icon_color}
            <span class="text-xs text-gray-600">· {item.icon_color}</span>
          {/if}
        </div>
      {/if}
    </div>

    {#if item.description}
      <section class="mb-8">
        <div class="flex items-center gap-2 mb-3">
          <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold">Description</h2>
          {#if hasCJK}
            <span
              class="text-[10px] px-2 py-0.5 rounded-full border border-amber-800 bg-amber-900/30 text-amber-300 font-semibold"
              title="Source text is in Japanese (Monster Item List)">🇯🇵 JP</span
            >
          {/if}
        </div>
        <div class="rounded-lg border themed-card p-5 leading-relaxed text-gray-200 text-[15px]">
          {item.description}
        </div>
      </section>
    {/if}

    {#if item.recipes.length > 0}
      <section class="mb-8">
        <div class="flex items-center gap-2 mb-3">
          <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold">
            Combination Recipe
          </h2>
          {#if item.recipes[0]}
            <span
              class="text-[10px] px-2 py-0.5 rounded-full border font-semibold
              {item.recipes[0].combine_type === 'alchemy'
                ? 'bg-amber-900/30 text-amber-300 border-amber-800'
                : item.recipes[0].combine_type === 'treasure'
                  ? 'bg-purple-900/30 text-purple-300 border-purple-800'
                  : 'bg-sky-900/30 text-sky-300 border-sky-800'}"
            >
              {item.recipes[0].combine_type === 'alchemy'
                ? '⚗️ Alchemy'
                : item.recipes[0].combine_type === 'treasure'
                  ? '💎 Treasure'
                  : '🧪 Normal'}
            </span>
            {#if item.recipes[0].chance != null}
              <span
                class="text-[10px] px-1.5 py-0.5 rounded border bg-[var(--theme-bg-elevated)] text-gray-400 border-[var(--theme-border)]"
                >{item.recipes[0].chance}% success</span
              >
            {/if}
          {/if}
        </div>
        <div class="rounded-lg border themed-card p-4">
          <div class="flex flex-wrap items-center gap-2">
            {#each item.recipes as recipe, i}
              {#if i > 0}
                <span class="text-gray-600 text-lg">+</span>
              {/if}
              <button
                onclick={() => goToItem(recipe.component_item_id)}
                class="px-3 py-1.5 rounded-md bg-[var(--theme-bg-elevated)] border border-[var(--theme-border)] flex items-center gap-2 hover:border-[var(--theme-border-strong)] hover:bg-[var(--theme-bg-surface)] transition-colors cursor-pointer text-left"
                title="Go to {recipe.component_name}"
              >
                <span
                  class="text-sm text-gray-200 hover:text-[var(--theme-accent)] transition-colors"
                  >{recipe.component_name}</span
                >
                <span class="text-xs font-semibold" style="color: var(--theme-accent);"
                  >x{recipe.quantity}</span
                >
              </button>
            {/each}
            <span class="text-gray-600 text-lg">=</span>
            <div
              class="px-3 py-1.5 rounded-md flex items-center gap-2"
              style="background-color: color-mix(in oklab, var(--theme-accent) 15%, var(--theme-bg-elevated)); border: 1px solid color-mix(in oklab, var(--theme-accent) 40%, transparent);"
            >
              <span class="text-sm text-gray-100">{item.name}</span>
              <span class="text-xs font-semibold" style="color: var(--theme-accent);"
                >x{item.recipes[0]?.result_quantity ?? 1}</span
              >
            </div>
          </div>
          {#if item.recipes[0]?.combine_type === 'alchemy'}
            <p class="text-[11px] text-amber-400/70 mt-2">
              ※ Alchemy — requires Alchemy Guide and Books 1-5 (progressively unlocks alchemy
              recipes)
            </p>
          {/if}
        </div>
      </section>
    {/if}

    {#if item.melder}
      <section class="mb-8">
        <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-3">Elder Melder</h2>
        <div class="rounded-lg border themed-card p-4 flex flex-wrap items-center gap-3">
          <div class="px-3 py-1.5 rounded-md bg-[var(--theme-bg-elevated)] border border-[var(--theme-border)]">
            <span class="text-xs text-gray-500">Research</span>
            <span class="ml-2 text-sm font-semibold" style="color: var(--theme-accent);">{item.melder.research_cost} RP</span>
          </div>
          <div class="px-3 py-1.5 rounded-md bg-[var(--theme-bg-elevated)] border border-[var(--theme-border)]">
            <span class="text-xs text-gray-500">Melding</span>
            <span class="ml-2 text-sm font-semibold" style="color: var(--theme-accent);">{item.melder.melding_cost} MP</span>
          </div>
          <span
            class="text-[10px] px-2 py-1 rounded-full border font-semibold
            {item.melder.melder_type === 'celestial' ? 'bg-purple-900/30 text-purple-300 border-purple-800' : item.melder.melder_type === 'gold' ? 'bg-yellow-900/30 text-yellow-300 border-yellow-800' : item.melder.melder_type === 'silver' ? 'bg-gray-800 text-gray-300 border-gray-700' : item.melder.melder_type === 'steel' ? 'bg-slate-800 text-slate-300 border-slate-700' : item.melder.melder_type === 'guiding' ? 'bg-emerald-900/30 text-emerald-300 border-emerald-800' : 'bg-sky-900/30 text-sky-300 border-sky-800'}"
          >
            {item.melder.melder_type === 'celestial' ? '✨ Celestial' : item.melder.melder_type === 'gold' ? '🥇 Gold' : item.melder.melder_type === 'silver' ? '🥈 Silver' : item.melder.melder_type === 'steel' ? '🔩 Steel' : item.melder.melder_type === 'guiding' ? '🗺️ Guiding' : '⚗️ Normal'}
            {item.melder.melder_type}
          </span>
          {#if item.melder.unlock_condition}
            <span class="text-xs text-gray-500">Unlock: {item.melder.unlock_condition}</span>
          {/if}
        </div>
        <p class="text-[11px] text-gray-500 mt-2">Meld at the Elder Melder in Astera/Seliana. Requires Research Points + materials for Melding Points.</p>
      </section>
    {/if}

    <section>
      <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-3">
        How to Obtain
      </h2>
      <DropTable sources={item.sources} />
    </section>
  {/if}
</div>
