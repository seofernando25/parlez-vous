<script lang="ts">
    import { Download, Trash2, Check, LoaderCircle } from 'lucide-svelte';
    let { title, ready, downloading, progress = -1, onDownload, onPurge, onVerify }: {
        title: string; ready: boolean; downloading: boolean; progress?: number;
        onDownload: () => void; onPurge: () => void; onVerify: () => void;
    } = $props();
</script>

<div class="asset-row">
    <div class="min-w-0"><strong>{title}</strong><span>{downloading ? (progress >= 0 ? `${progress}%` : 'Downloading') : ready ? 'Ready' : 'Not installed'}</span></div>
    {#if downloading}<LoaderCircle class="animate-spin text-accent" size={18} />
    {:else if ready}<div class="flex gap-1"><button class="icon-button" onclick={onVerify} title="Verify" aria-label={`Verify ${title}`}><Check size={17} /></button><button class="icon-button" onclick={onPurge} title="Remove" aria-label={`Remove ${title}`}><Trash2 size={17} /></button></div>
    {:else}<button class="icon-button" onclick={onDownload} title="Download" aria-label={`Download ${title}`}><Download size={17} /></button>{/if}
</div>

<style>
    .asset-row { display: flex; min-height: 3.7rem; align-items: center; justify-content: space-between; gap: 1rem; border-bottom: 1px solid var(--pv-border); padding: 0.65rem 0; }
    .asset-row:last-child { border-bottom: 0; }
    .asset-row strong { display: block; color: var(--pv-foreground); font-size: 0.88rem; }
    .asset-row span { display: block; margin-top: 0.15rem; color: var(--pv-subtle); font-size: 0.72rem; }
</style>
