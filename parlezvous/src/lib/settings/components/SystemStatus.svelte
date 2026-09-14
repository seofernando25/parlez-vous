<script lang="ts">
    import { ollamaState } from '$lib/state/ollama.svelte';
    let { visible }: { visible: boolean } = $props();
</script>
{#if visible}
<section class="space-y-3">
    <h3 class="text-ui-lg font-semibold text-foreground">System status</h3>
    <div class="flex items-center justify-between rounded-xl border border-border-strong bg-surface-raised p-4">
        <span class="text-muted">Ollama backend</span>
        <span class="flex items-center gap-2"><span class="h-2 w-2 rounded-full {ollamaState.isHealthy ? 'bg-success' : 'bg-danger'}"></span>{ollamaState.isHealthy ? 'Connected' : 'Disconnected'}</span>
    </div>
</section>
{/if}
