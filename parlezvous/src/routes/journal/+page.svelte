<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { listen } from '@tauri-apps/api/event';
    import { onMount, onDestroy } from 'svelte';
    import { settingsState } from '$lib/state/settings.svelte.ts';
    import toast from 'svelte-french-toast';
    import { playSmartTTS } from '$lib/tts';
    import { marked } from 'marked';
    import { Sparkles, PencilLine, Volume2, Plus, LoaderCircle } from 'lucide-svelte';
    import { timeTracker } from '$lib/state/timeTracker.svelte.ts';

    let selectedMoods = $state(['Happy']);
    let selectedWeathers = $state(['Sunny']);
    let selectedActivities = $state(['Studying']);
    
    let isCustomMode = $state(false);
    let customEntry = $state('');
    
    let isGenerating = $state(false);

    function toggleSelection(currentSelection: string[], item: string) {
        if (currentSelection.includes(item)) {
            return currentSelection.filter(i => i !== item);
        } else {
            return [...currentSelection, item];
        }
    }

    const moods = ['Happy', 'Tired', 'Excited', 'Sad', 'Calm'];
    const weathers = ['Sunny', 'Raining', 'Cloudy', 'Snowing', 'Windy'];
    const activities = ['Studying', 'Working', 'Relaxing', 'Traveling', 'Exercising'];

    let journalResult = $state<{ generated_target_text: string; native_translation: string; feedback?: string } | null>(null);
    let vocabChips = $state<any[]>([]);
    let unlisten: () => void;
    
    let activeThemeId = $state<string | null>(null);

    onMount(async () => {
        unlisten = await listen('vocabulary_extracted', (event) => {
            vocabChips = event.payload as any[];
        });

        try {
            const curr: any = await invoke('get_curriculum', { language: settingsState.targetLanguage });
            activeThemeId = curr.active_theme_id;
        } catch (e) {
            console.error("No curriculum found", e);
        }

        timeTracker.startTracking();
    });

    onDestroy(() => {
        timeTracker.flushTime();
        timeTracker.stopTracking();
        if (unlisten) unlisten();
    });

    async function generateJournal() {
        if (!settingsState.activeModel) {
            toast.error('Please select an AI model in settings (or wait for it to load).');
            return;
        }
        
        isGenerating = true;
        journalResult = null;
        vocabChips = [];

        try {
            const payload = {
                mood: selectedMoods.length ? selectedMoods.join(', ') : 'Neutral',
                weather: selectedWeathers.length ? selectedWeathers.join(', ') : 'Neutral',
                activity: selectedActivities.length ? selectedActivities.join(', ') : 'Neutral',
                model: settingsState.activeModel,
                language: settingsState.targetLanguage,
                activeTheme: activeThemeId
            };

            const result = await invoke('generate_journal', payload);
            journalResult = result as any;
        } catch (e) {
            console.error('Failed to generate journal:', e);
            toast.error('Failed to generate journal: ' + e);
        } finally {
            isGenerating = false;
        }
    }

    async function gradeJournal() {
        if (!settingsState.activeModel) {
            toast.error('Please select an AI model in settings (or wait for it to load).');
            return;
        }
        
        isGenerating = true;
        journalResult = null;
        vocabChips = [];

        try {
            const payload = {
                entry: customEntry,
                model: settingsState.activeModel,
                language: settingsState.targetLanguage,
            };

            const result = await invoke('grade_journal', payload);
            journalResult = result as any;
        } catch (e) {
            console.error('Failed to grade journal:', e);
            toast.error('Failed to grade journal: ' + e);
        } finally {
            isGenerating = false;
        }
    }

    async function addToSrs(vocabId: number) {
        try {
            await invoke('add_to_srs', { vocabId });
            // Remove the chip from UI to indicate success
            vocabChips = vocabChips.filter(chip => chip.id !== vocabId);
            toast.success('Added to SRS!');
        } catch (e) {
            console.error('Failed to add to SRS:', e);
            toast.error('Failed to add to SRS: ' + e);
        }
    }
</script>

<div class="app-page">
    <header class="page-heading">
        <div><h1 class="page-title">Journal</h1><p class="page-subtitle">{settingsState.targetLanguage} · {settingsState.skillLevel}</p></div>
        <div class="mode-switch" aria-label="Journal mode">
            <button class:active={!isCustomMode} onclick={() => isCustomMode = false} title="Prompted journal" aria-label="Prompted journal"><Sparkles size={18} /></button>
            <button class:active={isCustomMode} onclick={() => isCustomMode = true} title="Free write" aria-label="Free write"><PencilLine size={18} /></button>
        </div>
    </header>

    <div class="journal-layout">
        <section class="journal-input">
            {#if isCustomMode}
                <textarea bind:value={customEntry} placeholder={`Write in ${settingsState.targetLanguage}…`} aria-label="Journal entry"></textarea>
            {:else}
                <div class="prompt-group"><span>Mood</span><div>{#each moods as item}<button class:active={selectedMoods.includes(item)} onclick={() => selectedMoods = toggleSelection(selectedMoods, item)}>{item}</button>{/each}</div></div>
                <div class="prompt-group"><span>Weather</span><div>{#each weathers as item}<button class:active={selectedWeathers.includes(item)} onclick={() => selectedWeathers = toggleSelection(selectedWeathers, item)}>{item}</button>{/each}</div></div>
                <div class="prompt-group"><span>Activity</span><div>{#each activities as item}<button class:active={selectedActivities.includes(item)} onclick={() => selectedActivities = toggleSelection(selectedActivities, item)}>{item}</button>{/each}</div></div>
            {/if}

            <button class="primary-action" onclick={isCustomMode ? gradeJournal : generateJournal} disabled={isGenerating || (isCustomMode && !customEntry.trim())}>
                {#if isGenerating}<LoaderCircle size={17} class="animate-spin" />{:else}<Sparkles size={17} />{/if}
                <span>{isGenerating ? 'Working…' : isCustomMode ? 'Review' : 'Generate'}</span>
            </button>
        </section>

        <section class="journal-output" aria-live="polite">
            {#if isGenerating}
                <div class="output-empty"><LoaderCircle size={24} class="animate-spin" /></div>
            {:else if journalResult}
                <article class="entry-result">
                    <button class="icon-button listen" onclick={() => playSmartTTS(journalResult!.generated_target_text, settingsState.ttsServerUrl, undefined, settingsState.targetLanguage, settingsState.ttsProvider)} title="Listen" aria-label="Listen"><Volume2 size={18} /></button>
                    <p class="target-entry">{journalResult.generated_target_text}</p>
                    <p class="translation">{journalResult.native_translation}</p>
                    {#if isCustomMode && journalResult.feedback}<div class="feedback">{@html marked.parse(journalResult.feedback)}</div>{/if}
                </article>

                {#if vocabChips.length}
                    <div class="vocab-list">
                        {#each vocabChips as chip}
                            <div class="vocab-chip"><button onclick={() => addToSrs(chip.id)} title="Save word" aria-label={`Save ${chip.target_text}`}><Plus size={15} /></button><strong>{chip.target_text}</strong><span>{chip.native_text}</span><button onclick={() => playSmartTTS(chip.target_text, settingsState.ttsServerUrl, undefined, settingsState.targetLanguage, settingsState.ttsProvider)} title="Listen" aria-label={`Listen to ${chip.target_text}`}><Volume2 size={15} /></button></div>
                        {/each}
                    </div>
                {/if}
            {:else}
                <div class="output-empty"><Sparkles size={24} /></div>
            {/if}
        </section>
    </div>
</div>

<style>
    .mode-switch { display:flex; gap:.25rem; padding:.25rem; border:1px solid var(--pv-border); border-radius:.85rem; background:var(--pv-surface); }
    .mode-switch button { display:grid; width:2.4rem; height:2.4rem; place-items:center; border:0; border-radius:.65rem; background:transparent; color:var(--pv-subtle); cursor:pointer; }
    .mode-switch button.active { background:var(--pv-accent-soft); color:var(--pv-accent-strong); }
    .journal-layout { display:grid; gap:1.25rem; }
    .journal-input { min-width:0; }
    .journal-input textarea { width:100%; min-height:16rem; box-sizing:border-box; resize:vertical; border:1px solid var(--pv-border); border-radius:1rem; background:var(--pv-surface); color:var(--pv-foreground); padding:1rem; font:inherit; line-height:1.6; outline:none; }
    .journal-input textarea:focus { border-color:var(--pv-accent); }
    .prompt-group + .prompt-group { margin-top:1rem; }
    .prompt-group > span { display:block; margin-bottom:.45rem; color:var(--pv-subtle); font-size:.68rem; font-weight:800; letter-spacing:.08em; text-transform:uppercase; }
    .prompt-group > div { display:flex; flex-wrap:wrap; gap:.4rem; }
    .prompt-group button { border:1px solid var(--pv-border); border-radius:999px; background:transparent; color:var(--pv-muted); padding:.45rem .7rem; font-size:.78rem; font-weight:650; cursor:pointer; }
    .prompt-group button.active { border-color:transparent; background:var(--pv-accent-soft); color:var(--pv-accent-strong); }
    .primary-action { display:flex; width:100%; min-height:2.85rem; align-items:center; justify-content:center; gap:.45rem; margin-top:1.25rem; border:0; border-radius:.85rem; background:var(--pv-accent); color:var(--pv-on-accent); font-weight:800; cursor:pointer; }
    .primary-action:disabled { opacity:.45; cursor:not-allowed; }
    .journal-output { min-width:0; border-top:1px solid var(--pv-border); padding-top:1.25rem; }
    .output-empty { display:grid; min-height:12rem; place-items:center; color:var(--pv-subtle); }
    .entry-result { position:relative; }
    .listen { position:absolute; top:0; right:0; }
    .target-entry { margin:0; padding-right:3rem; color:var(--pv-foreground); font-size:1.08rem; line-height:1.75; }
    .translation { margin:1rem 0 0; color:var(--pv-muted); font-size:.88rem; font-style:italic; line-height:1.6; }
    .feedback { margin-top:1.2rem; border-left:2px solid var(--pv-info); padding-left:1rem; color:var(--pv-muted); font-size:.86rem; line-height:1.6; }
    .vocab-list { display:flex; flex-wrap:wrap; gap:.45rem; margin-top:1.3rem; }
    .vocab-chip { display:flex; align-items:center; gap:.45rem; border:1px solid var(--pv-border); border-radius:999px; padding:.25rem .35rem; background:var(--pv-surface); }
    .vocab-chip button { display:grid; width:1.75rem; height:1.75rem; place-items:center; border:0; border-radius:999px; background:transparent; color:var(--pv-subtle); cursor:pointer; }
    .vocab-chip button:hover { color:var(--pv-accent-strong); background:var(--pv-surface-raised); }
    .vocab-chip strong { color:var(--pv-foreground); font-size:.78rem; }
    .vocab-chip span { color:var(--pv-subtle); font-size:.75rem; }
    @media (min-width:800px) { .journal-layout { grid-template-columns:minmax(16rem, .8fr) minmax(0, 1.4fr); gap:2rem; } .journal-output { border-top:0; border-left:1px solid var(--pv-border); padding-top:0; padding-left:2rem; } }
</style>
