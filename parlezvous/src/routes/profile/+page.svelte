<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';
    import { ChartNoAxesColumnIncreasing, CalendarDays, Settings, ChevronRight } from 'lucide-svelte';
    import { settingsState } from '$lib/state/settings.svelte';
    import { profileState, loadProfile } from '$lib/state/profile.svelte';
    import { getTierFromXP } from '$lib/curriculum';

    let totalXp = $state(0);
    let activeSeconds = $state(0);
    let loading = $state(true);
    let tier = $derived(getTierFromXP(totalXp));
    let studyTime = $derived(activeSeconds < 3600 ? `${Math.max(0, Math.floor(activeSeconds / 60))}m` : `${Math.floor(activeSeconds / 3600)}h ${Math.floor((activeSeconds % 3600) / 60)}m`);

    onMount(async () => {
        try {
            if (!profileState.isLoaded) await loadProfile();
            const curriculum = await invoke<{ total_xp: number; active_seconds: number }>('get_curriculum', { language: settingsState.targetLanguage });
            totalXp = curriculum.total_xp || 0;
            activeSeconds = curriculum.active_seconds || 0;
        } catch (error) { console.error('Failed to load profile summary:', error); }
        finally { loading = false; }
    });
</script>

<div class="app-page app-page--narrow">
    <header class="profile-header">
        <h1 class="page-title">{profileState.displayName}</h1>
        <p class="page-subtitle">{settingsState.targetLanguage} · {settingsState.skillLevel}</p>
    </header>

    <section class="profile-stats" aria-label="Learning summary">
        <div><strong>{loading ? '—' : totalXp.toLocaleString()}</strong><span>XP</span></div>
        <div><strong>{loading ? '—' : tier}</strong><span>Tier</span></div>
        <div><strong>{loading ? '—' : studyTime}</strong><span>Study</span></div>
    </section>

    <nav class="profile-links" aria-label="Profile sections">
        <a class="link-row" href="/stats"><ChartNoAxesColumnIncreasing class="row-icon" size={20} /><span>Progress</span><ChevronRight class="row-chevron" size={18} /></a>
        <a class="link-row" href="/calendar"><CalendarDays class="row-icon" size={20} /><span>Journal history</span><ChevronRight class="row-chevron" size={18} /></a>
        <a class="link-row mobile-settings-link" href="/settings"><Settings class="row-icon" size={20} /><span>Settings</span><ChevronRight class="row-chevron" size={18} /></a>
    </nav>
</div>

<style>
    .profile-header { padding:.5rem 0 1.5rem; }
    .profile-stats { display:grid; grid-template-columns:repeat(3,1fr); border-block:1px solid var(--pv-border); }
    .profile-stats div { display:flex; flex-direction:column; align-items:center; gap:.2rem; padding:1.2rem .4rem; }
    .profile-stats div + div { border-left:1px solid var(--pv-border); }
    .profile-stats strong { color:var(--pv-foreground); font-size:1.15rem; }
    .profile-stats span { color:var(--pv-subtle); font-size:.72rem; letter-spacing:.08em; text-transform:uppercase; }
    .profile-links { margin-top:1.4rem; }
    @media (min-width:768px) { .mobile-settings-link { display:none; } }
</style>
