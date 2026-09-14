<script lang="ts">
    import { LoaderCircle, Check, X } from 'lucide-svelte';
    import type { ConjugationController } from '../controller.svelte';
    let { controller }: { controller: ConjugationController } = $props();
</script>

{#if controller.isGenerating}
    <div class="exercise-empty"><LoaderCircle size={24} class="animate-spin" /></div>
{:else if controller.exercise}
    <section class="exercise">
        {#if controller.timerActive}
            <div class="timer"><div><span style={`width:${controller.timerProgress * 100}%;background:${controller.timerColor}`}></span></div><small style={`color:${controller.timerColor}`}>{controller.formattedTime}</small></div>
        {/if}

        <div class="verb"><h2>{controller.exercise.verb}</h2><span>{controller.exercise.translation}</span></div>
        <p class="sentence">{controller.displaySentence}</p>
        <div class="facts"><span>{controller.exercise.tense}</span><span>{controller.exercise.subject}</span></div>

        <div class="answer-area">
            <input bind:this={controller.inputEl} bind:value={controller.userAnswer} onkeydown={controller.handleInputKeydown} disabled={controller.hasSubmitted} autocomplete="off" spellcheck="false" placeholder="…" aria-label="Conjugation answer" />
            {#if !controller.hasSubmitted}
                <button onclick={() => controller.checkAnswer()} disabled={!controller.userAnswer.trim()}>Check</button>
            {:else if controller.isCorrect}
                <div class="result correct"><Check size={18} /><span>Correct</span></div>
            {:else}
                <div class="result wrong"><X size={18} /><span>{controller.exercise.answer}</span></div>
            {/if}
        </div>
    </section>
{:else}
    <div class="exercise-empty">+</div>
{/if}

<style>
    .exercise-empty { display:grid; min-height:24rem; place-items:center; color:var(--pv-subtle); font-size:1.5rem; }
    .exercise { min-height:24rem; display:flex; flex-direction:column; align-items:center; justify-content:center; text-align:center; }
    .timer { display:flex; width:100%; align-items:center; gap:.7rem; margin-bottom:2rem; }
    .timer > div { height:3px; flex:1; overflow:hidden; border-radius:999px; background:var(--pv-surface-raised); }
    .timer > div span { display:block; height:100%; border-radius:inherit; transition:width 1s linear; }
    .timer small { min-width:2.7rem; font-family:ui-monospace, monospace; font-weight:700; }
    .verb h2 { margin:0; color:var(--pv-foreground); font-size:clamp(2rem,6vw,3.4rem); font-weight:850; letter-spacing:-.04em; }
    .verb span { display:block; margin-top:.25rem; color:var(--pv-subtle); font-size:.85rem; }
    .sentence { max-width:38rem; margin:2rem 0 1rem; color:var(--pv-muted); font-size:clamp(1.15rem,3vw,1.5rem); line-height:1.55; }
    .facts { display:flex; gap:.45rem; flex-wrap:wrap; justify-content:center; }
    .facts span { border:1px solid var(--pv-border); border-radius:999px; padding:.35rem .6rem; color:var(--pv-subtle); font-size:.72rem; }
    .answer-area { width:min(100%,24rem); margin-top:2rem; }
    .answer-area input { width:100%; box-sizing:border-box; border:0; border-bottom:2px solid var(--pv-border-strong); background:transparent; color:var(--pv-foreground); padding:.8rem .25rem; text-align:center; font-size:1.1rem; outline:none; }
    .answer-area input:focus { border-color:var(--pv-accent); }
    .answer-area > button { width:100%; margin-top:.8rem; border:0; border-radius:.8rem; background:var(--pv-accent); color:var(--pv-on-accent); padding:.75rem; font-weight:800; cursor:pointer; }
    .answer-area > button:disabled { opacity:.35; cursor:not-allowed; }
    .result { display:flex; min-height:2.8rem; align-items:center; justify-content:center; gap:.5rem; margin-top:.8rem; border-radius:.8rem; font-weight:800; }
    .correct { color:var(--pv-success); background:color-mix(in oklch,var(--pv-success) 10%,transparent); }
    .wrong { color:var(--pv-danger); background:color-mix(in oklch,var(--pv-danger) 10%,transparent); }
</style>
