<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { onMount } from 'svelte';
    import { ChevronLeft, Clock3 } from 'lucide-svelte';
    import { settingsState } from '$lib/state/settings.svelte';

    interface TenseStat { tense: string; total: number; correct: number; }
    let tenseStats = $state<TenseStat[]>([]);
    let activeSeconds = $state(0);
    let isLoading = $state(true);
    let studyTime = $derived(activeSeconds < 3600 ? `${Math.floor(activeSeconds / 60)} min` : `${Math.floor(activeSeconds / 3600)}h ${Math.floor((activeSeconds % 3600) / 60)}m`);

    onMount(async () => {
        if (!settingsState.isLoaded) await new Promise(resolve => setTimeout(resolve, 100));
        try {
            const curriculum = await invoke<{ active_seconds: number }>('get_curriculum', { language: settingsState.targetLanguage });
            activeSeconds = curriculum.active_seconds || 0;
            tenseStats = await invoke<TenseStat[]>('get_all_tense_stats', { language: settingsState.targetLanguage });
        } catch (error) { console.error('Failed to load stats:', error); }
        finally { isLoading = false; }
    });
</script>

<div class="app-page app-page--narrow">
    <header class="page-heading">
        <div class="flex items-center gap-3">
            <a class="icon-button" href="/profile" aria-label="Back to profile" title="Profile"><ChevronLeft size={19} /></a>
            <div><h1 class="page-title">Progress</h1><p class="page-subtitle">{settingsState.targetLanguage}</p></div>
        </div>
    </header>

    {#if isLoading}
        <div class="grid min-h-64 place-items-center"><div class="h-8 w-8 animate-spin rounded-full border-2 border-accent border-t-transparent"></div></div>
    {:else}
        <section class="study-total"><Clock3 size={20} /><div><strong>{studyTime}</strong><span>Study time</span></div></section>

        <section class="mt-8">
            <h2 class="section-title">Conjugation</h2>
            {#if tenseStats.length === 0}
                <p class="empty-copy">No results yet.</p>
            {:else}
                <div class="stat-list">
                    {#each tenseStats as stat}
                        {@const pct = stat.total ? Math.round(stat.correct / stat.total * 100) : 0}
                        <div class="stat-row">
                            <div class="stat-label"><span>{stat.tense}</span><strong>{pct}%</strong></div>
                            <div class="progress-track"><div style={`width:${pct}%`}></div></div>
                            <small>{stat.correct}/{stat.total}</small>
                        </div>
                    {/each}
                </div>
            {/if}
        </section>
    {/if}
</div>

<style>
    .study-total { display:flex; align-items:center; gap:.8rem; padding:1rem 0; border-block:1px solid var(--pv-border); color:var(--pv-accent-strong); }
    .study-total div { display:flex; flex-direction:column; }
    .study-total strong { color:var(--pv-foreground); font-size:1.35rem; }
    .study-total span { color:var(--pv-subtle); font-size:.72rem; text-transform:uppercase; letter-spacing:.08em; }
    .section-title { margin:0 0 .9rem; color:var(--pv-foreground); font-size:.92rem; font-weight:800; }
    .empty-copy { color:var(--pv-subtle); font-size:.86rem; }
    .stat-list { display:flex; flex-direction:column; }
    .stat-row { padding:.85rem 0; border-bottom:1px solid var(--pv-border); }
    .stat-label { display:flex; justify-content:space-between; gap:1rem; color:var(--pv-foreground); font-size:.85rem; text-transform:capitalize; }
    .stat-label strong { font-size:.8rem; }
    .progress-track { height:4px; margin:.55rem 0 .3rem; overflow:hidden; border-radius:999px; background:var(--pv-surface-raised); }
    .progress-track div { height:100%; border-radius:inherit; background:var(--pv-accent-strong); }
    .stat-row small { color:var(--pv-subtle); font-size:.68rem; }
</style>
