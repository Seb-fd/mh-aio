<script lang="ts">
  import { goto } from '$app/navigation';
  import { selectedGame } from '$lib/stores/game';
  import { api, type SearchResult } from '$lib/api';

  const game = $derived($selectedGame);

  let query = $state('');
  let results = $state<SearchResult[]>([]);
  let loading = $state(false);
  let open = $state(false);
  let activeIndex = $state(-1);
  let debounceTimer: ReturnType<typeof setTimeout> | undefined;

  const kindMeta: Record<string, { icon: string; label: string }> = {
    monster: { icon: '🐉', label: 'Monster' },
    item: { icon: '📦', label: 'Item' },
    skill: { icon: '✨', label: 'Skill' },
    weapon: { icon: '⚔️', label: 'Weapon' },
    armor: { icon: '🛡️', label: 'Armor' },
    armor_set: { icon: '🛡️', label: 'Armor Set' },
    quest: { icon: '📜', label: 'Quest' },
    decoration: { icon: '💎', label: 'Decoration' },
  };

  function onInput() {
    open = true;
    activeIndex = -1;
    clearTimeout(debounceTimer);
    const q = query.trim();
    if (q.length < 2) {
      results = [];
      loading = false;
      return;
    }
    loading = true;
    debounceTimer = setTimeout(() => runSearch(q), 250);
  }

  async function runSearch(q: string) {
    if (!game) return;
    try {
      const res = await api.globalSearch(game.dbId, q);
      results = res ?? [];
    } catch (e) {
      console.error('[search]', e);
      results = [];
    } finally {
      loading = false;
    }
  }

  function go(r: SearchResult) {
    if (!game) return;
    open = false;
    query = '';
    results = [];
    goto(`/${game.id}${r.route}`);
  }

  function onKeydown(e: KeyboardEvent) {
    if (!open || results.length === 0) {
      if (e.key === 'Escape') { open = false; query = ''; }
      return;
    }
    if (e.key === 'ArrowDown') {
      e.preventDefault();
      activeIndex = Math.min(activeIndex + 1, results.length - 1);
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      activeIndex = Math.max(activeIndex - 1, -1);
    } else if (e.key === 'Enter' && activeIndex >= 0) {
      e.preventDefault();
      go(results[activeIndex]);
    } else if (e.key === 'Escape') {
      open = false;
    }
  }

  function onBlur() {
    setTimeout(() => { open = false; }, 150);
  }

  const grouped = $derived.by(() => {
    const map = new Map<string, SearchResult[]>();
    for (const r of results) {
      const arr = map.get(r.kind) ?? [];
      arr.push(r);
      map.set(r.kind, arr);
    }
    return map;
  });

  const groupedEntries = $derived([...grouped.entries()]);
  let flatIndex = -1;
  function indexInGroup(group: string, idx: number): number {
    // compute flattened index for highlight navigation
    let base = 0;
    for (const [g, arr] of groupedEntries) {
      if (g === group) { return base + idx; }
      base += arr.length;
    }
    return base;
  }
</script>

<div class="relative w-full max-w-xs md:max-w-sm" onfocusout={onBlur}>
  <div class="relative">
    <span class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500 text-sm" aria-hidden="true">🔍</span>
    <label for="global-search-input" class="sr-only">Search {game?.shortName ?? 'current game'}</label>
    <input
      id="global-search-input"
      type="text"
      role="combobox"
      aria-expanded={open && query.trim().length >= 2}
      aria-controls="global-search-listbox"
      aria-autocomplete="list"
      aria-activedescendant={activeIndex >= 0 ? `search-opt-${activeIndex}` : undefined}
      placeholder="Search {game?.shortName ?? 'this game'} — skills, monsters, items…"
      bind:value={query}
      oninput={onInput}
      onkeydown={onKeydown}
      ondragstart={() => false}
      class="w-full pl-9 pr-8 py-1.5 text-sm rounded-full border placeholder-gray-600 focus:outline-none focus:border-[var(--theme-primary)]/60"
      style="background-color: var(--theme-bg-elevated); border-color: var(--theme-border); color: var(--theme-text-accent);"
    />
    {#if loading}
      <span class="absolute right-3 top-1/2 -translate-y-1/2 text-xs text-gray-500 w-3 h-3 border-2 border-gray-500 border-t-transparent rounded-full animate-spin"></span>
    {/if}
  </div>

  {#if open && query.trim().length >= 2}
    <div
      id="global-search-listbox"
      role="listbox"
      aria-label="Search results"
      class="absolute z-50 mt-2 w-full max-h-[70vh] overflow-auto rounded-xl border shadow-2xl"
      style="background-color: var(--theme-bg-surface); border-color: var(--theme-border-strong);"
    >
      {#if groupedEntries.length === 0}
        <div class="px-4 py-6 text-center text-sm text-gray-500">
          No matches for “{query}”
        </div>
      {:else}
        {#each groupedEntries as [kind, items]}
          {@const meta = kindMeta[kind] ?? { icon: '🔍', label: kind }}
          <div class="px-3 pt-2 pb-1 text-[10px] uppercase tracking-widest font-semibold text-gray-500">
            {meta.icon} {meta.label}
          </div>
          {#each items as r, idx}
            {@const flat = indexInGroup(kind, idx)}
            {@const selected = flat === activeIndex}
            <button
              id="search-opt-{flat}"
              role="option"
              aria-selected={selected}
              class="w-full flex items-center gap-2.5 px-3 py-2 text-left transition-colors {selected ? 'bg-[var(--theme-primary)]/10' : 'hover:bg-[var(--theme-bg-elevated)]'}"
              onmouseenter={() => (activeIndex = flat)}
              onclick={() => go(r)}
              value="search-{r.kind}-{r.id}"
            >
              <span class="text-base shrink-0">{meta.icon}</span>
              <div class="flex-1 min-w-0">
                <div class="text-sm text-gray-100 truncate">{r.name}</div>
                {#if r.subtitle}
                  <div class="text-[11px] text-gray-500 truncate">{r.subtitle}</div>
                {/if}
              </div>
            </button>
          {/each}
        {/each}
      {/if}
    </div>
  {/if}
</div>
