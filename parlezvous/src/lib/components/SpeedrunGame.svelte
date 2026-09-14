<script lang="ts">
    import { onMount } from 'svelte';
    import { LoaderCircle, Volume2, SkipForward } from 'lucide-svelte';
    import toast from 'svelte-french-toast';
    import { playSmartTTS, initTTSAudio } from '$lib/tts';
    import { settingsState } from '$lib/state/settings.svelte';

    let { isLanguageGame = false, store, initQueue, popPuzzle, stopQueue } = $props<{
        title?: string;
        subtitle?: string;
        isLanguageGame?: boolean;
        store: { isGenerating: boolean; queue: any[] };
        initQueue: () => Promise<void>;
        popPuzzle: () => Promise<any>;
        stopQueue: () => void;
    }>();

    let currentPuzzle = $state<any>(null);
    let puzzleType = $state<'keystone' | 'speedrun' | null>(null);
    let isLoading = $state(true);
    let answerInput = $state('');
    let options = $state<string[]>([]);

    const langMap: Record<string, string> = {
        english:'en', korean:'ko', japanese:'ja', arabic:'ar', bulgarian:'bg', czech:'cs', danish:'da', german:'de', greek:'el', spanish:'es', estonian:'et', finnish:'fi', french:'fr', hindi:'hi', croatian:'hr', hungarian:'hu', indonesian:'id', italian:'it', lithuanian:'lt', latvian:'lv', dutch:'nl', polish:'pl', portuguese:'pt', romanian:'ro', russian:'ru', slovak:'sk', slovenian:'sl', swedish:'sv', turkish:'tr', ukrainian:'uk', vietnamese:'vi'
    };

    function shuffle(values: string[]) {
        const result = [...values];
        for (let i = result.length - 1; i > 0; i--) {
            const j = Math.floor(Math.random() * (i + 1));
            [result[i], result[j]] = [result[j], result[i]];
        }
        return result;
    }

    async function playAudio() {
        if (!currentPuzzle) return;
        initTTSAudio();
        const text = (currentPuzzle.code_with_blank || currentPuzzle.code).replace('___BLANK___', 'blank');
        const key = currentPuzzle.language.toLowerCase();
        await playSmartTTS(text, settingsState.ttsServerUrl, undefined, langMap[key] || key, settingsState.ttsProvider);
    }

    async function nextPuzzle() {
        isLoading = true;
        answerInput = '';
        const raw = await popPuzzle();
        if (!raw) { currentPuzzle = null; puzzleType = null; isLoading = false; return; }
        try {
            currentPuzzle = JSON.parse(raw.question_data);
            puzzleType = raw.question_type;
            options = puzzleType === 'speedrun' ? shuffle([currentPuzzle.correct_answer, ...currentPuzzle.distractors]) : [];
        } catch (error) {
            console.error('Invalid puzzle JSON', error);
            isLoading = false;
            await nextPuzzle();
            return;
        }
        isLoading = false;
    }

    onMount(() => {
        void (async () => { await initQueue(); await nextPuzzle(); })();
        const poll = setInterval(() => { if (!currentPuzzle && !isLoading && store.queue.length) void nextPuzzle(); }, 1000);
        return () => { clearInterval(poll); stopQueue(); };
    });

    function checkKeystone() {
        if (!currentPuzzle) return;
        if (currentPuzzle.exact_answer.trim().toLowerCase() === answerInput.trim().toLowerCase()) { toast.success('Correct'); void nextPuzzle(); }
        else toast.error('Try again');
    }

    function checkSpeedrun(selected: string) {
        if (!currentPuzzle) return;
        if (selected === currentPuzzle.correct_answer) { toast.success('Correct'); void nextPuzzle(); }
        else toast.error('Try again');
    }
</script>

<div class="app-page app-page--medium speedrun-page">
    <header class="page-heading">
        <div><h1 class="page-title">{isLanguageGame ? 'Sprint' : 'Code'}</h1>{#if currentPuzzle}<p class="page-subtitle">{currentPuzzle.language}</p>{/if}</div>
        {#if currentPuzzle}<button class="icon-button" onclick={() => nextPuzzle()} title="Skip" aria-label="Skip"><SkipForward size={18} /></button>{/if}
    </header>

    <div class="puzzle-stage">
        {#if isLoading || (!currentPuzzle && store.isGenerating)}
            <LoaderCircle size={26} class="animate-spin text-accent" />
        {:else if currentPuzzle}
            <section class="puzzle">
                <div class="puzzle-copy">
                    {#if isLanguageGame}<button class="icon-button audio" onclick={playAudio} title="Listen" aria-label="Listen"><Volume2 size={17} /></button>{/if}
                    <pre><code>{puzzleType === 'keystone' ? currentPuzzle.code_with_blank : currentPuzzle.code}</code></pre>
                </div>

                {#if puzzleType === 'keystone'}
                    <form class="answer-form" onsubmit={(event) => { event.preventDefault(); checkKeystone(); }}>
                        <input bind:value={answerInput} placeholder="…" aria-label="Answer" autocomplete="off" spellcheck="false" />
                        <button type="submit" disabled={!answerInput.trim()}>Check</button>
                    </form>
                {:else}
                    <div class="choice-list">
                        {#each options as option}<button onclick={() => checkSpeedrun(option)}>{option}</button>{/each}
                    </div>
                {/if}
            </section>
        {:else}
            <button class="retry" onclick={() => nextPuzzle()}>Retry</button>
        {/if}
    </div>
</div>

<style>
    .speedrun-page { display:flex; min-height:100%; flex-direction:column; }
    .puzzle-stage { display:grid; flex:1; place-items:center; min-height:24rem; }
    .puzzle { width:min(100%,42rem); }
    .puzzle-copy { position:relative; border-block:1px solid var(--pv-border); padding:1.5rem .2rem; }
    .puzzle-copy pre { margin:0; overflow:auto; white-space:pre-wrap; color:var(--pv-foreground); font-family:ui-monospace,SFMono-Regular,Menlo,monospace; font-size:.92rem; line-height:1.7; }
    .audio { position:absolute; top:.65rem; right:0; }
    .answer-form { display:flex; gap:.6rem; margin-top:1.3rem; }
    .answer-form input { min-width:0; flex:1; border:1px solid var(--pv-border); border-radius:.8rem; background:var(--pv-surface); color:var(--pv-foreground); padding:.75rem .85rem; font:inherit; font-family:ui-monospace,monospace; outline:none; }
    .answer-form input:focus { border-color:var(--pv-accent); }
    .answer-form button { border:0; border-radius:.8rem; background:var(--pv-accent); color:var(--pv-on-accent); padding:.75rem 1rem; font-weight:800; cursor:pointer; }
    .answer-form button:disabled { opacity:.35; }
    .choice-list { display:grid; gap:.5rem; margin-top:1.2rem; }
    .choice-list button { border:1px solid var(--pv-border); border-radius:.8rem; background:transparent; color:var(--pv-foreground); padding:.8rem .9rem; text-align:left; font:inherit; font-size:.86rem; cursor:pointer; }
    .choice-list button:hover { background:var(--pv-surface-raised); border-color:var(--pv-border-strong); }
    .retry { border:0; background:transparent; color:var(--pv-accent-strong); font-weight:750; cursor:pointer; }
    @media (min-width:720px) { .choice-list { grid-template-columns:repeat(2,minmax(0,1fr)); } }
</style>
