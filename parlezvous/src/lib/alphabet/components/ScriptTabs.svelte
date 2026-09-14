<script lang="ts">
    import { SCRIPT_INFO, type ScriptType } from '../types';
    let { active, onSelect }: { active: ScriptType; onSelect: (script: ScriptType) => void } = $props();
</script>
<div class="flex w-full gap-1 overflow-x-auto rounded-xl border border-border bg-surface p-1">
    {#each Object.keys(SCRIPT_INFO) as key}
        {@const script = key as ScriptType}
        <button class="flex-1 whitespace-nowrap rounded-lg px-3 py-2 text-xs font-bold transition-colors {active === script ? 'bg-surface-raised text-accent shadow-sm' : 'text-muted hover:text-accent'}" onclick={() => onSelect(script)}>{SCRIPT_INFO[script].icon} {SCRIPT_INFO[script].title}</button>
    {/each}
</div>
