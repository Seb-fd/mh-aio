<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { api, type SkillDetail } from '$lib/api';
  import DetailHeader from '$lib/components/detail-header.svelte';

  const id = $derived(Number($page.params.id));
  const game = $derived($page.params.game);
  let skill = $state<SkillDetail | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);

  $effect(() => {
    if (!id || Number.isNaN(id)) return;
    loading = true;
    error = null;
    api.getSkillDetail(id)
      .then((data) => {
        skill = data;
      })
      .catch((e) => {
        error = String(e);
      })
      .finally(() => {
        loading = false;
      });
  });

  const sortedLevels = $derived(
    skill ? [...skill.levels].sort((a,b) => b.points - a.points) : []
  );
  const positive = $derived(sortedLevels.filter(l => l.points > 0));
  const negative = $derived(sortedLevels.filter(l => l.points < 0));

  function openArmor(aid: number) {
    goto(`/${game}/armor/${aid}`);
  }
  function openWeapon(wid: number) {
    goto(`/${game}/weapons/${wid}`);
  }
  function openItem(iid: number | null) {
    if (iid) goto(`/${game}/items/${iid}`);
  }
  function openDecoration(did: number) {
    goto(`/${game}/decorations/${did}`);
  }
</script>

<div class="max-w-5xl mx-auto">
  {#if loading}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Loading skill...</p>
    </div>
  {:else if error}
    <div class="bg-red-950/30 border border-red-900 rounded-lg p-8 text-center">
      <p class="text-red-400">Failed to load skill</p>
      <p class="text-gray-500 text-sm mt-2">{error}</p>
    </div>
  {:else if !skill}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Skill not found</p>
    </div>
  {:else}
    <DetailHeader
      title={skill.name}
      subtitle={skill.description ?? ''}
      icon="✨"
      tags={[
        { label: `${skill.levels.length} abilities`, color: 'bg-[var(--theme-bg-elevated)] text-gray-300 border-[var(--theme-border)]' },
        ...(skill.armors.length ? [{ label: `${skill.armors.length} armor pieces`, color: 'bg-[var(--theme-bg-elevated)] text-gray-300 border-[var(--theme-border)]' }] : []),
        ...(skill.decorations.length ? [{ label: `${skill.decorations.length} jewels`, color: 'bg-[var(--theme-bg-elevated)] text-gray-300 border-[var(--theme-border)]' }] : []),
      ]}
    />

    {#if skill.description}
      <section class="mb-8">
        <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-3">Description</h2>
        <div class="rounded-lg border themed-card p-5 leading-relaxed text-gray-200 text-[15px]">
          {skill.description}
        </div>
      </section>
    {/if}

    <!-- Levels / Abilities -->
    <section class="mb-8">
      <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-3">Skill Levels & Abilities</h2>
      {#if skill.levels.length === 0}
        <div class="rounded-lg border themed-card p-5 text-center text-gray-500 text-sm">No ability data available.</div>
      {:else}
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
          <!-- Positive -->
          <div class="rounded-lg border themed-card overflow-hidden">
            <div class="px-4 py-2 bg-emerald-950/30 border-b border-emerald-900/40 flex items-center gap-2">
              <span class="text-emerald-300 text-sm font-semibold">Positive Thresholds</span>
              <span class="text-[10px] px-1.5 py-0.5 rounded bg-emerald-900/40 text-emerald-300 border border-emerald-800">{positive.length}</span>
            </div>
            {#if positive.length === 0}
              <p class="p-4 text-center text-gray-500 text-sm">No positive levels</p>
            {:else}
              <div class="divide-y divide-gray-800">
                {#each positive as lvl}
                  <div class="px-4 py-3 flex items-center justify-between gap-3">
                    <div>
                      <p class="text-sm font-medium text-gray-100">{lvl.ability_name}</p>
                      {#if lvl.description}
                        <p class="text-xs text-gray-400 mt-0.5">{lvl.description}</p>
                      {/if}
                    </div>
                    <span class="shrink-0 text-xs font-bold px-2.5 py-1 rounded-full bg-emerald-900/40 text-emerald-300 border border-emerald-800">+{lvl.points}</span>
                  </div>
                {/each}
              </div>
            {/if}
          </div>

          <!-- Negative -->
          <div class="rounded-lg border themed-card overflow-hidden">
            <div class="px-4 py-2 bg-red-950/30 border-b border-red-900/40 flex items-center gap-2">
              <span class="text-red-300 text-sm font-semibold">Negative Thresholds</span>
              <span class="text-[10px] px-1.5 py-0.5 rounded bg-red-900/40 text-red-300 border border-red-800">{negative.length}</span>
            </div>
            {#if negative.length === 0}
              <p class="p-4 text-center text-gray-500 text-sm">No negative levels — safe to stack</p>
            {:else}
              <div class="divide-y divide-gray-800">
                {#each negative as lvl}
                  <div class="px-4 py-3 flex items-center justify-between gap-3">
                    <div>
                      <p class="text-sm font-medium text-gray-100">{lvl.ability_name}</p>
                      {#if lvl.description}
                        <p class="text-xs text-gray-400 mt-0.5">{lvl.description}</p>
                      {/if}
                    </div>
                    <span class="shrink-0 text-xs font-bold px-2.5 py-1 rounded-full bg-red-900/40 text-red-300 border border-red-800">{lvl.points}</span>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        </div>
        <p class="text-[11px] text-gray-500 mt-2">Points accumulate from armor and jewels. Reach the threshold to activate the ability. Negative abilities trigger when below 0.</p>
      {/if}
    </section>

    <!-- Decorations / Jewels - enriched with materials & unlock -->
    <section class="mb-8">
      <div class="flex items-center justify-between mb-3">
        <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold">Decorations & Jewels ({skill.decorations.length})</h2>
        {#if skill.decorations.length > 0}
          <a href="/{game}/decorations" class="text-xs text-[var(--theme-text-accent)] hover:underline">View all jewels →</a>
        {/if}
      </div>
      {#if skill.decorations.length === 0}
        <div class="rounded-lg border themed-card p-5 text-center text-gray-500 text-sm">No jewels grant points for this skill. Data is faithful to MHFU - some skills have no jewel.</div>
      {:else}
        <div class="grid grid-cols-1 gap-3">
          {#each skill.decorations as deco}
            <div class="rounded-lg border themed-card overflow-hidden hover:border-[var(--theme-border-strong)] transition-colors">
              <!-- Header -->
              <button onclick={() => openDecoration(deco.id)} class="w-full text-left px-4 py-3 flex items-start justify-between gap-3 hover:bg-[var(--theme-bg-elevated)]/40 transition-colors">
                <div class="min-w-0">
                  <div class="flex items-center gap-2 flex-wrap">
                    <p class="text-sm font-semibold text-gray-100">{deco.name}</p>
                    <span class="inline-flex items-center justify-center w-6 h-6 rounded text-xs font-bold border
                      {deco.slot_size === 1 ? 'bg-gray-800 text-gray-300 border-gray-700' : deco.slot_size === 2 ? 'bg-blue-900/30 text-blue-300 border-blue-800' : 'bg-yellow-900/30 text-yellow-300 border-yellow-800'}">
                      {deco.slot_size ?? '-'}
                    </span>
                    <span class="text-xs px-2 py-0.5 rounded-full font-bold border {deco.skill_points >= 0 ? 'bg-emerald-900/30 text-emerald-300 border-emerald-800' : 'bg-red-900/30 text-red-300 border-red-800'}">
                      {skill.name} {deco.skill_points > 0 ? '+' : ''}{deco.skill_points}
                    </span>
                    {#if deco.secondary_skill_name}
                      <span class="text-xs px-2 py-0.5 rounded bg-[var(--theme-bg-elevated)] text-gray-400 border border-[var(--theme-border)]">
                        {deco.secondary_skill_name} <span class="{(deco.secondary_points ?? 0) >= 0 ? 'text-emerald-300' : 'text-red-300'} font-bold">{(deco.secondary_points ?? 0) > 0 ? '+' : ''}{deco.secondary_points}</span>
                      </span>
                    {/if}
                  </div>
                  <div class="flex flex-wrap gap-1.5 mt-2">
                    <span class="text-[10px] uppercase tracking-wide px-2 py-0.5 rounded bg-[var(--theme-bg-elevated)] text-gray-400 border border-[var(--theme-border)]">{deco.unlock}</span>
                    <span class="text-[10px] px-2 py-0.5 rounded bg-yellow-900/20 text-yellow-300 border border-yellow-800">{deco.price ? `${deco.price}z` : '—'}</span>
                  </div>
                  <p class="text-[11px] text-gray-500 mt-1.5">{deco.acquisition} · Slot {deco.slot_size} jewel · Tap to view full detail</p>
                </div>
                <span class="shrink-0 text-gray-500 text-xs">›</span>
              </button>

              <!-- Materials -->
              <div class="px-4 pb-3">
                <p class="text-[11px] uppercase tracking-wide text-gray-500 font-semibold mb-2">Crafting Materials · 100% faithful</p>
                {#if deco.materials.length === 0}
                  <p class="text-xs text-gray-600">No material data</p>
                {:else}
                  <div class="flex flex-wrap gap-1.5">
                    {#each deco.materials as mat}
                      <button
                        onclick={() => openItem(mat.item_id)}
                        disabled={!mat.item_id}
                        class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs border transition-colors
                          {mat.item_id ? 'bg-[var(--theme-bg-elevated)] text-gray-200 border-[var(--theme-border)] hover:border-[var(--theme-border-strong)] hover:bg-[var(--theme-bg-surface)] cursor-pointer' : 'bg-gray-800 text-gray-400 border-gray-700 cursor-default'}"
                        title={mat.item_id ? 'View item source (monster drops, gathering)' : 'Material not in item DB - faithful name'}
                      >
                        <span class="font-medium">{mat.item_name}</span>
                        <span class="px-1.5 py-0.5 rounded bg-black/20 text-[11px] font-bold">x{mat.quantity}</span>
                      </button>
                    {/each}
                  </div>
                {/if}
                <p class="text-[10px] text-gray-500 mt-2 leading-relaxed">Faithful to MHFU game data (mhfu-db / mhfu-blacksmith). Materials are consumed at the Smith. Base jewels: <span class="text-gray-300">Suiko Jewel</span> (Low), <span class="text-gray-300">Akito Jewel</span> (High), <span class="text-gray-300">Battlefield Jewel</span> (G), <span class="text-gray-300">LapisLazuli Jewel</span> (G). Each material links to its monster drop source.</p>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </section>

    <!-- Armor pieces -->
    <section class="mb-8">
      <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-3">Armor Pieces ({skill.armors.length})</h2>
      {#if skill.armors.length === 0}
        <div class="rounded-lg border themed-card p-5 text-center text-gray-500 text-sm">No armor grants points for this skill.</div>
      {:else}
        <div class="rounded-lg border themed-card overflow-hidden">
          <div class="divide-y divide-gray-800 max-h-[420px] overflow-y-auto">
            {#each skill.armors as armor}
              <button onclick={() => openArmor(armor.id)} class="w-full text-left px-4 py-3 flex items-center justify-between gap-3 hover:bg-[var(--theme-bg-elevated)]/50 transition-colors">
                <div class="min-w-0">
                  <p class="text-sm font-medium text-gray-100 truncate">{armor.name}</p>
                  <p class="text-xs text-gray-500">{armor.slot_type} · {armor.rank} · Rarity {armor.rarity ?? 1} {armor.slots ? `· Slots ${armor.slots}` : ''}</p>
                </div>
                <span class="shrink-0 text-xs font-bold px-2.5 py-1 rounded-full border {armor.points >= 0 ? 'bg-emerald-900/30 text-emerald-300 border-emerald-800' : 'bg-red-900/30 text-red-300 border-red-800'}">
                  {armor.points > 0 ? '+' : ''}{armor.points}
                </span>
              </button>
            {/each}
          </div>
        </div>
        {#if skill.armors.length >= 200}
          <p class="text-[11px] text-gray-500 mt-1">Showing first 200 results — all pieces provide points for {skill.name}.</p>
        {/if}
      {/if}
    </section>

    <!-- Weapons -->
    <section class="mb-8">
      <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-3">Weapons ({skill.weapons.length})</h2>
      {#if skill.weapons.length === 0}
        <div class="rounded-lg border themed-card p-5 text-center text-gray-500 text-sm">No weapons grant points for this skill in current data.</div>
      {:else}
        <div class="rounded-lg border themed-card overflow-hidden">
          <div class="divide-y divide-gray-800 max-h-[420px] overflow-y-auto">
            {#each skill.weapons as wp}
              <button onclick={() => openWeapon(wp.id)} class="w-full text-left px-4 py-3 flex items-center justify-between gap-3 hover:bg-[var(--theme-bg-elevated)]/50 transition-colors">
                <div class="min-w-0">
                  <p class="text-sm font-medium text-gray-100 truncate">{wp.name}</p>
                  <p class="text-xs text-gray-500">{wp.weapon_type} · Rarity {wp.rarity ?? 1} · ATK {wp.attack ?? 0} {wp.slots ? `· Slots ${wp.slots}` : ''}</p>
                </div>
                <span class="shrink-0 text-xs font-bold px-2.5 py-1 rounded-full border {wp.points >= 0 ? 'bg-emerald-900/30 text-emerald-300 border-emerald-800' : 'bg-red-900/30 text-red-300 border-red-800'}">
                  {wp.points > 0 ? '+' : ''}{wp.points}
                </span>
              </button>
            {/each}
          </div>
        </div>
      {/if}
    </section>
  {/if}
</div>
