<script lang="ts">
    import { onDestroy, onMount } from 'svelte';
    import { page } from '$app/stores';
    import { ChevronLeft } from 'lucide-svelte';
    import { SettingsController } from '$lib/settings/controller.svelte';
    import { provideSettingsController } from '$lib/settings/context';
    import { SETTINGS_SECTIONS, settingsSection } from '$lib/settings/sections';

    let { children } = $props();
    const controller = new SettingsController();
    provideSettingsController(controller);
    let pathname = $derived($page.url.pathname);
    let title = $derived(settingsSection(pathname));

    onMount(() => { void controller.init(); });
    onDestroy(() => controller.dispose());
</script>

<div class="settings-shell">
    <aside class="settings-sidebar">
        <h1>Settings</h1>
        <nav aria-label="Settings sections">
            {#each SETTINGS_SECTIONS as section}
                {@const Icon = section.icon}
                <a href={section.href} class:active={pathname === section.href}><Icon size={17} /><span>{section.label}</span></a>
            {/each}
        </nav>
    </aside>

    <main class="settings-content">
        {#if pathname !== '/settings'}
            <header class="settings-mobile-header">
                <a href="/settings" aria-label="Back to settings"><ChevronLeft size={20} /></a>
                <h1>{title}</h1>
            </header>
        {/if}
        {@render children()}
    </main>
</div>

<style>
    .settings-shell { width:min(100%,64rem); min-height:100%; margin-inline:auto; }
    .settings-sidebar { display:none; }
    .settings-content { min-width:0; padding:1.25rem 1rem 2rem; }
    .settings-mobile-header { display:flex; align-items:center; gap:.75rem; margin-bottom:1.25rem; }
    .settings-mobile-header a { display:grid; width:2.35rem; height:2.35rem; place-items:center; border-radius:.7rem; color:var(--pv-muted); }
    .settings-mobile-header h1 { margin:0; color:var(--pv-foreground); font-size:1.15rem; font-weight:800; }
    @media (min-width:768px) {
        .settings-shell { display:grid; grid-template-columns:12rem minmax(0,1fr); gap:2.5rem; padding:2rem clamp(1.5rem,3vw,3rem) 3rem; }
        .settings-sidebar { display:block; align-self:start; position:sticky; top:2rem; }
        .settings-sidebar h1 { margin:.35rem 0 1.1rem; color:var(--pv-foreground); font-size:1.15rem; font-weight:800; }
        .settings-sidebar nav { display:flex; flex-direction:column; gap:.2rem; }
        .settings-sidebar a { display:flex; min-height:2.4rem; align-items:center; gap:.65rem; border-radius:.65rem; padding:0 .7rem; color:var(--pv-muted); font-size:.82rem; font-weight:650; text-decoration:none; }
        .settings-sidebar a:hover { background:var(--pv-surface-raised); color:var(--pv-foreground); }
        .settings-sidebar a.active { background:var(--pv-accent-soft); color:var(--pv-accent-strong); }
        .settings-content { padding:0; }
        .settings-mobile-header { display:none; }
    }
</style>
