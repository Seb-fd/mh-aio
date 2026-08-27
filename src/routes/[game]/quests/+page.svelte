<script lang="ts">
  import { goto } from '$app/navigation';
  import { selectedGame } from '$lib/stores/game';
  import { api, type Quest } from '$lib/api';
  import Card from '$lib/components/ui/card.svelte';

  const game = $derived($selectedGame);
  const dbId = $derived(game?.dbId);

  let quests = $state<Quest[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let hubFilter = $state<string>('elder');

  $effect(() => {
    if (dbId == null) return;
    loading = true;
    error = null;
    api.getQuests(dbId)
      .then((data) => {
        quests = data;
      })
      .catch((e) => {
        error = String(e);
      })
      .finally(() => {
        loading = false;
      });
  });

  const hubMeta: Record<string, { label: string; sub: string; icon: string }> = {
    elder: { label: 'Village Elder', sub: 'Low Rank Village', icon: '🏠' },
    nekoto: { label: 'Village Nekoto', sub: 'High Rank Village', icon: '🐱' },
    guild_low: { label: 'Guild Low', sub: '★1-5', icon: '⚔️' },
    guild_high: { label: 'Guild High', sub: '★6-8', icon: '🛡️' },
    guild_g: { label: 'Guild G', sub: 'G★1-3', icon: '👑' },
    event: { label: 'Event Quests', sub: 'Downloadable', icon: '🎉' },
    challenge: { label: 'Challenge Quests', sub: 'Arena challenges', icon: '🏆' },
    training: { label: 'Training School', sub: 'Learn the weapons', icon: '🎓' },
    treasure: { label: 'Treasure Hunt', sub: 'Gather & deliver', icon: '💰' },
    hot_spring: { label: 'Hot Spring', sub: 'Bath quests', icon: '♨️' },
    drink: { label: 'Drink Quests', sub: 'Felyne bar', icon: '🍺' },
    nyanta: { label: 'Nyanta Quests', sub: 'Farm felyne', icon: '🐈' },
    other: { label: 'Other', sub: 'Misc', icon: '📦' },
  };

  const hubs = $derived(
    Array.from(new Set(quests.map(q => q.hub).filter((h): h is string => !!h))).sort((a,b)=>{
      const order = ['elder','nekoto','guild_low','guild_high','guild_g','event','challenge','training','treasure','hot_spring','drink','nyanta','other'];
      return order.indexOf(a)-order.indexOf(b);
    })
  );

  // Default to first hub when quests load
  $effect(() => {
    if (quests.length > 0 && !hubs.includes(hubFilter)) {
      hubFilter = hubs[0] ?? 'elder';
    }
  });

  const hubCounts = $derived(
    hubs.reduce((acc, h) => { acc[h] = quests.filter(q=>q.hub===h).length; return acc; }, {} as Record<string,number>)
  );

  const filtered = $derived(
    quests.filter(q => q.hub === hubFilter)
  );

  // Group quests by difficulty stars (ascending), collapsible accordion
  interface StarGroup {
    stars: number | null;
    label: string;
    items: Quest[];
  }

  // Set of star levels explicitly toggled by the user
  let expandedStars = $state<Set<number | null>>(new Set());
  // Whether the user has interacted; before that, first group is open by default
  let userToggled = $state(false);

  const starGroups = $derived.by<StarGroup[]>(() => {
    const map = new Map<number | null, Quest[]>();
    for (const q of filtered) {
      const key = q.stars ?? null;
      const arr = map.get(key) ?? [];
      arr.push(q);
      map.set(key, arr);
    }
    const groups: StarGroup[] = [];
    for (const [stars, items] of map.entries()) {
      groups.push({
        stars,
        label: stars == null ? 'Unknown' : groupLabel(stars),
        items,
      });
    }
    groups.sort((a, b) => (a.stars ?? -1) - (b.stars ?? -1));
    return groups;
  });

  // Default open = first group, until the user toggles
  const defaultOpenStars = $derived(starGroups[0]?.stars ?? null);

  function groupLabel(stars: number): string {
    if (hubFilter === 'guild_g') return `G★${stars}`;
    return `★${stars}`;
  }

  function isOpen(stars: number | null): boolean {
    if (!userToggled) return stars === defaultOpenStars;
    return expandedStars.has(stars);
  }

  function toggle(stars: number | null) {
    const next = new Set(expandedStars);
    if (next.has(stars)) next.delete(stars);
    else next.add(stars);
    expandedStars = next;
    userToggled = true;
  }

  function open(id: number) {
    if (!game) return;
    goto(`/${game.id}/quests/${id}`);
  }

  const rankColor: Record<string, string> = {
    Low: 'bg-gray-700 text-gray-300',
    High: 'bg-blue-900/40 text-blue-300',
    G: 'bg-yellow-900/40 text-yellow-300',
  };

  const typeIcon: Record<string, string> = {
    Hunting: '⚔️',
    Gathering: '🧺',
    Slaying: '🗡️',
    Capturing: '🪤',
    Training: '🎓',
  };

  function starsLabel(q: Quest): string {
    if (q.stars == null) return '';
    if (q.hub === 'guild_g') return `G★${q.stars}`;
    return `★${q.stars}`;
  }
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-6">
    <h1 class="text-2xl font-bold text-gray-100">Quests</h1>
    <p class="text-sm text-gray-500 mt-1">
      {#if game}
        {game.shortName} · {quests.length} quests · {hubs.length} hubs · Key quests, rewards and drop rates
      {:else}
        Select a game first
      {/if}
    </p>
  </div>

  {#if loading}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Loading quests...</p>
    </div>
  {:else if error}
    <div class="bg-red-950/30 border border-red-900 rounded-lg p-8 text-center">
      <p class="text-red-400">Failed to load quests</p>
      <p class="text-gray-500 text-sm mt-2">{error}</p>
    </div>
  {:else if quests.length === 0}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">No quests found for {game?.shortName ?? 'this game'}</p>
    </div>
  {:else}
    <!-- Hub filter tabs -->
    <div class="flex flex-wrap gap-2 mb-6">
      {#each hubs as hub}
        {@const meta = hubMeta[hub] ?? { label: hub, sub: '', icon: '📜' }}
        <button
          onclick={() => (hubFilter = hub)}
          class="px-3 py-1.5 text-xs rounded-full border transition-colors flex items-center gap-1.5"
          style={hubFilter === hub
            ? `background-color: color-mix(in oklab, var(--theme-accent) 12%, transparent); border-color: color-mix(in oklab, var(--theme-accent) 50%, transparent); color: var(--theme-accent);`
            : `background-color: var(--theme-bg-surface); border-color: var(--theme-border); color: rgb(156 163 175);`}
        >
          <span>{meta.icon}</span>
          <span>{meta.label}</span>
          <span class="text-[10px] opacity-60">({hubCounts[hub] ?? 0})</span>
        </button>
      {/each}
    </div>

    {@const meta = hubMeta[hubFilter] ?? { label: hubFilter ?? 'Unknown', sub: '', icon: '📜' }}
    <div class="mb-6">
      <div class="flex items-center gap-2 mb-3">
        <span class="text-sm">{meta.icon}</span>
        <h2 class="text-sm font-semibold text-gray-200">{meta.label}</h2>
        <span class="text-[11px] text-gray-500">{meta.sub} · {filtered.length}</span>
        <div class="flex-1 h-px bg-[var(--theme-border)] ml-2"></div>
      </div>
      {#if filtered.length === 0}
        <div class="border rounded-lg p-8 text-center themed-card">
          <p class="text-gray-400">No quests in this hub</p>
        </div>
      {:else}
        <div class="space-y-2">
          {#each starGroups as group}
            <div class="rounded-lg border border-[var(--theme-border)] bg-[var(--theme-bg-surface)] overflow-hidden">
              <button
                onclick={() => toggle(group.stars)}
                class="w-full flex items-center gap-2.5 px-4 py-2.5 text-left bg-[var(--theme-bg-elevated)] hover:bg-[var(--theme-bg-elevated)]/70 transition-colors"
              >
                <span class="text-[10px] text-gray-400 transition-transform {isOpen(group.stars) ? 'rotate-90' : ''}">▶</span>
                <span class="text-sm font-semibold text-gray-100">{group.label}</span>
                <span class="text-[10px] text-gray-500">({group.items.length})</span>
                <div class="flex-1 h-px bg-[var(--theme-border)] ml-1"></div>
              </button>
              {#if isOpen(group.stars)}
                <div class="p-2 space-y-2">
                  {#each group.items as quest}
                    <button onclick={() => open(quest.id)} class="w-full text-left">
                      <Card class="p-4 border transition-all cursor-pointer themed-card">
                        <div class="flex items-start justify-between gap-3">
                          <div class="min-w-0 flex-1">
                            <div class="flex items-center gap-2 mb-1 flex-wrap">
                              <span class="text-base">{typeIcon[quest.type ?? ''] ?? '📜'}</span>
                              <h3 class="font-semibold text-gray-100">{quest.name}</h3>
                              {#if quest.stars}
                                <span class="text-[10px] px-1.5 py-0.5 rounded bg-[var(--theme-bg-elevated)] text-gray-300 border border-[var(--theme-border)]">{starsLabel(quest)}</span>
                              {/if}
                              {#if quest.is_key_quest}
                                <span class="text-[10px] uppercase tracking-wide px-1.5 py-0.5 rounded bg-yellow-500/10 text-yellow-500 border border-yellow-500/30">
                                  Key
                                </span>
                              {/if}
                              {#if quest.is_urgent}
                                <span class="text-[10px] uppercase tracking-wide px-1.5 py-0.5 rounded bg-red-500/10 text-red-400 border border-red-500/30">
                                  Urgent
                                </span>
                              {/if}
                            </div>
                            <div class="flex flex-wrap gap-3 text-xs text-gray-500">
                              {#if quest.type}
                                <span>{quest.type}</span>
                              {/if}
                              {#if quest.location}
                                <span>📍 {quest.location}</span>
                              {/if}
                              {#if quest.client}
                                <span>👤 {quest.client}</span>
                              {/if}
                              {#if quest.time_limit}
                                <span>⏱️ {quest.time_limit} min</span>
                              {/if}
                              {#if quest.requirements}
                                <span class="text-amber-400/70">🔓 {quest.requirements}</span>
                              {/if}
                            </div>
                          </div>
                          {#if quest.rank}
                            <span class="text-[10px] uppercase tracking-wide px-2 py-0.5 rounded shrink-0 {rankColor[quest.rank] ?? 'bg-gray-800 text-gray-400'}">
                              {quest.rank}
                            </span>
                          {/if}
                        </div>
                      </Card>
                    </button>
                  {/each}
                </div>
              {/if}
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</div>
