<script lang="ts">
    import { onDestroy, onMount } from 'svelte';
    import AiGate from '$lib/components/AiGate.svelte';
    import { Plus } from 'lucide-svelte';
    import { settingsState } from '$lib/state/settings.svelte';
    import { timeTracker } from '$lib/state/timeTracker.svelte';
    import { ConjugationController } from '$lib/conjugation/controller.svelte';
    import ExerciseCard from '$lib/conjugation/components/ExerciseCard.svelte';
    import HistorySidebar from '$lib/conjugation/components/HistorySidebar.svelte';

    const controller = new ConjugationController();
    onMount(() => timeTracker.startTracking());
    onDestroy(() => { controller.dispose(); timeTracker.flushTime(); timeTracker.stopTracking(); });
</script>

<AiGate feature="Conjugation">
<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div bind:this={controller.pageEl} role="application" tabindex="0" onkeydown={controller.handlePageKeydown} class="app-page app-page--medium">
    <header class="page-heading">
        <div><h1 class="page-title">Conjugation</h1><p class="page-subtitle">{settingsState.targetLanguage}</p></div>
        <div class="flex items-center gap-2">
            {#if controller.history.length}<span class="score-pill">{controller.recentScore}/10</span>{/if}
            <button class="icon-button active" onclick={() => controller.generate()} disabled={controller.isGenerating} title="New exercise" aria-label="New exercise"><Plus size={19} /></button>
        </div>
    </header>

    <div class="practice-layout">
        <ExerciseCard {controller} />
        <HistorySidebar {controller} />
    </div>
</div>

<style>
    .score-pill { border-radius:999px; background:var(--pv-surface-raised); padding:.45rem .65rem; color:var(--pv-muted); font-size:.76rem; font-weight:800; }
    .practice-layout { display:grid; gap:1.5rem; }
    @media (min-width:900px) { .practice-layout { grid-template-columns:minmax(0,1fr) 15rem; gap:2rem; align-items:start; } }
</style>
</AiGate>
