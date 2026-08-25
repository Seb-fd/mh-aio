<script lang="ts">
  import { selectedGame } from '$lib/stores/game';
  import { api, type Skill, type SkillLevel } from '$lib/api';
  import { onMount } from 'svelte';

  const game = $derived($selectedGame);

  // ASS parity controls - Form1.h:539 etc
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

  // 5 skill slots like ASS NumSkills=5
  type SkillSlot = { skillId: number | null; skillName: string; points: number; filter: string; };
  let skillSlots = $state<SkillSlot[]>([
    { skillId: null, skillName: '', points: 10, filter: '' },
    { skillId: null, skillName: '', points: 10, filter: '' },
    { skillId: null, skillName: '', points: 10, filter: '' },
    { skillId: null, skillName: '', points: 10, filter: '' },
    { skillId: null, skillName: '', points: 10, filter: '' },
  ]);

  let allSkills = $state<Skill[]>([]);
  let skillLevelsMap = $state<Map<number, SkillLevel[]>>(new Map());
  let loadingSkills = $state(false);
  let searching = $state(false);
  let error = $state<string | null>(null);
  let results = $state<any[]>([]);
  let resultsCountText = $state('');

  async function loadSkills() {
    if (!game) return;
    loadingSkills = true;
    try {
      const dbId = game.dbId;
      const skills = await api.getSkills(dbId);
      allSkills = skills;
      // Preload levels for each skill (for points helper)
      const m = new Map<number, SkillLevel[]>();
      await Promise.all(skills.slice(0, 60).map(async (s) => {
        try {
          const detail = await api.getSkillDetail(s.id);
          if (detail) m.set(s.id, detail.levels);
        } catch {}
      }));
      skillLevelsMap = m;
    } catch (e: any) {
      console.error(e);
    } finally { loadingSkills = false; }
  }

  $effect(() => { if (game) loadSkills(); });

  function filteredSkills(filter: string) {
    const f = filter.toLowerCase();
    if (!f) return allSkills.slice(0, 60);
    return allSkills.filter(s => s.name.toLowerCase().includes(f)).slice(0, 60);
  }

  function selectSkill(idx: number, skill: Skill) {
    skillSlots[idx].skillId = skill.id;
    skillSlots[idx].skillName = skill.name;
    skillSlots[idx].filter = skill.name;
    const levels = skillLevelsMap.get(skill.id);
    if (levels && levels.length) {
      // default to smallest positive threshold (like ASS: first >0 entry)
      const pos = [...levels].filter(l => l.points > 0).sort((a,b)=>a.points-b.points)[0];
      if (pos) skillSlots[idx].points = pos.points;
    }
  }

  function clearSkill(idx: number) {
    skillSlots[idx] = { skillId: null, skillName: '', points: 10, filter: '' };
  }

  async function doSearch() {
    if (!game) return;
    const reqSkills = skillSlots.filter(s => s.skillId !== null).map(s => ({ skill_id: s.skillId!, points_required: s.points }));
    if (reqSkills.length === 0) { error = 'Selecciona al menos 1 habilidad (como en ASS Quick Search)'; return; }
    if (reqSkills.length > 5) { error = 'Máximo 5 habilidades (límite ASS)'; return; }
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
      if (res.length >= 1000) resultsCountText = `Mostrando primeros 1000 soluciones (límite ASS)`;
      else resultsCountText = `Soluciones encontradas: ${res.length}`;
    } catch (e: any) {
      error = e?.toString() ?? 'Error en búsqueda';
    } finally { searching = false; }
  }

  function cancelSearch() { searching = false; }

  const sortOptions = [
    { v: 'none', l: 'None' },
    { v: 'dragon_res', l: 'Dragon Res' },
    { v: 'fire_res', l: 'Fire Res' },
    { v: 'ice_res', l: 'Ice Res' },
    { v: 'thunder_res', l: 'Thunder Res' },
    { v: 'water_res', l: 'Water Res' },
    { v: 'defence', l: 'Base Defence' },
    { v: 'difficulty', l: 'Difficulty' },
    { v: 'rarity', l: 'Rarity' },
    { v: 'slots_spare', l: 'Slots Spare' },
  ];
</script>

<div class="max-w-7xl mx-auto">
  <div class="mb-4">
    <h1 class="text-2xl font-bold text-gray-100">Builds · Armor Set Search</h1>
    <p class="text-sm text-gray-500 mt-1">
      {#if game}{game.shortName} · ASS port (paridad total con Athenas ASS — MHFU){:else}Selecciona un juego{/if}
    </p>
  </div>

  <div class="grid grid-cols-1 lg:grid-cols-[300px_1fr] gap-4">
    <!-- Left controls: HR / Elder / Slots etc like Form1.h groupBox1 -->
    <div class="space-y-3">
      <div class="themed-card rounded-lg border p-3">
        <div class="text-xs font-semibold text-gray-400 uppercase tracking-widest mb-2">Filtros de cazador</div>
        <div class="grid grid-cols-2 gap-2">
          <label class="text-xs text-gray-500">HR
            <input type="number" min="1" max="9" bind:value={hr} class="mt-1 w-full bg-gray-950 border border-gray-800 rounded px-2 py-1 text-sm text-gray-200" />
          </label>
          <label class="text-xs text-gray-500">Village Elder
            <input type="number" min="1" max="9" bind:value={elderStar} class="mt-1 w-full bg-gray-950 border border-gray-800 rounded px-2 py-1 text-sm text-gray-200" />
          </label>
          <label class="text-xs text-gray-500 col-span-2">Max Weapon Slots
            <input type="number" min="0" max="3" bind:value={weaponSlots} class="mt-1 w-full bg-gray-950 border border-gray-800 rounded px-2 py-1 text-sm text-gray-200" />
            <span class="text-[10px] text-gray-600">Como ASS: weapon_slots_allowed (0..3) se añade a slots_spare</span>
          </label>
        </div>
        <div class="mt-3 flex gap-2">
          <label class="flex items-center gap-1 text-xs text-gray-300"><input type="radio" name="gender" checked={gender==='male'} onchange={()=>gender='male'} /> Male</label>
          <label class="flex items-center gap-1 text-xs text-gray-300"><input type="radio" name="gender" checked={gender==='female'} onchange={()=>gender='female'} /> Female</label>
        </div>
        <div class="mt-3 space-y-1.5">
          <label class="flex items-center gap-2 text-xs text-gray-300"><input type="checkbox" bind:checked={allowBad} /> Allow Bad Skills <span class="text-gray-600">(ASS chkBadSkills off por defecto)</span></label>
          <label class="flex items-center gap-2 text-xs text-gray-300"><input type="checkbox" bind:checked={includePiercings} /> Allow Piercings</label>
          <label class="flex items-center gap-2 text-xs text-gray-300"><input type="checkbox" bind:checked={allowTorsoInc} /> Allow Torso Inc</label>
          <label class="flex items-center gap-2 text-xs text-gray-500"><input type="checkbox" bind:checked={includeDummy} /> Allow (dummy)</label>
        </div>
        <div class="mt-3">
          <label class="text-xs text-gray-500">Sort By
            <select bind:value={sortBy} class="mt-1 w-full bg-gray-950 border border-gray-800 rounded px-2 py-1 text-sm text-gray-200">
              {#each sortOptions as o}<option value={o.v}>{o.l}</option>{/each}
            </select>
          </label>
        </div>
      </div>

      <!-- Hunter type tabs like tabHunterType -->
      <div class="themed-card rounded-lg border p-3">
        <div class="flex gap-1 mb-2">
          <button class="flex-1 text-xs py-1.5 rounded border {hunterType==='blade' ? 'bg-[var(--theme-primary)] text-white border-[var(--theme-primary)]' : 'border-gray-800 text-gray-400'}" onclick={()=>hunterType='blade'}>Blademaster</button>
          <button class="flex-1 text-xs py-1.5 rounded border {hunterType==='gunner' ? 'bg-[var(--theme-primary)] text-white border-[var(--theme-primary)]' : 'border-gray-800 text-gray-400'}" onclick={()=>hunterType='gunner'}>Gunner</button>
        </div>
        <div class="text-xs font-semibold text-gray-400 uppercase tracking-widest">Skills (máx 5)</div>
        <div class="mt-2 space-y-2">
          {#each skillSlots as slot, idx}
            <div class="bg-gray-950 border border-gray-800 rounded p-2">
              <div class="flex gap-1">
                <input placeholder="(none) — escribe para filtrar" class="flex-1 bg-gray-900 border border-gray-800 rounded px-2 py-1 text-xs text-gray-200" bind:value={slot.filter} oninput={()=>{ if(!slot.filter) clearSkill(idx); }} />
                {#if slot.skillId !== null}<button class="text-xs px-2 py-1 bg-gray-800 rounded text-gray-400" onclick={()=>clearSkill(idx)}>×</button>{/if}
              </div>
              {#if slot.filter && !slot.skillId}
                <div class="mt-1 max-h-28 overflow-auto bg-gray-900 border border-gray-800 rounded">
                  {#each filteredSkills(slot.filter) as s}
                    <button class="w-full text-left px-2 py-1 text-xs hover:bg-gray-800 text-gray-300" onclick={()=>selectSkill(idx, s)}>{s.name}</button>
                  {/each}
                </div>
              {/if}
              {#if slot.skillId !== null}
                <div class="mt-1 flex items-center gap-2">
                  <span class="text-xs text-[var(--theme-primary)]">{slot.skillName}</span>
                  <input type="number" class="ml-auto w-16 bg-gray-900 border border-gray-800 rounded px-1 py-0.5 text-xs text-gray-200" bind:value={slot.points} />
                  {#if skillLevelsMap.get(slot.skillId!) }
                    <span class="text-[10px] text-gray-500">sugerido: {skillLevelsMap.get(slot.skillId!)?.filter(l=>l.points>0).map(l=>l.points).join('/')}</span>
                  {/if}
                </div>
              {/if}
            </div>
          {/each}
        </div>
        <button class="mt-3 w-full py-2 rounded text-sm font-semibold {searching ? 'bg-gray-800 text-gray-500' : 'bg-[var(--theme-primary)] text-white'}" disabled={searching} onclick={doSearch}>
          {#if searching}Buscando...{:else}Quick Search{/if}
        </button>
        {#if searching}<button class="mt-1 w-full py-1 rounded text-xs border border-gray-800 text-gray-400" onclick={cancelSearch}>Cancel</button>{/if}
        {#if error}<p class="mt-2 text-xs text-red-400">{error}</p>{/if}
      </div>
    </div>

    <!-- Results like grpResults txtSolutions -->
    <div class="themed-card rounded-lg border flex flex-col min-h-[500px]">
      <div class="px-4 py-3 border-b border-gray-800 flex items-center justify-between">
        <span class="text-sm font-semibold text-gray-300">Results</span>
        <span class="text-xs text-gray-500">{resultsCountText}</span>
      </div>
      <div class="flex-1 p-3 space-y-3 overflow-auto max-h-[75vh]">
        {#if results.length === 0 && !searching}
          <div class="text-center py-12 text-gray-600 text-sm">Selecciona 1-5 habilidades y pulsa Quick Search. Paridad con ASS: equivalencias, joyas (1/2/3 slots), Torso Inc multiplicador, límite 1000, sort.</div>
        {/if}
        {#each results as sol, i}
          <div class="bg-gray-950 border border-gray-800 rounded p-3">
            <div class="text-xs text-gray-500 mb-1">#{i+1} · Def {sol.defense} · Fire {sol.fire_res} Ice {sol.ice_res} Thunder {sol.thunder_res} Water {sol.water_res} Dragon {sol.dragon_res} · Rare {sol.rarity} · Diff {sol.difficulty} · Slots spare {sol.slots_spare}</div>
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-1 text-xs">
              {#each sol.armors as a}
                <div class="flex justify-between bg-gray-900 rounded px-2 py-1">
                  <span class="text-gray-300">{a.name} <span class="text-gray-600">[{a.slot_type}]</span></span>
                  <span class="text-gray-500">{a.slots} slots</span>
                </div>
              {/each}
            </div>
            {#if sol.decorations.length}
              <div class="mt-2 text-xs">
                <div class="text-gray-500">Joyas:</div>
                {#each sol.decorations as d}
                  <span class="inline-block mr-2 px-1.5 py-0.5 bg-gray-900 border border-gray-800 rounded text-gray-400">{d.count}x {d.name} ({d.skill_name} {d.skill_points}{#if d.secondary_skill_name} / {d.secondary_skill_name} {d.secondary_points}{/if})</span>
                {/each}
              </div>
            {/if}
            {#if sol.extra_skills.length}
              <div class="mt-1 text-xs text-gray-500">Extras: {sol.extra_skills.join(', ')}</div>
            {/if}
            {#if sol.slots_spare >0}<div class="mt-1 text-xs text-emerald-400">{sol.slots_spare} slot{sol.slots_spare===1?'':'s'} spare</div>{/if}
          </div>
        {/each}
      </div>
    </div>
  </div>
</div>
