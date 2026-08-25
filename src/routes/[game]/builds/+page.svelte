<script lang="ts">
  import { selectedGame } from '$lib/stores/game';
  import { api, type Skill, type SkillLevel } from '$lib/api';

  const game = $derived($selectedGame);

  let hr = $state(9);
  let elderStar = $state(9);
  let weaponSlots = $state(3);
  let gender: 'male' | 'female' = $state('male');
  let hunterType: 'blade' | 'gunner' = $state('blade');
  let allowBad = $state(false);
  let includePiercings = $state(true);
  let allowTorsoInc = $state(true);
  let includeDummy = $state(false);
  let sortBy = $state('none');
  let showAdvanced = $state(false);

  type SkillSlot = { skillId: number | null; skillName: string; points: number; filter: string; open: boolean };
  let skillSlots = $state<SkillSlot[]>([
    { skillId: null, skillName: '', points: 10, filter: '', open: false },
    { skillId: null, skillName: '', points: 10, filter: '', open: false },
    { skillId: null, skillName: '', points: 10, filter: '', open: false },
    { skillId: null, skillName: '', points: 10, filter: '', open: false },
    { skillId: null, skillName: '', points: 10, filter: '', open: false },
  ]);

  let allSkills = $state<Skill[]>([]);
  let skillLevelsMap = $state<Map<number, SkillLevel[]>>(new Map());
  let searching = $state(false);
  let error = $state<string | null>(null);
  let results = $state<any[]>([]);
  let resultsCountText = $state('');

  async function loadSkills() {
    if (!game) return;
    try {
      const dbId = game.dbId;
      const skills = await api.getSkills(dbId);
      allSkills = skills;
      const m = new Map<number, SkillLevel[]>();
      await Promise.all(skills.slice(0, 80).map(async (s) => {
        try {
          const detail = await api.getSkillDetail(s.id);
          if (detail) m.set(s.id, detail.levels);
        } catch {}
      }));
      skillLevelsMap = m;
    } catch (e: any) {
      console.error(e);
    }
  }

  $effect(() => { if (game) loadSkills(); });

  function filteredSkills(filter: string) {
    const f = filter.toLowerCase().trim();
    if (!f) return allSkills.slice(0, 40);
    return allSkills.filter(s => s.name.toLowerCase().includes(f)).slice(0, 40);
  }

  function selectSkill(idx: number, skill: Skill) {
    skillSlots[idx].skillId = skill.id;
    skillSlots[idx].skillName = skill.name;
    skillSlots[idx].filter = skill.name;
    skillSlots[idx].open = false;
    const levels = skillLevelsMap.get(skill.id);
    if (levels && levels.length) {
      const pos = [...levels].filter(l => l.points > 0).sort((a,b)=>a.points-b.points)[0];
      if (pos) skillSlots[idx].points = pos.points;
    }
  }

  function clearSkill(idx: number) {
    skillSlots[idx] = { skillId: null, skillName: '', points: 10, filter: '', open: false };
  }

  function getAbilityOptions(skillId: number | null): SkillLevel[] {
    if (skillId == null) return [];
    const levels = skillLevelsMap.get(skillId) ?? [];
    return [...levels].filter(l => l.points > 0).sort((a,b)=>a.points-b.points);
  }

  function applyExample(skills: { name: string, points: number }[]) {
    for (let i = 0; i < 5; i++) {
      if (i < skills.length) {
        const s = allSkills.find(x => x.name.toLowerCase() === skills[i].name.toLowerCase());
        if (s) {
          skillSlots[i].skillId = s.id;
          skillSlots[i].skillName = s.name;
          skillSlots[i].filter = s.name;
          skillSlots[i].points = skills[i].points;
          skillSlots[i].open = false;
        }
      } else {
        clearSkill(i);
      }
    }
  }

  async function doSearch() {
    if (!game) return;
    const reqSkills = skillSlots.filter(s => s.skillId !== null).map(s => ({ skill_id: s.skillId!, points_required: s.points }));
    if (reqSkills.length === 0) { error = 'Choose at least one skill to find armor sets.'; return; }
    if (reqSkills.length > 5) { error = 'You can select up to 5 skills.'; return; }
    searching = true; error = null; results = []; resultsCountText = '';
    try {
      const query = {
        game_id: game.dbId,
        skills: reqSkills,
        hunter_type: hunterType,
        gender,
        hr,
        elder_star: elderStar,
        weapon_slots: weaponSlots,
        include_piercings: includePiercings,
        allow_bad: allowBad,
        allow_torso_inc: allowTorsoInc,
        allow_dummy: includeDummy,
        sort_by: sortBy === 'none' ? null : sortBy,
      };
      const res = await api.searchArmorSets(query as any);
      results = res;
      if (res.length >= 1000) resultsCountText = `Showing first 1000 results`;
      else resultsCountText = `${res.length} ${res.length === 1 ? 'set found' : 'sets found'}`;
    } catch (e: any) {
      error = e?.toString() ?? 'Search failed. Try adjusting your skills or filters.';
    } finally { searching = false; }
  }

  const sortOptions = [
    { v: 'none', l: 'Best Match' },
    { v: 'defence', l: 'Defense' },
    { v: 'slots_spare', l: 'Spare Slots' },
    { v: 'rarity', l: 'Rarity' },
    { v: 'difficulty', l: 'Ease to Craft' },
    { v: 'fire_res', l: 'Fire Res' },
    { v: 'water_res', l: 'Water Res' },
    { v: 'thunder_res', l: 'Thunder Res' },
    { v: 'ice_res', l: 'Ice Res' },
    { v: 'dragon_res', l: 'Dragon Res' },
  ];

  const activeCount = $derived(skillSlots.filter(s => s.skillId !== null).length);
</script>

<div class="max-w-7xl mx-auto">
  <div class="mb-6">
    <h1 class="text-2xl font-bold text-gray-100">Builds</h1>
    <p class="text-sm text-gray-500 mt-1">
      {#if game}Find armor sets for {game.shortName} — pick the skills you want, we handle decorations and slots.{:else}Select a game to start building.{/if}
    </p>
  </div>

  <div class="grid grid-cols-1 lg:grid-cols-[360px_1fr] gap-5">
    <!-- Left: Builder -->
    <div class="space-y-4">
      <!-- Step 1: Hunter Type -->
      <div class="themed-card rounded-xl border p-4">
        <div class="flex items-center gap-2 mb-3">
          <span class="w-6 h-6 rounded-full bg-[var(--theme-primary)] text-white flex items-center justify-center text-xs font-bold">1</span>
          <h2 class="text-sm font-semibold text-gray-200">Hunter Type</h2>
        </div>
        <div class="grid grid-cols-2 gap-2">
          <button onclick={() => hunterType = 'blade'} class="group relative p-3 rounded-xl border-2 text-left transition-all {hunterType==='blade' ? 'border-[var(--theme-primary)] bg-[var(--theme-primary)]/10' : 'border-gray-800 bg-gray-900/50 hover:border-gray-700'}">
            <div class="text-lg">⚔️</div>
            <div class="text-sm font-medium mt-1 {hunterType==='blade' ? 'text-white' : 'text-gray-300'}">Blademaster</div>
            <div class="text-xs text-gray-500">Sword & melee</div>
            {#if hunterType==='blade'}<div class="absolute top-2 right-2 w-2 h-2 rounded-full bg-[var(--theme-primary)]"></div>{/if}
          </button>
          <button onclick={() => hunterType = 'gunner'} class="group relative p-3 rounded-xl border-2 text-left transition-all {hunterType==='gunner' ? 'border-[var(--theme-primary)] bg-[var(--theme-primary)]/10' : 'border-gray-800 bg-gray-900/50 hover:border-gray-700'}">
            <div class="text-lg">🏹</div>
            <div class="text-sm font-medium mt-1 {hunterType==='gunner' ? 'text-white' : 'text-gray-300'}">Gunner</div>
            <div class="text-xs text-gray-500">Bowguns & bows</div>
            {#if hunterType==='gunner'}<div class="absolute top-2 right-2 w-2 h-2 rounded-full bg-[var(--theme-primary)]"></div>{/if}
          </button>
        </div>
      </div>

      <!-- Step 2: Skills -->
      <div class="themed-card rounded-xl border p-4">
        <div class="flex items-center justify-between mb-3">
          <div class="flex items-center gap-2">
            <span class="w-6 h-6 rounded-full bg-[var(--theme-primary)] text-white flex items-center justify-center text-xs font-bold">2</span>
            <h2 class="text-sm font-semibold text-gray-200">Skills</h2>
            <span class="text-xs px-2 py-0.5 rounded-full bg-gray-800 text-gray-400">{activeCount}/5</span>
          </div>
          {#if activeCount>0}<button onclick={() => skillSlots.forEach((_,i)=>clearSkill(i))} class="text-xs text-gray-500 hover:text-gray-300">Clear all</button>{/if}
        </div>
        <p class="text-xs text-gray-500 mb-3">Choose up to 5 skills. We’ll find sets that activate them.</p>
        <div class="space-y-2">
          {#each skillSlots as slot, idx}
            <div class="rounded-xl border-2 bg-gray-950 p-2.5 transition-colors {slot.skillId ? 'border-[var(--theme-primary)]/40 bg-[var(--theme-primary)]/5' : 'border-dashed border-gray-800 hover:border-gray-700'}">
              {#if slot.skillId}
                <div class="flex items-start gap-2">
                  <div class="w-8 h-8 rounded-lg bg-[var(--theme-primary)]/15 flex items-center justify-center text-sm shrink-0">✨</div>
                  <div class="flex-1 min-w-0">
                    <div class="text-sm font-medium text-gray-100 truncate">{slot.skillName}</div>
                    <select bind:value={slot.points} class="mt-1 w-full bg-gray-900 border border-gray-800 rounded-lg px-2 py-1 text-xs text-gray-300">
                      {#each getAbilityOptions(slot.skillId) as lvl}
                        <option value={lvl.points}>{lvl.ability_name} ({lvl.points >0 ? '+' : ''}{lvl.points})</option>
                      {/each}
                      {#if getAbilityOptions(slot.skillId).length===0}
                        <option value={slot.points}>+{slot.points} pts</option>
                      {/if}
                    </select>
                  </div>
                  <button onclick={() => clearSkill(idx)} class="w-7 h-7 rounded-lg bg-gray-900 hover:bg-gray-800 flex items-center justify-center text-gray-500 hover:text-gray-300 shrink-0">×</button>
                </div>
              {:else}
                <div class="relative">
                  <input
                    placeholder="Add a skill — try “Attack” or “Earplug”"
                    class="w-full bg-gray-900 border border-gray-800 rounded-lg pl-9 pr-3 py-2.5 text-sm text-gray-200 placeholder-gray-600 focus:outline-none focus:border-[var(--theme-primary)]/50"
                    bind:value={slot.filter}
                    onfocus={() => slot.open = true}
                    oninput={() => slot.open = true}
                  />
                  <span class="absolute left-3 top-2.5 text-gray-600 text-sm">🔍</span>
                  {#if slot.open}
                    <div class="absolute z-10 mt-1 w-full max-h-48 overflow-auto bg-gray-900 border border-gray-800 rounded-xl shadow-xl">
                      {#each filteredSkills(slot.filter) as s}
                        <button class="w-full text-left px-3 py-2.5 text-sm hover:bg-gray-800 text-gray-300 flex items-center justify-between" onclick={() => selectSkill(idx, s)}>
                          <span>{s.name}</span>
                          <span class="text-xs text-gray-600">+{skillLevelsMap.get(s.id)?.filter(l=>l.points>0)[0]?.points ?? 10}</span>
                        </button>
                      {/each}
                      {#if filteredSkills(slot.filter).length===0}
                        <div class="px-3 py-4 text-xs text-gray-500 text-center">No skills found</div>
                      {/if}
                    </div>
                  {/if}
                </div>
              {/if}
            </div>
          {/each}
        </div>
        <div class="mt-3 flex flex-wrap gap-1.5">
          <span class="text-xs text-gray-600 py-1">Try:</span>
          <button onclick={() => applyExample([{name:'Attack', points:20},{name:'Sharpness', points:10}])} class="text-xs px-2.5 py-1 rounded-full bg-gray-800 hover:bg-gray-700 text-gray-300">Attack + Sharpness</button>
          <button onclick={() => applyExample([{name:'HearProtct', points:10},{name:'WindPress', points:10}])} class="text-xs px-2.5 py-1 rounded-full bg-gray-800 hover:bg-gray-700 text-gray-300">Earplug + Wind</button>
          <button onclick={() => applyExample([{name:'Expert', points:20}])} class="text-xs px-2.5 py-1 rounded-full bg-gray-800 hover:bg-gray-700 text-gray-300">Critical</button>
        </div>
      </div>

      <!-- Step 3: Filters -->
      <div class="themed-card rounded-xl border p-4">
        <div class="flex items-center gap-2 mb-3">
          <span class="w-6 h-6 rounded-full bg-gray-800 text-gray-400 flex items-center justify-center text-xs font-bold">3</span>
          <h2 class="text-sm font-semibold text-gray-200">Hunter Details</h2>
          <span class="text-xs text-gray-600 ml-auto">Optional</span>
        </div>
        <div class="grid grid-cols-3 gap-2">
          <label class="text-xs text-gray-500">HR
            <select bind:value={hr} class="mt-1 w-full bg-gray-900 border border-gray-800 rounded-lg px-2 py-2 text-sm text-gray-200">
              {#each [1,2,3,4,5,6,7,8,9] as v}<option value={v}>{v}</option>{/each}
            </select>
          </label>
          <label class="text-xs text-gray-500">Elder ★
            <select bind:value={elderStar} class="mt-1 w-full bg-gray-900 border border-gray-800 rounded-lg px-2 py-2 text-sm text-gray-200">
              {#each [1,2,3,4,5,6,7,8,9] as v}<option value={v}>{v}</option>{/each}
            </select>
          </label>
          <label class="text-xs text-gray-500">Weapon Slots
            <select bind:value={weaponSlots} class="mt-1 w-full bg-gray-900 border border-gray-800 rounded-lg px-2 py-2 text-sm text-gray-200">
              <option value={0}>0 — none</option>
              <option value={1}>1 — O--</option>
              <option value={2}>2 — OO-</option>
              <option value={3}>3 — OOO</option>
            </select>
          </label>
        </div>
        <div class="mt-3 flex gap-2">
          <button onclick={() => gender='male'} class="flex-1 py-2 rounded-lg border text-xs font-medium {gender==='male' ? 'bg-[var(--theme-primary)] text-white border-[var(--theme-primary)]' : 'bg-gray-900 border-gray-800 text-gray-400'}">Male</button>
          <button onclick={() => gender='female'} class="flex-1 py-2 rounded-lg border text-xs font-medium {gender==='female' ? 'bg-[var(--theme-primary)] text-white border-[var(--theme-primary)]' : 'bg-gray-900 border-gray-800 text-gray-400'}">Female</button>
        </div>
        <button onclick={() => showAdvanced = !showAdvanced} class="mt-3 w-full text-xs text-gray-500 hover:text-gray-300 flex items-center justify-center gap-1">
          {showAdvanced ? 'Hide advanced' : 'Show advanced'} <span class="text-[10px]">{showAdvanced ? '▲' : '▼'}</span>
        </button>
        {#if showAdvanced}
          <div class="mt-3 space-y-2 pt-3 border-t border-gray-800">
            <label class="flex items-center justify-between gap-2 text-xs text-gray-300 bg-gray-900 rounded-lg px-3 py-2"><span>Allow negative skills</span><input type="checkbox" bind:checked={allowBad} class="accent-[var(--theme-primary)]" /></label>
            <label class="flex items-center justify-between gap-2 text-xs text-gray-300 bg-gray-900 rounded-lg px-3 py-2"><span>Include piercings</span><input type="checkbox" bind:checked={includePiercings} /></label>
            <label class="flex items-center justify-between gap-2 text-xs text-gray-300 bg-gray-900 rounded-lg px-3 py-2"><span>Include Torso Inc</span><input type="checkbox" bind:checked={allowTorsoInc} /></label>
            <label class="flex items-center justify-between gap-2 text-xs text-gray-500 bg-gray-900 rounded-lg px-3 py-2"><span>Include dummy armors</span><input type="checkbox" bind:checked={includeDummy} /></label>
            <label class="text-xs text-gray-500">Sort by
              <select bind:value={sortBy} class="mt-1 w-full bg-gray-900 border border-gray-800 rounded-lg px-2 py-2 text-xs text-gray-300">
                {#each sortOptions as o}<option value={o.v}>{o.l}</option>{/each}
              </select>
            </label>
          </div>
        {/if}
      </div>

      <button class="w-full py-3 rounded-xl text-sm font-bold flex items-center justify-center gap-2 {searching ? 'bg-gray-800 text-gray-500' : 'bg-[var(--theme-primary)] text-white hover:opacity-90 shadow-lg shadow-[var(--theme-primary)]/20'}" disabled={searching || activeCount===0} onclick={doSearch}>
        {#if searching}<span class="w-4 h-4 border-2 border-gray-500 border-t-transparent rounded-full animate-spin"></span> Searching...{:else}Find Armor Sets →{/if}
      </button>
      <p class="text-xs text-center text-gray-600">{activeCount === 0 ? 'Add at least one skill to search' : `${activeCount} skill${activeCount>1?'s':''} selected — ready to search`}</p>
      {#if error}<p class="text-xs text-center text-red-400 bg-red-950/30 border border-red-900 rounded-lg px-3 py-2">{error}</p>{/if}
    </div>

    <!-- Results -->
    <div class="themed-card rounded-xl border flex flex-col min-h-[540px]">
      <div class="px-5 py-4 border-b border-gray-800 flex items-center justify-between">
        <div>
          <h2 class="text-sm font-semibold text-gray-100">Results</h2>
          <p class="text-xs text-gray-500">{resultsCountText || 'Your matching sets will appear here'}</p>
        </div>
        {#if results.length>0}<span class="text-xs px-2.5 py-1 rounded-full bg-gray-800 text-gray-400">{results.length}</span>{/if}
      </div>
      <div class="flex-1 p-4 space-y-3 overflow-auto max-h-[78vh]">
        {#if results.length === 0 && !searching}
          <div class="py-16 text-center">
            <div class="w-16 h-16 mx-auto rounded-2xl bg-gray-900 border border-gray-800 flex items-center justify-center text-2xl mb-4">🛡️</div>
            <h3 class="text-sm font-semibold text-gray-300">No sets yet</h3>
            <p class="text-xs text-gray-500 mt-1 max-w-sm mx-auto">Choose the skills you need — like Attack, Earplug, or Sharpness — and we’ll find every armor combination that activates them, including the jewels you’ll need to slot in.</p>
            <div class="mt-6 flex flex-wrap justify-center gap-2">
              <button onclick={() => applyExample([{name:'Attack', points:20}])} class="text-xs px-3 py-1.5 rounded-full border border-gray-800 bg-gray-900 text-gray-400 hover:text-gray-200">Try Attack Up (Large)</button>
              <button onclick={() => applyExample([{name:'HearProtct', points:10}])} class="text-xs px-3 py-1.5 rounded-full border border-gray-800 bg-gray-900 text-gray-400 hover:text-gray-200">Try Earplug</button>
            </div>
          </div>
        {/if}
        {#if searching}
          <div class="space-y-3">
            {#each [1,2,3] as _}
              <div class="bg-gray-950 border border-gray-800 rounded-xl p-4 animate-pulse">
                <div class="h-3 bg-gray-800 rounded w-1/3 mb-3"></div>
                <div class="grid grid-cols-2 gap-2"><div class="h-8 bg-gray-900 rounded"></div><div class="h-8 bg-gray-900 rounded"></div><div class="h-8 bg-gray-900 rounded"></div><div class="h-8 bg-gray-900 rounded"></div></div>
              </div>
            {/each}
          </div>
        {/if}
        {#each results as sol, i}
          <div class="bg-gray-950 border border-gray-800 rounded-xl p-4 hover:border-gray-700 transition-colors">
            <div class="flex items-center justify-between mb-3">
              <span class="text-xs font-bold px-2.5 py-1 rounded-full bg-[var(--theme-primary)] text-white">#{i+1}</span>
              <div class="flex items-center gap-1.5 text-[11px]">
                <span class="px-2 py-1 rounded-full bg-gray-900 border border-gray-800 text-gray-400">DEF {sol.defense}</span>
                <span class="px-2 py-1 rounded-full bg-gray-900 border border-gray-800 text-emerald-400">{sol.slots_spare} spare</span>
              </div>
            </div>
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-2">
              {#each sol.armors as a}
                <div class="flex items-center gap-2 bg-gray-900 rounded-xl px-3 py-2.5 border border-gray-800">
                  <span class="w-8 h-8 rounded-lg bg-gray-800 flex items-center justify-center text-xs shrink-0">{a.slot_type==='head' ? '⛑️' : a.slot_type==='chest' ? '🦺' : a.slot_type==='arms' ? '🥊' : a.slot_type==='waist' ? '🪢' : '🥾'}</span>
                  <div class="flex-1 min-w-0">
                    <div class="text-sm font-medium text-gray-100 truncate">{a.name}</div>
                    <div class="text-xs text-gray-500">{a.slot_type} · {a.slots} {a.slots==='1' ? 'slot' : 'slots'}</div>
                  </div>
                </div>
              {/each}
            </div>
            <div class="mt-3 flex flex-wrap gap-1.5 text-xs">
              <span class="px-2 py-1 rounded-full bg-orange-950/40 text-orange-300 border border-orange-900/30">Fire {sol.fire_res}</span>
              <span class="px-2 py-1 rounded-full bg-blue-950/40 text-blue-300 border border-blue-900/30">Water {sol.water_res}</span>
              <span class="px-2 py-1 rounded-full bg-yellow-950/40 text-yellow-300 border border-yellow-900/30">Thunder {sol.thunder_res}</span>
              <span class="px-2 py-1 rounded-full bg-cyan-950/40 text-cyan-300 border border-cyan-900/30">Ice {sol.ice_res}</span>
              <span class="px-2 py-1 rounded-full bg-purple-950/40 text-purple-300 border border-purple-900/30">Dragon {sol.dragon_res}</span>
              <span class="px-2 py-1 rounded-full bg-gray-900 border border-gray-800 text-gray-400">Rarity {sol.rarity}</span>
            </div>
            {#if sol.decorations.length}
              <div class="mt-3">
                <div class="text-xs font-medium text-gray-400 mb-1.5">Jewels needed</div>
                <div class="flex flex-wrap gap-1.5">
                  {#each sol.decorations as d}
                    <span class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full bg-[var(--theme-primary)]/10 border border-[var(--theme-primary)]/20 text-xs text-[var(--theme-primary)]">💎 {d.count}× {d.name}<span class="text-gray-500">({d.skill_name} {d.skill_points >0 ? '+' : ''}{d.skill_points}{#if d.secondary_skill_name} / {d.secondary_skill_name}{/if})</span></span>
                  {/each}
                </div>
              </div>
            {/if}
            {#if sol.extra_skills.length}
              <div class="mt-2 text-xs"><span class="text-gray-500">Bonus skills:</span> <span class="text-emerald-300">{sol.extra_skills.join(', ')}</span></div>
            {/if}
          </div>
        {/each}
      </div>
    </div>
  </div>
</div>
