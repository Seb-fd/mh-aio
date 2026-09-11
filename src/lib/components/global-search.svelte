<script lang="ts">
  import { goto } from '$app/navigation'
  import { selectedGame } from '$lib/stores/game'
  import { api, type SearchResult } from '$lib/api'
  import { createQuery } from '@tanstack/svelte-query'
  import { browser } from '$app/environment'
  import type { Component } from 'svelte'
  import {
    Search,
    Swords,
    Package,
    Sparkles,
    Shield,
    ShieldPlus,
    ScrollText,
    Gem,
    CircleHelp,
  } from '@lucide/svelte'

  const game = $derived($selectedGame)

  let {
    inputId = 'global-search-input',
    autoFocus = false,
  }: { inputId?: string; autoFocus?: boolean } = $props()

  let inputEl: HTMLInputElement | undefined = $state(undefined)
  $effect(() => {
    // User-initiated focus only (mobile search toggle) — avoids a11y_autofocus lint.
    if (autoFocus && inputEl) inputEl.focus()
  })

  let query = $state('')
  let debouncedQ = $state('')
  let open = $state(false)
  let activeIndex = $state(-1)
  let debounceTimer: ReturnType<typeof setTimeout> | undefined

  interface KindMeta {
    Icon: Component
    label: string
  }

  const kindMeta: Record<string, KindMeta> = {
    monster: { Icon: Swords, label: 'Monster' },
    item: { Icon: Package, label: 'Item' },
    skill: { Icon: Sparkles, label: 'Skill' },
    weapon: { Icon: Swords, label: 'Weapon' },
    armor: { Icon: Shield, label: 'Armor' },
    armor_set: { Icon: ShieldPlus, label: 'Armor Set' },
    quest: { Icon: ScrollText, label: 'Quest' },
    decoration: { Icon: Gem, label: 'Decoration' },
  }
  const fallbackKind: KindMeta = { Icon: CircleHelp, label: '' }

  const RECENT_KEY = 'mh-search-recent'
  const MAX_RECENT = 5

  function loadRecents(): SearchResult[] {
    if (!browser) return []
    try {
      const raw = localStorage.getItem(RECENT_KEY)
      if (!raw) return []
      const parsed = JSON.parse(raw)
      if (!Array.isArray(parsed)) return []
      return parsed.filter(
        (r): r is SearchResult =>
          r &&
          typeof r.route === 'string' &&
          typeof r.name === 'string' &&
          typeof r.kind === 'string',
      )
    } catch {
      return []
    }
  }

  let recents = $state<SearchResult[]>(loadRecents())

  function saveRecent(r: SearchResult) {
    const next = [{ ...r, id: r.id }, ...recents.filter((x) => x.route !== r.route)].slice(
      0,
      MAX_RECENT,
    )
    recents = next
    try {
      localStorage.setItem(RECENT_KEY, JSON.stringify(next))
    } catch {
      // storage full/blocked — recents stay in memory only
    }
  }

  function clearRecents() {
    recents = []
    try {
      localStorage.removeItem(RECENT_KEY)
    } catch {
      // ignore
    }
  }

  function onInput() {
    open = true
    activeIndex = -1
    clearTimeout(debounceTimer)
    const q = query.trim()
    if (q.length < 2) {
      debouncedQ = ''
      return
    }
    debounceTimer = setTimeout(() => {
      debouncedQ = q
    }, 250)
  }

  function onFocus() {
    open = true
  }

  const searchQuery = createQuery(() => ({
    queryKey: ['global-search', game?.dbId ?? 0, debouncedQ],
    queryFn: () => api.globalSearch(game!.dbId, debouncedQ),
    enabled: !!game && debouncedQ.length >= 2,
    staleTime: 60_000,
  }))

  const results = $derived<SearchResult[]>(searchQuery.data ?? [])
  const loading = $derived(searchQuery.isFetching && debouncedQ.length >= 2)

  // Flat list for keyboard navigation: recents when idle, results when searching
  const searching = $derived(query.trim().length >= 2)
  const flatList = $derived(searching ? results : recents)
  const showDropdown = $derived(open && (searching || recents.length > 0))

  function go(r: SearchResult) {
    if (!game) return
    saveRecent(r)
    open = false
    query = ''
    debouncedQ = ''
    activeIndex = -1
    goto(`/${game.id}${r.route}`)
  }

  function onKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      open = false
      if (searching) {
        query = ''
        debouncedQ = ''
      }
      return
    }
    if (!open || flatList.length === 0) return
    if (e.key === 'ArrowDown') {
      e.preventDefault()
      activeIndex = Math.min(activeIndex + 1, flatList.length - 1)
    } else if (e.key === 'ArrowUp') {
      e.preventDefault()
      activeIndex = Math.max(activeIndex - 1, -1)
    } else if (e.key === 'Enter' && activeIndex >= 0 && flatList[activeIndex]) {
      e.preventDefault()
      go(flatList[activeIndex])
    }
  }

  /** Global shortcut: `/` or Ctrl/Cmd+K focuses this input when visible. */
  function onWindowKeydown(e: KeyboardEvent) {
    const target = e.target as HTMLElement | null
    const inField =
      !!target &&
      (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA' || target.isContentEditable)
    if ((e.key === '/' && !inField) || ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'k')) {
      if (!inputEl || inputEl.offsetParent === null) return
      e.preventDefault()
      inputEl.focus()
      open = true
    }
  }

  function onBlur() {
    setTimeout(() => {
      open = false
    }, 150)
  }

  const grouped = $derived.by(() => {
    const map = new Map<string, SearchResult[]>()
    for (const r of results) {
      const arr = map.get(r.kind) ?? []
      arr.push(r)
      map.set(r.kind, arr)
    }
    return map
  })

  const groupedEntries = $derived([...grouped.entries()])
  const listboxId = $derived(`${inputId}-listbox`)
  function indexInGroup(group: string, idx: number): number {
    // compute flattened index for highlight navigation
    let base = 0
    for (const [g, arr] of groupedEntries) {
      if (g === group) {
        return base + idx
      }
      base += arr.length
    }
    return base
  }
</script>

<svelte:window onkeydown={onWindowKeydown} />

<div class="relative w-full max-w-xs md:max-w-sm" onfocusout={onBlur}>
  <div class="relative">
    <Search
      class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-gray-400"
      aria-hidden="true"
    />
    <label for={inputId} class="sr-only">Search {game?.shortName ?? 'current game'}</label>
    <input
      bind:this={inputEl}
      id={inputId}
      type="text"
      role="combobox"
      aria-expanded={showDropdown}
      aria-controls={listboxId}
      aria-autocomplete="list"
      aria-activedescendant={activeIndex >= 0 ? `${inputId}-opt-${activeIndex}` : undefined}
      placeholder="Search {game?.shortName ?? 'this game'} — skills, monsters, items… ( / )"
      bind:value={query}
      oninput={onInput}
      onfocus={onFocus}
      onkeydown={onKeydown}
      ondragstart={() => false}
      autocomplete="off"
      spellcheck={false}
      class="w-full pl-9 pr-8 py-1.5 text-sm rounded-full border placeholder-gray-600 focus:outline-none focus:border-[var(--theme-primary)]/60 min-h-[44px] sm:min-h-0"
      style="background-color: var(--theme-bg-elevated); border-color: var(--theme-border); color: var(--theme-text-accent);"
    />
    {#if loading}
      <span
        class="absolute right-3 top-1/2 -translate-y-1/2 text-xs text-gray-400 w-3 h-3 border-2 border-gray-500 border-t-transparent rounded-full animate-spin"
      ></span>
    {/if}
  </div>

  {#if showDropdown}
    <div
      id={listboxId}
      role="listbox"
      aria-label="Search results"
      class="absolute z-50 mt-2 w-full max-h-[70vh] overflow-auto rounded-xl border shadow-2xl"
      style="background-color: var(--theme-bg-surface); border-color: var(--theme-border-strong);"
    >
      {#if searching}
        {#if groupedEntries.length === 0 && !loading}
          <div class="px-4 py-6 text-center text-sm text-gray-400">
            No matches for “{query}”
          </div>
        {:else}
          {#each groupedEntries as [kind, items]}
            {@const rawGroup = kindMeta[kind]}
            {@const groupMeta: KindMeta = rawGroup
              ? { ...rawGroup }
              : { ...fallbackKind, label: kind }}
            <div
              class="px-3 pt-2 pb-1 text-[10px] uppercase tracking-widest font-semibold text-gray-400 flex items-center gap-1.5"
            >
              <groupMeta.Icon class="h-3 w-3" aria-hidden="true" />
              {groupMeta.label}
            </div>
            {#each items as r, idx}
              {@const flat = indexInGroup(kind, idx)}
              {@const selected = flat === activeIndex}
              <button
                id="{inputId}-opt-{flat}"
                role="option"
                aria-selected={selected}
                class="w-full flex items-center gap-2.5 px-3 py-2 text-left transition-colors motion-safe:transition-colors motion-reduce:transition-none min-h-[44px] {selected
                  ? 'bg-[var(--theme-primary)]/10'
                  : 'hover:bg-[var(--theme-bg-elevated)]'}"
                onmouseenter={() => (activeIndex = flat)}
                onclick={() => go(r)}
                value="search-{r.kind}-{r.id}"
              >
                <groupMeta.Icon class="h-4 w-4 shrink-0 text-gray-400" aria-hidden="true" />
                <div class="flex-1 min-w-0">
                  <div class="text-sm text-gray-100 truncate">{r.name}</div>
                  {#if r.subtitle}
                    <div class="text-[11px] text-gray-400 truncate">{r.subtitle}</div>
                  {/if}
                </div>
              </button>
            {/each}
          {/each}
        {/if}
      {:else}
        <div
          class="px-3 pt-2 pb-1 flex items-center justify-between text-[10px] uppercase tracking-widest font-semibold text-gray-400"
        >
          <span>Recent</span>
          <button
            type="button"
            onclick={clearRecents}
            class="normal-case tracking-normal font-normal text-gray-400 hover:text-gray-300 min-h-[44px] px-2"
          >
            Clear
          </button>
        </div>
        {#each recents as r, idx}
          {@const rawRecent = kindMeta[r.kind]}
          {@const recentMeta: KindMeta = rawRecent
            ? { ...rawRecent }
            : { ...fallbackKind, label: r.kind }}
          {@const selected = idx === activeIndex}
          <button
            id="{inputId}-opt-{idx}"
            role="option"
            aria-selected={selected}
            class="w-full flex items-center gap-2.5 px-3 py-2 text-left transition-colors motion-safe:transition-colors motion-reduce:transition-none min-h-[44px] {selected
              ? 'bg-[var(--theme-primary)]/10'
              : 'hover:bg-[var(--theme-bg-elevated)]'}"
            onmouseenter={() => (activeIndex = idx)}
            onclick={() => go(r)}
            value="search-{r.kind}-{r.id}"
          >
            <recentMeta.Icon class="h-4 w-4 shrink-0 text-gray-400" aria-hidden="true" />
            <div class="flex-1 min-w-0">
              <div class="text-sm text-gray-100 truncate">{r.name}</div>
              {#if r.subtitle}
                <div class="text-[11px] text-gray-400 truncate">{r.subtitle}</div>
              {/if}
            </div>
          </button>
        {/each}
      {/if}
    </div>
  {/if}
</div>
