<script lang="ts">
    import { Eye, Volume2, ChevronLeft, ChevronRight, Shuffle } from 'lucide-svelte';
    import type { AlphabetPracticeController } from '../controller.svelte';
    let { practice }: { practice: AlphabetPracticeController } = $props();
</script>

<section class="letter-card">
    {#if practice.letters.length}
        <div class="letter-strip">
            {#each practice.letters as item, index}
                <button class:active={index === practice.currentIndex} onclick={() => practice.select(index)}>{practice.isUppercase ? item.uppercase : item.lowercase}</button>
            {/each}
        </div>
    {/if}

    <div class="letter-toolbar">
        {#if practice.activeScript !== 'korean'}
            <div class="case-switch"><button class:active={practice.isUppercase} onclick={() => practice.setUppercase(true)}>Aa</button><button class:active={!practice.isUppercase} onclick={() => practice.setUppercase(false)}>aa</button></div>
        {:else}<span></span>{/if}
        <div class="flex gap-1">
            <button class="icon-button" class:active={practice.showGuide} onclick={() => practice.showGuide = !practice.showGuide} title="Guide" aria-label="Toggle guide"><Eye size={16} /></button>
            <button class="icon-button" onclick={() => practice.playAudio()} title="Listen" aria-label="Listen"><Volume2 size={16} /></button>
        </div>
    </div>

    <div class="letter-main"><strong>{practice.targetChar}</strong><div><span>{practice.currentItem?.romanization || ''}</span><small>{practice.currentItem?.pronunciation || ''}</small></div></div>

    <div class="letter-nav">
        <button class="icon-button" onclick={() => practice.previous()} title="Previous" aria-label="Previous"><ChevronLeft size={17} /></button>
        <button class="icon-button" onclick={() => practice.random()} title="Random" aria-label="Random"><Shuffle size={16} /></button>
        <button class="icon-button" onclick={() => practice.next()} title="Next" aria-label="Next"><ChevronRight size={17} /></button>
    </div>
</section>

<style>
    .letter-card { width:100%; border-bottom:1px solid var(--pv-border); padding-bottom:1rem; }
    .letter-strip { display:flex; gap:.3rem; overflow-x:auto; padding-bottom:.6rem; scrollbar-width:none; }
    .letter-strip::-webkit-scrollbar { display:none; }
    .letter-strip button { flex:0 0 2rem; height:2rem; border:1px solid var(--pv-border); border-radius:.55rem; background:transparent; color:var(--pv-subtle); font-weight:750; cursor:pointer; }
    .letter-strip button.active { border-color:transparent; background:var(--pv-accent-soft); color:var(--pv-accent-strong); }
    .letter-toolbar { display:flex; align-items:center; justify-content:space-between; gap:1rem; margin-top:.4rem; }
    .case-switch { display:flex; gap:.2rem; padding:.2rem; border-radius:.65rem; background:var(--pv-surface-raised); }
    .case-switch button { border:0; border-radius:.5rem; background:transparent; color:var(--pv-subtle); padding:.35rem .55rem; font-size:.72rem; font-weight:700; cursor:pointer; }
    .case-switch button.active { background:var(--pv-surface); color:var(--pv-foreground); }
    .letter-main { display:flex; flex-direction:column; align-items:center; gap:.45rem; padding:1.2rem 0; }
    .letter-main > strong { color:var(--pv-foreground); font-size:4.2rem; font-weight:850; line-height:1; }
    .letter-main div { display:flex; align-items:center; gap:.55rem; }
    .letter-main span { color:var(--pv-accent-strong); font-weight:750; }
    .letter-main small { color:var(--pv-subtle); font-family:ui-monospace,monospace; }
    .letter-nav { display:flex; justify-content:center; gap:.4rem; }
</style>
