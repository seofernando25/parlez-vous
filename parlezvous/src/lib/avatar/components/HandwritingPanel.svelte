<script lang="ts">
    import { onMount } from 'svelte';
    import type { HandwritingController } from '../handwriting/controller.svelte';
    let { handwriting, onClose }: { handwriting: HandwritingController; onClose: () => void } = $props();
    let canvas: HTMLCanvasElement;
    onMount(() => { handwriting.drawingCanvas = canvas; handwriting.initCanvas(); });
</script>

<div class="relative z-0 flex max-h-[60vh] min-h-0 shrink flex-col gap-3 overflow-y-auto overscroll-contain border-t border-border bg-surface p-3 pb-8 shadow-inner md:max-h-none md:p-4">
    <button type="button" onclick={onClose} class="absolute right-2.5 top-2.5 z-20 rounded-lg bg-surface-raised/90 p-1.5 text-muted transition-colors hover:bg-surface-hover hover:text-foreground" aria-label="Close handwriting keyboard">✕</button>
    <div class="flex flex-col items-center gap-4 xl:flex-row xl:items-stretch">
        <div class="flex w-full shrink-0 flex-col items-center gap-2 xl:w-auto">
            <canvas
                bind:this={canvas} width="280" height="280"
                class="h-[200px] w-[200px] touch-none cursor-crosshair rounded-2xl border-2 border-border-strong bg-paper shadow-inner sm:h-[240px] sm:w-[240px]"
                onmousedown={handwriting.startDrawing} onmousemove={handwriting.draw}
                onmouseup={handwriting.stopDrawing} onmouseleave={handwriting.stopDrawing}
                ontouchstart={handwriting.startDrawing} ontouchmove={handwriting.draw} ontouchend={handwriting.stopDrawing}
            ></canvas>
            <div class="flex w-[200px] gap-2 sm:w-[240px]">
                <button onclick={handwriting.clearCanvas} class="flex-1 rounded-lg bg-surface-raised py-2 text-xs font-medium text-muted transition-colors hover:bg-surface-hover sm:text-sm">Clear</button>
                <button onclick={handwriting.recognize} disabled={handwriting.isRecognizing} class="flex-1 rounded-lg border border-accent/30 bg-accent/20 py-2 text-xs font-bold text-accent transition-colors hover:bg-accent/30 disabled:opacity-50 sm:text-sm">{handwriting.isRecognizing ? '...' : 'Recognize'}</button>
            </div>
            <p class="mt-1 h-3 text-center text-[10px] text-muted sm:text-xs">{handwriting.status || ''}</p>
        </div>

        <div class="relative flex w-full min-w-0 flex-1 shrink-0 flex-col gap-3 rounded-2xl border border-border bg-canvas p-3 sm:p-4">
            {#if handwriting.activeScript === 'korean'}
                <div class="shrink-0 text-center">
                    <h3 class="text-xs font-semibold uppercase tracking-widest text-muted">Block</h3>
                    <div class="mt-1 flex h-14 items-center justify-center text-4xl font-black text-accent sm:h-16 sm:text-5xl">{handwriting.composedHangul || '-'}</div>
                </div>
                <div class="flex shrink-0 justify-between gap-1 sm:gap-2">
                    {#each ['initial', 'vowel', 'final'] as slot}
                        <button onclick={() => handwriting.setSlot(slot as 'initial' | 'vowel' | 'final')} class="flex flex-1 flex-col items-center rounded-xl border p-2 transition-colors sm:p-3 {handwriting.currentSlot === slot ? 'border-accent/50 bg-accent/10' : 'border-border bg-surface hover:bg-surface-raised'}">
                            <span class="text-[10px] uppercase text-subtle sm:text-xs">{slot}</span>
                            <span class="text-lg font-bold text-foreground sm:text-xl">{slot === 'initial' ? handwriting.blockInitial || '-' : slot === 'vowel' ? handwriting.blockVowel || '-' : handwriting.blockFinal || '-'}</span>
                        </button>
                    {/each}
                </div>
                <div class="mt-auto flex shrink-0 gap-2">
                    <button onclick={() => handwriting.clearHangulBlock()} class="rounded-xl bg-surface-raised px-3 py-2 text-xs font-medium text-muted hover:bg-danger/20 hover:text-danger sm:px-4 sm:py-3 sm:text-sm">Clear</button>
                    <button onclick={() => handwriting.appendInputSpace()} class="rounded-xl bg-surface-raised px-3 py-2 text-xs font-medium text-muted hover:bg-surface-hover sm:px-4 sm:py-3 sm:text-sm">Space</button>
                    <button onclick={() => handwriting.commitHangulBlock()} disabled={!handwriting.composedHangul} class="flex-1 rounded-xl bg-accent py-2 text-xs font-bold text-on-accent transition-colors hover:bg-accent-strong disabled:opacity-50 sm:py-3 sm:text-sm">Enter</button>
                </div>
            {:else}
                <div class="shrink-0 text-center">
                    <h3 class="text-xs font-semibold uppercase tracking-widest text-muted">{handwriting.activeScript === 'russian' ? 'Russian Word' : 'Ukrainian Word'}</h3>
                    <div class="mt-1 flex h-12 items-center justify-center overflow-x-auto px-2 text-2xl font-black tracking-wider text-accent sm:h-14 sm:text-3xl">{handwriting.composedWord || '—'}</div>
                </div>
                <div class="flex shrink-0 flex-col gap-1">
                    <div class="flex items-center justify-between">
                        <span class="text-[10px] font-semibold uppercase tracking-wider text-subtle">Alphabet</span>
                        <button type="button" onclick={() => handwriting.toggleCase()} class="rounded-lg border border-border-strong bg-surface-raised px-2 py-0.5 text-[11px] font-bold text-muted">{handwriting.isUppercase ? 'AA' : 'aa'}</button>
                    </div>
                    <div class="flex max-w-full gap-1 overflow-x-auto pb-1">
                        {#each handwriting.alphabet as letter}
                            <button type="button" onclick={() => handwriting.appendLetter(letter)} class="flex h-7 w-7 shrink-0 items-center justify-center rounded-lg border border-border bg-surface text-xs font-bold text-muted transition-colors hover:bg-surface-raised hover:text-accent sm:h-8 sm:w-8">{handwriting.isUppercase ? letter.toUpperCase() : letter}</button>
                        {/each}
                    </div>
                </div>
                <div class="mt-auto flex shrink-0 gap-2">
                    <button onclick={() => handwriting.clearWord()} disabled={!handwriting.composedWord} class="rounded-xl bg-surface-raised px-2.5 py-2 text-xs font-medium text-muted hover:bg-danger/20 hover:text-danger disabled:opacity-40">Clear</button>
                    <button onclick={() => handwriting.backspaceWord()} disabled={!handwriting.composedWord} class="rounded-xl bg-surface-raised px-2.5 py-2 text-xs font-medium text-muted hover:bg-surface-hover disabled:opacity-40">⌫</button>
                    <button onclick={() => handwriting.appendWordSpace()} class="rounded-xl bg-surface-raised px-2.5 py-2 text-xs font-medium text-muted hover:bg-surface-hover">Space</button>
                    <button onclick={() => handwriting.commitWord()} disabled={!handwriting.composedWord.trim()} class="flex-1 rounded-xl bg-accent py-2 text-xs font-bold text-on-accent hover:bg-accent-strong disabled:opacity-50 sm:py-3 sm:text-sm">Enter</button>
                </div>
            {/if}
        </div>
    </div>
</div>
