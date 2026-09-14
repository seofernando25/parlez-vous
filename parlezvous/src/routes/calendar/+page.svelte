<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { onMount } from 'svelte';
    import { ChevronLeft, Volume2, X } from 'lucide-svelte';
    import { playSmartTTS } from '$lib/tts';
    import { settingsState } from '$lib/state/settings.svelte';

    interface JournalEntry {
        id: number;
        language_code: string;
        date: string;
        mood_input: string;
        weather_input: string;
        activity_input: string;
        generated_target_text: string;
        native_translation: string;
    }

    let entries = $state<JournalEntry[]>([]);
    let isLoading = $state(true);
    let selected = $state<JournalEntry | null>(null);

    onMount(async () => {
        try { entries = await invoke<JournalEntry[]>('get_journal_entries'); }
        catch (error) { console.error('Failed to load journal entries:', error); }
        finally { isLoading = false; }
    });

    function dateLabel(date: string) {
        const parsed = new Date(date);
        if (Number.isNaN(parsed.getTime())) return date;
        return new Intl.DateTimeFormat(undefined, { month: 'short', day: 'numeric', year: 'numeric' }).format(parsed);
    }
</script>

<div class="app-page app-page--narrow">
    <header class="page-heading">
        <div class="flex items-center gap-3"><a class="icon-button" href="/profile" aria-label="Back to profile" title="Profile"><ChevronLeft size={19} /></a><h1 class="page-title">Journal history</h1></div>
    </header>

    {#if isLoading}
        <div class="grid min-h-64 place-items-center"><div class="h-8 w-8 animate-spin rounded-full border-2 border-accent border-t-transparent"></div></div>
    {:else if entries.length === 0}
        <div class="history-empty">No entries yet.</div>
    {:else}
        <div class="history-list">
            {#each entries as entry}
                <button onclick={() => selected = entry}>
                    <div><strong>{dateLabel(entry.date)}</strong><span>{entry.generated_target_text}</span></div>
                    <small>{entry.language_code}</small>
                </button>
            {/each}
        </div>
    {/if}

    {#if selected}
        <div class="entry-overlay">
            <button class="entry-backdrop" onclick={() => selected = null} aria-label="Close journal entry"></button>
            <article class="entry-detail">
                <div class="entry-actions">
                    <button class="icon-button" onclick={() => playSmartTTS(selected!.generated_target_text, settingsState.ttsServerUrl, undefined, settingsState.targetLanguage, settingsState.ttsProvider)} title="Listen" aria-label="Listen"><Volume2 size={17} /></button>
                    <button class="icon-button" onclick={() => selected = null} title="Close" aria-label="Close"><X size={17} /></button>
                </div>
                <time>{dateLabel(selected.date)}</time>
                <p class="entry-target">{selected.generated_target_text}</p>
                <p class="entry-translation">{selected.native_translation}</p>
            </article>
        </div>
    {/if}
</div>

<style>
    .history-empty { min-height:18rem; display:grid; place-items:center; color:var(--pv-subtle); font-size:.84rem; }
    .history-list { border-top:1px solid var(--pv-border); }
    .history-list > button { display:grid; width:100%; grid-template-columns:minmax(0,1fr) auto; gap:1rem; align-items:center; min-height:4.8rem; border:0; border-bottom:1px solid var(--pv-border); background:transparent; padding:.8rem .15rem; text-align:left; cursor:pointer; }
    .history-list > button:hover { background:color-mix(in oklch,var(--pv-surface-raised) 45%,transparent); }
    .history-list div { min-width:0; }
    .history-list strong,.history-list span { display:block; }
    .history-list strong { color:var(--pv-foreground); font-size:.84rem; }
    .history-list span { margin-top:.3rem; overflow:hidden; color:var(--pv-muted); font-size:.8rem; text-overflow:ellipsis; white-space:nowrap; }
    .history-list small { color:var(--pv-subtle); font-size:.68rem; font-weight:750; text-transform:uppercase; }
    .entry-overlay { position:fixed; inset:0; z-index:70; display:grid; place-items:center; padding:1rem; }
    .entry-backdrop { position:absolute; inset:0; border:0; background:color-mix(in oklch,var(--pv-canvas) 65%,transparent); backdrop-filter:blur(10px); cursor:default; }
    .entry-detail { position:relative; z-index:1; width:min(100%,38rem); max-height:85dvh; overflow:auto; border:1px solid var(--pv-border); border-radius:1rem; background:var(--pv-surface); padding:1.5rem; box-shadow:var(--pv-shadow); }
    .entry-actions { position:absolute; top:.75rem; right:.75rem; display:flex; gap:.35rem; }
    .entry-detail time { color:var(--pv-subtle); font-size:.72rem; font-weight:750; }
    .entry-target { margin:2rem 0 0; color:var(--pv-foreground); font-size:1.05rem; line-height:1.75; white-space:pre-wrap; }
    .entry-translation { margin:1.2rem 0 0; color:var(--pv-muted); font-size:.88rem; font-style:italic; line-height:1.6; white-space:pre-wrap; }
</style>
