<script lang="ts">
    import { FileText, X, LoaderCircle } from 'lucide-svelte';
    import type { TextbookController } from '../textbooks.svelte';
    let { textbooks }: { textbooks: TextbookController } = $props();
</script>

{#if textbooks.active}
    <div class="textbook-viewport">
        {#if textbooks.hasError}
            <div class="textbook-error"><FileText size={28} /><button class="icon-button" onclick={() => textbooks.dismiss()} title="Close textbook" aria-label="Close textbook"><X size={17} /></button></div>
        {:else if textbooks.src}
            <embed src={textbooks.src} type="application/pdf" onerror={() => textbooks.hasError = true} />
        {:else}
            <div class="textbook-loading"><LoaderCircle size={24} class="animate-spin" /></div>
        {/if}
    </div>
{/if}

<style>
    .textbook-viewport { position:relative; min-height:0; flex:1; overflow:hidden; border:1px solid var(--pv-border); border-radius:1rem; background:var(--pv-surface); }
    .textbook-viewport embed { width:100%; height:100%; }
    .textbook-error,.textbook-loading { display:flex; width:100%; height:100%; align-items:center; justify-content:center; gap:.7rem; color:var(--pv-subtle); }
    @media (min-width:768px) { .textbook-viewport { border-radius:0; border-left:0; border-bottom:0; } }
</style>
