<script lang="ts">
    import { aiProviderState } from '$lib/state/aiProvider.svelte';
    import { providerPreset } from '$lib/ai/providers';
    import { settingsState } from '$lib/state/settings.svelte';
    let { visible }: { visible: boolean } = $props();
    let providerName = $derived(providerPreset(settingsState.aiProvider).label);
</script>
{#if visible}
<section class="space-y-3">
    <div class="flex items-center justify-between rounded-xl border border-border-strong bg-surface-raised p-4">
        <span class="text-muted">{providerName} endpoint</span>
        <span class="flex items-center gap-2"><span class="h-2 w-2 rounded-full {aiProviderState.isHealthy ? 'bg-success' : 'bg-danger'}"></span>{aiProviderState.isHealthy ? 'Connected' : 'Unavailable'}</span>
    </div>
</section>
{/if}
