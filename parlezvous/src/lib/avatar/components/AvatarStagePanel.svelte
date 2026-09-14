<script lang="ts">
    import { onDestroy, onMount } from 'svelte';
    import { Mic, Square } from 'lucide-svelte';
    import { setPlaybackRate } from '$lib/tts';
    import type { AvatarStageController } from '../stage.svelte';
    import type { VoiceCaptureController } from '../voice.svelte';
    import type { ViewMode } from '../types';

    let { stage, voice, viewMode, isChatting, hasTextbook }: {
        stage: AvatarStageController; voice: VoiceCaptureController; viewMode: ViewMode;
        isChatting: boolean; hasTextbook: boolean;
    } = $props();
    let container: HTMLDivElement;

    onMount(() => { void stage.mount(container); });
    onDestroy(() => stage.dispose());
    $effect(() => { viewMode; [0, 80, 160, 240].forEach(delay => setTimeout(stage.resize, delay)); });
</script>

<div class="stage-panel {hasTextbook ? 'with-textbook' : ''}">
    <div bind:this={container} class="stage-canvas"></div>

    <div class="stage-status">
        <span class="status-dot" class:ready={Boolean(stage.currentVrm)} title={stage.currentVrm ? 'Avatar ready' : 'Loading avatar'}></span>
        <select aria-label="Speech speed" onchange={(event) => setPlaybackRate(parseFloat(event.currentTarget.value))}>
            <option value="1">1×</option><option value="0.75">0.75×</option><option value="0.5">0.5×</option>
        </select>
    </div>
    {#if viewMode === 'avatar'}
        <div class="voice-control">
            {#if isChatting}<div class="activity-dots"><span></span><span></span><span></span></div>{/if}
            <button type="button" onclick={voice.toggle} disabled={isChatting} class:active={voice.isActive} title={voice.isActive ? 'Stop voice' : 'Voice'} aria-label={voice.isActive ? 'Stop voice' : 'Voice'}>
                {#if voice.isActive}<Square size={19} fill="currentColor" />{:else}<Mic size={21} />{/if}
            </button>
        </div>
    {/if}
</div>

<style>
    .stage-panel { position:relative; min-height:0; flex:1; overflow:hidden; border:1px solid var(--pv-border); border-radius:1rem; background:var(--pv-surface); }
    .stage-panel.with-textbook { max-height:50%; }
    .stage-canvas { width:100%; height:100%; }
    .stage-status { position:absolute; top:.55rem; left:.55rem; display:flex; align-items:center; gap:.35rem; }
    .status-dot { width:.55rem; height:.55rem; border-radius:999px; background:var(--pv-warning); }
    .status-dot.ready { background:var(--pv-success); }
    .stage-status select { border:0; border-radius:.55rem; background:color-mix(in oklch,var(--pv-surface) 82%,transparent); color:var(--pv-muted); padding:.35rem .45rem; font-size:.7rem; outline:none; backdrop-filter:blur(10px); }
    .voice-control { position:absolute; bottom:1rem; left:50%; transform:translateX(-50%); }
    .voice-control > button { display:grid; width:3.5rem; height:3.5rem; place-items:center; border:1px solid var(--pv-border-strong); border-radius:999px; background:color-mix(in oklch,var(--pv-surface) 86%,transparent); color:var(--pv-foreground); cursor:pointer; backdrop-filter:blur(10px); }
    .voice-control > button.active { border-color:color-mix(in oklch,var(--pv-danger) 45%,var(--pv-border)); color:var(--pv-danger); }
    .voice-control > button:disabled { opacity:.35; cursor:not-allowed; }
    .activity-dots { position:absolute; bottom:calc(100% + .5rem); left:50%; display:flex; gap:.2rem; transform:translateX(-50%); padding:.4rem .55rem; border-radius:999px; background:color-mix(in oklch,var(--pv-surface) 86%,transparent); }
    .activity-dots span { width:.3rem; height:.3rem; border-radius:999px; background:var(--pv-accent-strong); animation:bounce 1s infinite alternate; }
    .activity-dots span:nth-child(2) { animation-delay:.12s; }.activity-dots span:nth-child(3) { animation-delay:.24s; }
    @keyframes bounce { to { transform:translateY(-2px); } }
    @media (min-width:768px) { .stage-panel { border-radius:0; border-block:0; border-left:0; } }
</style>
