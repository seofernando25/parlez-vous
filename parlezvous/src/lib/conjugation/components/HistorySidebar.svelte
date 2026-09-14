<script lang="ts">
    import { Check, X } from 'lucide-svelte';
    import type { ConjugationController } from '../controller.svelte';
    let { controller }: { controller: ConjugationController } = $props();
</script>

{#if controller.history.length}
<aside class="history" aria-label="Recent exercises">
    <span class="history-title">Recent</span>
    {#each [...controller.history].reverse() as entry}
        <div class="history-row">
            {#if entry.correct}<Check size={15} class="text-success" />{:else}<X size={15} class="text-danger" />{/if}
            <div><strong>{entry.exercise.verb}</strong><span>{entry.exercise.tense}</span></div>
            <small>{entry.correct ? entry.userAnswer : entry.exercise.answer}</small>
        </div>
    {/each}
</aside>
{/if}

<style>
    .history { border-top:1px solid var(--pv-border); padding-top:1rem; }
    .history-title { display:block; margin-bottom:.4rem; color:var(--pv-subtle); font-size:.68rem; font-weight:800; letter-spacing:.08em; text-transform:uppercase; }
    .history-row { display:grid; grid-template-columns:auto minmax(0,1fr) auto; align-items:center; gap:.55rem; min-height:2.8rem; border-bottom:1px solid var(--pv-border); }
    .history-row div { min-width:0; }
    .history-row strong,.history-row span { display:block; overflow:hidden; text-overflow:ellipsis; white-space:nowrap; }
    .history-row strong { color:var(--pv-foreground); font-size:.8rem; }
    .history-row span,.history-row small { color:var(--pv-subtle); font-size:.68rem; }
    @media (min-width:900px) { .history { border-top:0; border-left:1px solid var(--pv-border); padding-top:0; padding-left:1rem; } }
</style>
