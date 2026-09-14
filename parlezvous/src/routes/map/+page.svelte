<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { onMount } from 'svelte';
    import { Lock, MapPin, NotebookPen, Languages } from 'lucide-svelte';
    import { settingsState } from '$lib/state/settings.svelte';
    import { CURRICULUM_TIERS, getTierFromXP, XP_THRESHOLDS } from '$lib/curriculum';
    import toast from 'svelte-french-toast';

    let userTier = $state(1);
    let totalXp = $state(0);
    let activeThemeId = $state('greetings');
    let isLoading = $state(true);

    onMount(async () => {
        if (!settingsState.isLoaded) await new Promise(resolve => setTimeout(resolve, 100));
        try {
            const curr = await invoke<{ total_xp: number; active_theme_id: string }>('get_curriculum', { language: settingsState.targetLanguage });
            userTier = getTierFromXP(curr.total_xp);
            totalXp = curr.total_xp;
            activeThemeId = curr.active_theme_id;
        } catch (error) { console.error('Failed to load curriculum', error); }
        finally { isLoading = false; }
    });

    function isLocked(level: number) {
        const skill = settingsState.skillLevel.toLowerCase();
        let max = userTier + 1;
        if (skill === 'intermediate') max = Math.max(max, 3);
        else if (skill === 'advanced') max = Math.max(max, 5);
        else if (skill === 'fluent') return false;
        return level > max;
    }

    async function setActiveTheme(themeId: string) {
        try {
            await invoke('set_active_theme', { language: settingsState.targetLanguage, themeId });
            activeThemeId = themeId;
        } catch { toast.error('Could not change topic'); }
    }

    function progress(level: number) {
        if (level < userTier) return 100;
        if (level > userTier) return 0;
        const current = XP_THRESHOLDS[level as keyof typeof XP_THRESHOLDS] || 0;
        const next = XP_THRESHOLDS[(level + 1) as keyof typeof XP_THRESHOLDS] || current + 100000;
        return Math.min(100, Math.max(0, ((totalXp - current) / (next - current)) * 100));
    }
</script>

<div class="app-page app-page--medium">
    <header class="page-heading">
        <div><h1 class="page-title">Learn</h1><p class="page-subtitle">{settingsState.targetLanguage}</p></div>
        <span class="xp-pill">{totalXp.toLocaleString()} XP</span>
    </header>

    {#if isLoading}
        <div class="grid min-h-64 place-items-center"><div class="h-8 w-8 animate-spin rounded-full border-2 border-accent border-t-transparent"></div></div>
    {:else}
        <div class="learning-path">
            {#each CURRICULUM_TIERS as tier}
                {@const locked = isLocked(tier.level)}
                <section class="tier-row" class:locked>
                    <div class="tier-node {tier.color}">{#if locked}<Lock size={18} />{:else}{tier.level}{/if}</div>
                    <div class="tier-content">
                        <div class="tier-heading">
                            <div><span class="tier-cefr">{tier.cefr}</span><h2>{tier.name}</h2></div>
                            {#if tier.level === userTier && !locked}<MapPin size={18} class="text-accent-strong" aria-label="Current tier" />{/if}
                        </div>
                        {#if !locked}
                            {#if tier.level === userTier}<div class="progress-track"><div style={`width:${progress(tier.level)}%`}></div></div>{/if}
                            <div class="theme-grid">
                                {#each tier.themes as theme}
                                    <button class:active={activeThemeId === theme.id} onclick={() => setActiveTheme(theme.id)}>{theme.name}</button>
                                {/each}
                            </div>
                            {#if tier.level === userTier}
                                <div class="tier-actions">
                                    <a href="/journal" title="Journal"><NotebookPen size={18} /></a>
                                    <a href="/conjugator" title="Conjugation"><Languages size={18} /></a>
                                </div>
                            {/if}
                        {/if}
                    </div>
                </section>
            {/each}
        </div>
    {/if}
</div>

<style>
    .xp-pill { border:1px solid var(--pv-border); border-radius:999px; padding:.4rem .7rem; color:var(--pv-muted); font-size:.76rem; font-weight:750; }
    .learning-path { position:relative; display:flex; flex-direction:column; gap:.75rem; }
    .learning-path::before { content:''; position:absolute; left:1.5rem; top:1.5rem; bottom:1.5rem; width:2px; background:var(--pv-border); }
    .tier-row { position:relative; z-index:1; display:grid; grid-template-columns:3rem minmax(0,1fr); gap:1rem; align-items:start; }
    .tier-row.locked { opacity:.45; }
    .tier-node { display:grid; width:3rem; height:3rem; place-items:center; border-radius:999px; color:var(--pv-on-tier); font-weight:900; box-shadow:0 0 0 5px var(--pv-canvas); }
    .tier-row.locked .tier-node { background:var(--pv-surface-raised); color:var(--pv-subtle); }
    .tier-content { padding:.15rem 0 1rem; }
    .tier-heading { display:flex; align-items:center; justify-content:space-between; gap:1rem; }
    .tier-heading h2 { margin:.1rem 0 0; color:var(--pv-foreground); font-size:1rem; font-weight:800; }
    .tier-cefr { color:var(--pv-subtle); font-size:.68rem; font-weight:800; letter-spacing:.08em; }
    .progress-track { height:4px; margin:.75rem 0; overflow:hidden; border-radius:999px; background:var(--pv-surface-raised); }
    .progress-track div { height:100%; border-radius:inherit; background:var(--pv-accent-strong); }
    .theme-grid { display:flex; flex-wrap:wrap; gap:.45rem; margin-top:.75rem; }
    .theme-grid button { border:1px solid var(--pv-border); border-radius:999px; background:transparent; color:var(--pv-muted); padding:.45rem .7rem; font-size:.76rem; font-weight:650; cursor:pointer; }
    .theme-grid button.active { border-color:transparent; background:var(--pv-accent-soft); color:var(--pv-accent-strong); }
    .tier-actions { display:flex; gap:.4rem; margin-top:.7rem; }
    .tier-actions a { display:grid; width:2.25rem; height:2.25rem; place-items:center; border-radius:.7rem; background:var(--pv-surface-raised); color:var(--pv-muted); }
</style>
