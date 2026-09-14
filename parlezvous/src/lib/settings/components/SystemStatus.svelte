<script lang="ts">
    import { aiProviderState } from '$lib/state/aiProvider.svelte';
    import { managedAiState } from '$lib/state/managedAi.svelte';
    import { providerPreset } from '$lib/ai/providers';
    import { settingsState } from '$lib/state/settings.svelte';
    let { visible }: { visible: boolean } = $props();
    let providerName = $derived(providerPreset(settingsState.aiProvider).label);
    let healthy = $derived(settingsState.aiProvider === 'managed' ? managedAiState.ready : aiProviderState.isHealthy);
</script>
{#if visible}
<section class="status-line">
    <span>{providerName}</span>
    <span class:healthy><i></i>{healthy ? 'Ready' : 'Unavailable'}</span>
</section>
{/if}
<style>
    .status-line { display:flex; align-items:center; justify-content:space-between; gap:1rem; padding:.8rem 0; color:var(--pv-muted); font-size:.8rem; }
    .status-line span:last-child { display:flex; align-items:center; gap:.45rem; }
    .status-line i { width:.45rem; height:.45rem; border-radius:999px; background:var(--pv-danger); }
    .status-line .healthy i { background:var(--pv-success); }
</style>
