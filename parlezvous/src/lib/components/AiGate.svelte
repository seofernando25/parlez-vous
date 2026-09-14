<script lang="ts">
    import type { Snippet } from 'svelte';
    import { page } from '$app/stores';
    import { Brain, LoaderCircle, Image as ImageIcon } from 'lucide-svelte';
    import { isAiReady } from '$lib/ai/readiness';
    import { installManagedVision, managedAiState } from '$lib/state/managedAi.svelte';
    import { settingsState } from '$lib/state/settings.svelte';
    import toast from 'svelte-french-toast';

    let { feature, capability = 'text', children }: {
        feature: string;
        capability?: 'text' | 'vision';
        children: Snippet;
    } = $props();

    let baseReady = $derived(isAiReady());
    let needsManagedVision = $derived(
        capability === 'vision' && settingsState.aiProvider === 'managed' && !managedAiState.visionInstalled
    );
    let ready = $derived(baseReady && !needsManagedVision);
    let setupHref = $derived(`/setup/ai?return=${encodeURIComponent($page.url.pathname)}`);

    async function addVision() {
        try { await installManagedVision(); }
        catch (error) { toast.error(`Could not add vision: ${error}`); }
    }
</script>

{#if ready}
    {@render children()}
{:else}
    <div class="ai-gate">
        <div class="gate-icon">{#if capability === 'vision'}<ImageIcon size={25} />{:else}<Brain size={25} />{/if}</div>
        <h1>{feature}</h1>
        {#if settingsState.aiProvider === 'managed' && !managedAiState.checked}
            <p>Checking local AI…</p>
            <LoaderCircle size={20} class="animate-spin" />
        {:else if managedAiState.installing || managedAiState.installingVision}
            <p>{managedAiState.stage || 'Setting up AI'}{managedAiState.progress >= 0 ? ` · ${managedAiState.progress}%` : ''}</p>
            <LoaderCircle size={20} class="animate-spin" />
        {:else if needsManagedVision && baseReady}
            <p>Add local vision once to use this practice. About 1.2 GB.</p>
            <button type="button" onclick={addVision}>Add vision</button>
        {:else if settingsState.aiProvider === 'managed' && managedAiState.supported}
            <p>Set up AI once to use this feature.</p>
            <a href={setupHref}>Set up AI</a>
        {:else}
            <p>The configured AI service isn’t available yet.</p>
            <a href="/settings/advanced">AI settings</a>
        {/if}
    </div>
{/if}

<style>
    .ai-gate { display:flex; width:min(100%,34rem); min-height:70%; margin:auto; flex-direction:column; align-items:center; justify-content:center; gap:.7rem; padding:2rem; text-align:center; }
    .gate-icon { display:grid; width:3.5rem; height:3.5rem; place-items:center; border-radius:1rem; background:var(--pv-accent-soft); color:var(--pv-accent-strong); }
    h1 { margin:.25rem 0 0; font-size:1.35rem; letter-spacing:-.025em; }
    p { margin:0; color:var(--pv-muted); font-size:.85rem; }
    button,a { margin-top:.35rem; border:0; border-radius:.75rem; background:var(--pv-accent); color:var(--pv-on-accent); padding:.65rem .9rem; font:inherit; font-size:.8rem; font-weight:750; text-decoration:none; cursor:pointer; }
</style>
