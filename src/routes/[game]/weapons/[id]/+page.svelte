<script lang="ts">
  import { page } from '$app/stores';
  import { api, type WeaponDetail } from '$lib/api';
  import DetailHeader from '$lib/components/detail-header.svelte';
  import MaterialList from '$lib/components/material-list.svelte';

  const id = $derived(Number($page.params.id));
  let weapon = $state<WeaponDetail | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);

  $effect(() => {
    if (!id || Number.isNaN(id)) return;
    loading = true;
    error = null;
    api.getWeaponDetail(id)
      .then((data) => {
        weapon = data;
      })
      .catch((e) => {
        error = String(e);
      })
      .finally(() => {
        loading = false;
      });
  });

  function elementColor(elem: string | null): string {
    if (!elem) return 'text-gray-400';
    const lower = elem.toLowerCase();
    if (lower === 'fire') return 'text-orange-400';
    if (lower === 'water') return 'text-blue-400';
    if (lower === 'thunder') return 'text-yellow-400';
    if (lower === 'ice') return 'text-cyan-400';
    if (lower === 'dragon') return 'text-purple-400';
    return 'text-gray-400';
  }
</script>

<div class="max-w-5xl mx-auto">
  {#if loading}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Loading weapon...</p>
    </div>
  {:else if error}
    <div class="bg-red-950/30 border border-red-900 rounded-lg p-8 text-center">
      <p class="text-red-400">Failed to load weapon</p>
      <p class="text-gray-500 text-sm mt-2">{error}</p>
    </div>
  {:else if !weapon}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Weapon not found</p>
    </div>
  {:else}
    <DetailHeader
      title={weapon.name}
      subtitle={weapon.weapon_type}
      icon="⚔️"
      tags={[
        { label: `Rarity ${weapon.rarity ?? 1}`, color: 'bg-[var(--theme-bg-elevated)] text-gray-300 border-[var(--theme-border)]' },
        ...(weapon.element_type && weapon.element_type !== ''
          ? [{ label: `${weapon.element_type} ${weapon.element_value ?? 0}`, color: `bg-[var(--theme-bg-elevated)] ${elementColor(weapon.element_type)} border-[var(--theme-border-strong)]` }]
          : []),
      ]}
    />

    <div class="grid grid-cols-2 sm:grid-cols-4 gap-3 mb-8">
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-500">Attack</p>
        <p class="text-2xl font-bold text-gray-100 mt-1">{weapon.attack ?? 0}</p>
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-500">Affinity</p>
        <p class="text-2xl font-bold mt-1" class:text-emerald-400={(weapon.affinity ?? 0) > 0} class:text-red-400={(weapon.affinity ?? 0) < 0} class:text-gray-100={(weapon.affinity ?? 0) === 0}>
          {weapon.affinity ?? 0}%
        </p>
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-500">Element</p>
        <p class="text-lg font-bold mt-1 {elementColor(weapon.element_type)}">
          {weapon.element_type && weapon.element_type !== '' ? `${weapon.element_type} ${weapon.element_value ?? 0}` : '—'}
        </p>
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-500">Cost</p>
        <p class="text-lg font-bold mt-1" style="color: var(--theme-accent);">{weapon.crafting_cost ?? 0}z</p>
      </div>
    </div>

    {#if weapon.description}
      <section class="mb-8">
        <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-3">Description</h2>
        <div class="rounded-lg border themed-card p-5 leading-relaxed text-gray-200 text-[15px]">
          {weapon.description}
        </div>
      </section>
    {/if}

    <section>
      <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-3">Crafting Materials</h2>
      <MaterialList materials={weapon.materials} showCraftingCost={false} />
    </section>
  {/if}
</div>
