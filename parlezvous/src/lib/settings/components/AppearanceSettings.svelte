<script lang="ts">
    import { Monitor, Sun, Moon } from 'lucide-svelte';
    import { setTheme, themeState, type ThemeMode } from '$lib/state/theme.svelte';
    const options: Array<{ mode: ThemeMode; label: string; icon: typeof Monitor }> = [
        { mode: 'system', label: 'System', icon: Monitor }, { mode: 'light', label: 'Light', icon: Sun }, { mode: 'dark', label: 'Dark', icon: Moon }
    ];
</script>
<div class="grid grid-cols-3 gap-2">
    {#each options as option}
        {@const Icon = option.icon}
        <button type="button" aria-pressed={themeState.mode === option.mode} onclick={() => setTheme(option.mode)} class="appearance-option {themeState.mode === option.mode ? 'active' : ''}">
            <Icon size={18} /><span>{option.label}</span>
        </button>
    {/each}
</div>
<style>
    .appearance-option { display:flex; align-items:center; justify-content:center; gap:.45rem; min-height:2.7rem; border:1px solid var(--pv-border); border-radius:.8rem; background:transparent; color:var(--pv-muted); font-size:.8rem; font-weight:650; }
    .appearance-option.active { background:var(--pv-accent-soft); color:var(--pv-accent-strong); border-color:transparent; }
</style>
