<script lang="ts">
    import { goto } from '$app/navigation';
    import { page } from '$app/stores';
    import { ArrowLeft, Check, LoaderCircle, Server, SlidersHorizontal } from 'lucide-svelte';
    import { installManagedAi, managedAiState } from '$lib/state/managedAi.svelte';
    import toast from 'svelte-french-toast';

    let returnTo = $derived($page.url.searchParams.get('return') || '/map');

    async function express() {
        try {
            await installManagedAi();
            await goto(returnTo);
        } catch (error) {
            toast.error(`Could not set up local AI: ${error}`);
        }
    }
</script>

<div class="setup-page">
    <header>
        <a href={returnTo} class="back" aria-label="Back"><ArrowLeft size={18} /></a>
        <div><h1>Set up AI</h1><p>Choose the simple local setup or connect something you already use.</p></div>
    </header>

    <div class="choices">
        <section class="choice recommended">
            <div class="choice-icon">{#if managedAiState.ready}<Check size={22} />{:else}<Server size={22} />{/if}</div>
            <div class="copy">
                <div class="title-row"><h2>Express</h2><span>Recommended</span></div>
                <p>Private AI on this computer. ParlezVous chooses and configures the runtime, tutor model, and textbook search model for you.</p>
                <small>About 3.5 GB · downloaded once</small>
            </div>
            <button type="button" onclick={express} disabled={managedAiState.installing || (!managedAiState.supported && managedAiState.checked)}>
                {#if managedAiState.installing}<LoaderCircle size={17} class="animate-spin" /> {managedAiState.stage || 'Setting up'}
                {:else if managedAiState.ready}Continue
                {:else}Set up locally{/if}
            </button>
            {#if managedAiState.installing && managedAiState.progress >= 0}
                <div class="progress"><span style={`width:${managedAiState.progress}%`}></span></div>
            {/if}
        </section>

        <a class="choice advanced" href={`/setup/ai/advanced?return=${encodeURIComponent(returnTo)}`}>
            <div class="choice-icon"><SlidersHorizontal size={22} /></div>
            <div class="copy"><h2>Advanced</h2><p>Connect LM Studio, Ollama, OpenRouter, OpenAI, or another compatible endpoint.</p></div>
        </a>
    </div>

    <a class="not-now" href={returnTo}>Not now</a>
</div>

<style>
    .setup-page { width:min(100%,46rem); margin:0 auto; padding:clamp(1.25rem,4vw,3rem); }
    header { display:flex; align-items:flex-start; gap:.9rem; margin:0 0 2rem; }
    header h1 { margin:0; font-size:clamp(1.7rem,4vw,2.2rem); letter-spacing:-.04em; }
    header p { margin:.35rem 0 0; color:var(--pv-muted); line-height:1.5; }
    .back { display:grid; width:2.3rem; height:2.3rem; flex:0 0 2.3rem; place-items:center; border-radius:.7rem; color:var(--pv-muted); }
    .choices { display:grid; gap:.8rem; }
    .choice { position:relative; display:grid; grid-template-columns:3rem minmax(0,1fr) auto; align-items:center; gap:1rem; border:1px solid var(--pv-border); border-radius:1rem; background:var(--pv-surface); padding:1rem; color:var(--pv-foreground); text-decoration:none; }
    .choice.recommended { border-color:color-mix(in oklch,var(--pv-accent) 38%,var(--pv-border)); }
    .choice-icon { display:grid; width:3rem; height:3rem; place-items:center; border-radius:.9rem; background:var(--pv-surface-raised); color:var(--pv-accent-strong); }
    .copy h2 { margin:0; font-size:1rem; }
    .copy p { margin:.28rem 0 0; color:var(--pv-muted); font-size:.82rem; line-height:1.5; }
    .copy small { display:block; margin-top:.45rem; color:var(--pv-subtle); font-size:.7rem; }
    .title-row { display:flex; align-items:center; gap:.55rem; }
    .title-row span { border-radius:999px; background:var(--pv-accent-soft); color:var(--pv-accent-strong); padding:.2rem .42rem; font-size:.62rem; font-weight:800; }
    button { display:flex; align-items:center; gap:.4rem; border:0; border-radius:.75rem; background:var(--pv-accent); color:var(--pv-on-accent); padding:.65rem .85rem; font:inherit; font-size:.78rem; font-weight:750; cursor:pointer; }
    button:disabled { opacity:.55; cursor:default; }
    .advanced:hover { background:var(--pv-surface-raised); }
    .progress { position:absolute; right:1rem; bottom:.35rem; left:5rem; height:2px; overflow:hidden; border-radius:999px; background:var(--pv-border); }
    .progress span { display:block; height:100%; background:var(--pv-accent); }
    .not-now { display:inline-block; margin-top:1.2rem; color:var(--pv-muted); font-size:.8rem; text-decoration:none; }
    @media (max-width:640px) { .choice { grid-template-columns:3rem minmax(0,1fr); } .choice > button { grid-column:2; justify-self:start; } }
</style>
