<script lang="ts">
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';
    import { ChevronLeft, ChevronRight } from 'lucide-svelte';
    import { SETTINGS_SECTIONS } from '$lib/settings/sections';
    onMount(() => { if (matchMedia('(min-width: 768px)').matches) void goto('/settings/general', { replaceState: true }); });
</script>

<div class="settings-index">
    <header class="index-header">
        <a href="/profile" aria-label="Back to profile"><ChevronLeft size={20} /></a>
        <h1>Settings</h1>
    </header>
    <nav aria-label="Settings sections">
        {#each SETTINGS_SECTIONS as section}
            {@const Icon = section.icon}
            <a href={section.href}><Icon size={19} /><span>{section.label}</span><ChevronRight class="chevron" size={17} /></a>
        {/each}
    </nav>
</div>

<style>
    .settings-index { max-width:34rem; margin-inline:auto; }
    .index-header { display:flex; align-items:center; gap:.75rem; margin-bottom:1rem; }
    .index-header a { display:grid; width:2.35rem; height:2.35rem; place-items:center; border-radius:.7rem; color:var(--pv-muted); }
    .index-header h1 { margin:0; color:var(--pv-foreground); font-size:1.3rem; font-weight:800; }
    .settings-index nav { border-top:1px solid var(--pv-border); }
    .settings-index nav a { display:flex; min-height:3.7rem; align-items:center; gap:.8rem; border-bottom:1px solid var(--pv-border); color:var(--pv-foreground); text-decoration:none; }
    .settings-index nav a > :global(svg:first-child) { color:var(--pv-muted); }
    .settings-index :global(.chevron) { margin-left:auto; color:var(--pv-subtle); }
    @media (min-width:768px) { .settings-index { display:none; } }
</style>
