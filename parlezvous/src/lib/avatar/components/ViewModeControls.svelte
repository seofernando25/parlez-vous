<script lang="ts">
    import { Columns2, UserRound, MessagesSquare } from 'lucide-svelte';
    import type { ViewMode } from '../types';
    let { mode, onChange }: { mode: ViewMode; onChange: (mode: ViewMode) => void } = $props();
    const options: Array<{ mode: ViewMode; label: string; icon: typeof Columns2 }> = [
        { mode: 'split', label: 'Split view', icon: Columns2 },
        { mode: 'avatar', label: 'Avatar', icon: UserRound },
        { mode: 'chat', label: 'Chat', icon: MessagesSquare }
    ];
</script>

<div class="view-controls">
    {#each options as option}
        {@const Icon = option.icon}
        <button class:active={mode === option.mode} onclick={() => onChange(option.mode)} title={option.label} aria-label={option.label}><Icon size={16} /></button>
    {/each}
</div>

<style>
    .view-controls { display:flex; gap:.15rem; padding:.2rem; border:1px solid var(--pv-border); border-radius:.75rem; background:color-mix(in oklch,var(--pv-surface) 88%,transparent); backdrop-filter:blur(10px); }
    .view-controls button { display:grid; width:2rem; height:2rem; place-items:center; border:0; border-radius:.55rem; background:transparent; color:var(--pv-subtle); cursor:pointer; }
    .view-controls button:hover { color:var(--pv-foreground); }
    .view-controls button.active { background:var(--pv-accent-soft); color:var(--pv-accent-strong); }
</style>
