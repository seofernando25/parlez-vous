<script lang="ts">
    import { Check, LoaderCircle, Server } from 'lucide-svelte';
    import { managedAiState, useManagedAi } from '$lib/state/managedAi.svelte';
    import { settingsState } from '$lib/state/settings.svelte';
    import toast from 'svelte-french-toast';

    async function useLocal() {
        try { await useManagedAi(); }
        catch (error) { toast.error(`Could not start local AI: ${error}`); }
    }
</script>

<div class="managed-ai">
    <div class="ai-icon" class:ready={managedAiState.ready}>
        {#if managedAiState.installing}<LoaderCircle size={20} class="animate-spin" />
        {:else if managedAiState.ready}<Check size={20} />
        {:else}<Server size={20} />{/if}
    </div>
    <div class="ai-copy">
        <strong>Local AI</strong>
        {#if managedAiState.installing}
            <span>{managedAiState.stage}{managedAiState.progress >= 0 ? ` · ${managedAiState.progress}%` : ''}</span>
        {:else if managedAiState.ready && settingsState.aiProvider === 'managed'}
            <span>Ready on this device.</span>
        {:else if managedAiState.installed}
            <span>Installed on this device.</span>
        {:else}
            <span>Private tutor and textbook search. About 3.5 GB.</span>
        {/if}
    </div>
    {#if !managedAiState.installing}
        {#if managedAiState.installed && settingsState.aiProvider !== 'managed'}
            <button class="quiet-action" type="button" onclick={useLocal}>Use local</button>
        {:else if !managedAiState.installed}
            <a class="primary-compact" href="/setup/ai">Set up</a>
        {/if}
    {/if}
    {#if managedAiState.installing && managedAiState.progress >= 0}
        <div class="progress"><span style={`width:${managedAiState.progress}%`}></span></div>
    {/if}
</div>

<style>
    .managed-ai { position:relative; display:grid; grid-template-columns:2.8rem minmax(0,1fr) auto; align-items:center; gap:.85rem; padding:1rem 0; }
    .ai-icon { display:grid; width:2.8rem; height:2.8rem; place-items:center; border-radius:.85rem; background:var(--pv-surface-raised); color:var(--pv-muted); }
    .ai-icon.ready { background:color-mix(in oklch,var(--pv-success) 14%,var(--pv-surface)); color:var(--pv-success); }
    .ai-copy { display:flex; min-width:0; flex-direction:column; gap:.2rem; }
    .ai-copy strong { font-size:.92rem; }
    .ai-copy span { color:var(--pv-muted); font-size:.76rem; line-height:1.4; }
    .primary-compact,.quiet-action { display:flex; align-items:center; gap:.4rem; border:0; border-radius:.7rem; padding:.55rem .75rem; font:inherit; font-size:.78rem; font-weight:700; cursor:pointer; text-decoration:none; }
    .primary-compact { background:var(--pv-accent); color:var(--pv-on-accent); }
    .quiet-action { background:var(--pv-surface-raised); color:var(--pv-foreground); }
    .progress { position:absolute; right:0; bottom:0; left:3.65rem; height:2px; overflow:hidden; border-radius:999px; background:var(--pv-border); }
    .progress span { display:block; height:100%; background:var(--pv-accent); transition:width 180ms ease; }
    @media (max-width:640px) { .managed-ai { grid-template-columns:2.8rem minmax(0,1fr); } .managed-ai > :global(a),.managed-ai > button { grid-column:2; justify-self:start; } }
</style>
