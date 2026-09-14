<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { onMount } from 'svelte';
    import { BookOpen, Volume2 } from 'lucide-svelte';
    import { playSmartTTS } from '$lib/tts';
    import { settingsState } from '$lib/state/settings.svelte';

    interface VocabItem { id: number; target_text: string; native_text: string; is_character: boolean; }
    let vocabulary = $state<VocabItem[]>([]);
    let isLoading = $state(true);

    onMount(async () => {
        try { vocabulary = await invoke<VocabItem[]>('get_all_vocabulary'); }
        catch (error) { console.error('Failed to load vocabulary:', error); }
        finally { isLoading = false; }
    });
</script>

<div class="app-page app-page--medium">
    <header class="page-heading">
        <div><h1 class="page-title">Vocabulary</h1><p class="page-subtitle">{settingsState.targetLanguage}</p></div>
        {#if !isLoading}<span class="count-pill">{vocabulary.length}</span>{/if}
    </header>

    {#if isLoading}
        <div class="grid min-h-64 place-items-center"><div class="h-8 w-8 animate-spin rounded-full border-2 border-accent border-t-transparent"></div></div>
    {:else if vocabulary.length === 0}
        <div class="empty-state"><BookOpen size={28} /><p>No saved words yet.</p><a href="/journal">Write something</a></div>
    {:else}
        <div class="vocab-grid">
            {#each vocabulary as vocab}
                <article class="vocab-card">
                    <button class="icon-button speak" onclick={() => playSmartTTS(vocab.target_text, settingsState.ttsServerUrl, undefined, settingsState.targetLanguage, settingsState.ttsProvider)} title="Listen" aria-label={`Listen to ${vocab.target_text}`}><Volume2 size={17} /></button>
                    <strong>{vocab.target_text}</strong>
                    <span>{vocab.native_text}</span>
                </article>
            {/each}
        </div>
    {/if}
</div>

<style>
    .count-pill { min-width:2rem; border-radius:999px; background:var(--pv-surface-raised); padding:.35rem .6rem; color:var(--pv-muted); text-align:center; font-size:.75rem; font-weight:800; }
    .vocab-grid { display:grid; grid-template-columns:repeat(auto-fill,minmax(10rem,1fr)); gap:.7rem; }
    .vocab-card { position:relative; display:flex; min-height:8rem; flex-direction:column; justify-content:center; gap:.35rem; border:1px solid var(--pv-border); border-radius:1rem; background:var(--pv-surface); padding:1rem; text-align:center; }
    .vocab-card strong { color:var(--pv-foreground); font-size:1.3rem; overflow-wrap:anywhere; }
    .vocab-card > span { color:var(--pv-muted); font-size:.85rem; overflow-wrap:anywhere; }
    .speak { position:absolute; top:.5rem; right:.5rem; width:2.1rem; height:2.1rem; border:0; }
    .empty-state { display:flex; min-height:18rem; flex-direction:column; align-items:center; justify-content:center; gap:.65rem; color:var(--pv-subtle); text-align:center; }
    .empty-state p { margin:0; color:var(--pv-muted); }
    .empty-state a { color:var(--pv-accent-strong); font-size:.82rem; font-weight:700; }
</style>
