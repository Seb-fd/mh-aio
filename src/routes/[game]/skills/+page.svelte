<script lang="ts">
  import { goto } from '$app/navigation'
  import { selectedGame } from '$lib/stores/game'
  import { api, type Skill } from '$lib/api'
  import Card from '$lib/components/ui/card.svelte'

  const game = $derived($selectedGame)
  const dbId = $derived(game?.dbId)

  let skills = $state<Skill[]>([])
  let loading = $state(true)
  let error = $state<string | null>(null)

  async function loadSkillsData(id: number, attempt = 0) {
    try {
      const data = await api.getSkills(id)
      skills = data
      error = null
    } catch (e) {
      const msg = String(e)
      if (msg.includes('state not managed') && attempt < 6) {
        error = 'Preparing database...'
        setTimeout(() => loadSkillsData(id, attempt + 1), 400 * (attempt + 1))
        return
      }
      error = msg
    } finally {
      if (error !== 'Preparing database...') loading = false
    }
  }
  $effect(() => {
    if (dbId == null) return
    loading = true
    error = null
    loadSkillsData(dbId)
  })

  function open(id: number) {
    if (!game) return
    goto(`/${game.id}/skills/${id}`)
  }
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-6">
    <h1 class="text-2xl font-bold text-gray-100">Skills</h1>
    <p class="text-sm text-gray-500 mt-1">
      {#if game}
        {game.shortName} · {skills.length} skills · Effects per level and synergies
      {:else}
        Select a game first
      {/if}
    </p>
  </div>

  {#if loading}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">Loading skills...</p>
    </div>
  {:else if error}
    <div class="bg-red-950/30 border border-red-900 rounded-lg p-8 text-center">
      <p class="text-red-400">Failed to load skills</p>
      <p class="text-gray-500 text-sm mt-2">{error}</p>
    </div>
  {:else if skills.length === 0}
    <div class="border rounded-lg p-8 text-center themed-card">
      <p class="text-gray-400">No skills found for {game?.shortName ?? 'this game'}</p>
    </div>
  {:else}
    <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
      {#each skills as skill}
        <button onclick={() => open(skill.id)} class="text-left">
          <Card class="p-4 border transition-all cursor-pointer themed-card">
            <div class="flex items-start justify-between gap-2 mb-1">
              <h3 class="font-semibold text-gray-100">{skill.name}</h3>
              {#if skill.max_level}
                <span
                  class="text-[10px] uppercase tracking-wide text-gray-500 bg-[var(--theme-bg-elevated)] px-2 py-0.5 rounded shrink-0 border border-[var(--theme-border)]"
                >
                  Lv 1-{skill.max_level}
                </span>
              {/if}
            </div>
            {#if skill.description}
              <p class="text-xs text-gray-500 mt-2">{skill.description}</p>
            {/if}
          </Card>
        </button>
      {/each}
    </div>
  {/if}
</div>
