<script lang="ts">
  import { page } from '$app/stores';
  import { api, type Skill } from '$lib/api';
  import DetailHeader from '$lib/components/detail-header.svelte';

  const id = $derived(Number($page.params.id));
  let skill = $state<Skill | null>(null);
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
      subtitle={skill.max_level ? `Max Level ${skill.max_level}` : ''}
      icon="✨"
      tags={[
        { label: `Lv 1-${skill.max_level ?? 1}`, color: 'bg-[var(--theme-bg-elevated)] text-gray-300 border-[var(--theme-border)]' },
      ]}
    />

    {#if skill.description}
      <section>
        <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-3">Effect</h2>
        <div class="rounded-lg border themed-card p-5 leading-relaxed text-gray-200 text-[15px]">
          {skill.description}
        </div>
      </section>
    {/if}
  {/if}
</div>
