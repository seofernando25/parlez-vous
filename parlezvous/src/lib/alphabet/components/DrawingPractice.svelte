<script lang="ts">
    import { onMount } from 'svelte';
    import { Eraser, Check, ChevronRight } from 'lucide-svelte';
    import type { AlphabetPracticeController } from '../controller.svelte';
    let { practice }: { practice: AlphabetPracticeController } = $props();
    let canvas: HTMLCanvasElement;
    onMount(() => { void practice.init(canvas); });
</script>

<div class="drawing-zone" class:correct={practice.result !== null && practice.isCorrect} class:wrong={practice.result !== null && !practice.isCorrect}>
    {#if practice.showGuide && practice.targetChar}<div class="guide">{practice.targetChar}</div>{/if}
    <canvas bind:this={canvas} width="280" height="280" onmousedown={practice.startDrawing} onmousemove={practice.draw} onmouseup={practice.stopDrawing} onmouseout={practice.stopDrawing} onblur={practice.stopDrawing} ontouchstart={practice.startDrawing} ontouchmove={practice.draw} ontouchend={practice.stopDrawing} ontouchcancel={practice.stopDrawing}></canvas>
</div>

<div class="drawing-actions">
    <button class="icon-button" onclick={practice.clearCanvas} title="Clear" aria-label="Clear drawing"><Eraser size={18} /></button>
    <button class="submit" onclick={() => practice.submit()} disabled={practice.isSubmitting} title="Check" aria-label="Check drawing"><Check size={19} /></button>
    <button class="icon-button" onclick={() => practice.next()} title="Next" aria-label="Next letter"><ChevronRight size={19} /></button>
</div>

{#if practice.result !== null}
    <div class="drawing-result" class:correct={practice.isCorrect} class:wrong={!practice.isCorrect}>{practice.isCorrect ? `${practice.targetChar} · ${practice.currentItem?.romanization || ''}` : `${practice.result} → ${practice.targetChar}`}</div>
{/if}
{#if practice.sessionTotal > 0}<div class="score">{practice.sessionCorrect}/{practice.sessionTotal}</div>{/if}

<style>
    .drawing-zone { position:relative; border:1px solid var(--pv-border-strong); border-radius:1rem; background:white; padding:.25rem; transition:border-color 140ms ease; }
    .drawing-zone.correct { border-color:var(--pv-success); }
    .drawing-zone.wrong { border-color:var(--pv-danger); }
    .guide { pointer-events:none; position:absolute; inset:0; display:grid; place-items:center; color:rgb(0 0 0 / .08); font-size:9rem; font-weight:850; user-select:none; }
    canvas { position:relative; z-index:1; display:block; width:280px; max-width:100%; height:auto; aspect-ratio:1; border-radius:.8rem; cursor:crosshair; touch-action:none; }
    .drawing-actions { display:flex; align-items:center; justify-content:center; gap:.6rem; }
    .submit { display:grid; width:3.25rem; height:3.25rem; place-items:center; border:0; border-radius:999px; background:var(--pv-accent); color:var(--pv-on-accent); cursor:pointer; }
    .submit:disabled { opacity:.4; }
    .drawing-result { align-self:center; border-radius:999px; padding:.4rem .7rem; font-size:.78rem; font-weight:750; }
    .drawing-result.correct { background:color-mix(in oklch,var(--pv-success) 10%,transparent); color:var(--pv-success); }
    .drawing-result.wrong { background:color-mix(in oklch,var(--pv-danger) 10%,transparent); color:var(--pv-danger); }
    .score { align-self:center; color:var(--pv-subtle); font-size:.7rem; font-weight:750; }
</style>
