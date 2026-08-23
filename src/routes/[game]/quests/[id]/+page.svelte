<script lang="ts">
  import { page } from '$app/stores';
  import { api, type QuestDetail } from '$lib/api';
  import DetailHeader from '$lib/components/detail-header.svelte';

  const id = $derived(Number($page.params.id));
  let quest = $state<QuestDetail | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);

  $effect(() => {
    if (!id || Number.isNaN(id)) return;
    loading = true;
    error = null;
    api.getQuestDetail(id)
      .then((data) => {
        quest = data;
      })
      .catch((e) => {
        error = String(e);
      })
      .finally(() => {
        loading = false;
      });
  });

  const typeIcon: Record<string, string> = {
    Hunting: '⚔️',
    Gathering: '🧺',
    Slaying: '🗡️',
    Capturing: '🪤',
  };

  const rankColor: Record<string, string> = {
    Low: 'bg-gray-700 text-gray-300',
    High: 'bg-blue-900/40 text-blue-300',
    G: 'bg-yellow-900/40 text-yellow-300',
  };
</script>

<div class="max-w-5xl mx-auto">
  {#if loading}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Loading quest...</p>
    </div>
  {:else if error}
    <div class="bg-red-950/30 border border-red-900 rounded-lg p-8 text-center">
      <p class="text-red-400">Failed to load quest</p>
      <p class="text-gray-500 text-sm mt-2">{error}</p>
    </div>
  {:else if !quest}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Quest not found</p>
    </div>
  {:else}
    <DetailHeader
      title={quest.name}
      subtitle={quest.type ?? ''}
      icon={typeIcon[quest.type ?? ''] ?? '📜'}
      tags={[
        { label: quest.rank ?? 'Unknown', color: rankColor[quest.rank ?? ''] ?? 'bg-gray-800 text-gray-300' },
        ...(quest.is_key_quest ? [{ label: 'Key Quest', color: 'bg-yellow-500/10 text-yellow-500 border border-yellow-500/30' }] : []),
      ]}
    />

    <div class="grid grid-cols-2 sm:grid-cols-4 gap-3 mb-8">
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-500">Type</p>
        <p class="text-base font-semibold text-gray-100 mt-1">{quest.type ?? '—'}</p>
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-500">Location</p>
        <p class="text-base font-semibold text-gray-100 mt-1">{quest.location ?? '—'}</p>
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-500">Time</p>
        <p class="text-base font-semibold text-gray-100 mt-1">{quest.time_limit ?? '—'} min</p>
      </div>
      <div class="rounded-lg border themed-card p-3 text-center">
        <p class="text-[10px] uppercase tracking-wide text-gray-500">Faints</p>
        <p class="text-base font-semibold text-gray-100 mt-1">{quest.faints_allowed ?? '—'}</p>
      </div>
    </div>

    {#if quest.description}
      <section class="mb-8">
        <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-3">Description</h2>
        <div class="rounded-lg border themed-card p-5 leading-relaxed text-gray-200 text-[15px]">
          {quest.description}
        </div>
      </section>
    {/if}

    {#if quest.objective}
      <section>
        <h2 class="text-xs uppercase tracking-wider text-gray-500 font-semibold mb-3">Objective</h2>
        <div class="rounded-lg border themed-card p-4 text-gray-200">
          {quest.objective}
        </div>
      </section>
    {/if}
  {/if}
</div>
