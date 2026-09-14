<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { onDestroy, onMount } from 'svelte';
    import { CURRICULUM_TIERS, getTierFromXP } from '$lib/curriculum';
    import { settingsState } from '$lib/state/settings.svelte';
    import { timeTracker } from '$lib/state/timeTracker.svelte';
    import { AvatarChatController } from '$lib/avatar/chat.svelte';
    import { HandwritingController } from '$lib/avatar/handwriting/controller.svelte';
    import { AvatarStageController } from '$lib/avatar/stage.svelte';
    import { TextbookController } from '$lib/avatar/textbooks.svelte';
    import type { ViewMode } from '$lib/avatar/types';
    import { VoiceCaptureController } from '$lib/avatar/voice.svelte';
    import AvatarStagePanel from '$lib/avatar/components/AvatarStagePanel.svelte';
    import ChatPanel from '$lib/avatar/components/ChatPanel.svelte';
    import ReviewModal from '$lib/avatar/components/ReviewModal.svelte';
    import TextbookViewport from '$lib/avatar/components/TextbookViewport.svelte';
    import ViewModeControls from '$lib/avatar/components/ViewModeControls.svelte';

    let viewMode = $state<ViewMode>('split');
    let mapFollowMode = $state(true);
    let muteTts = $state(false);
    let activeThemeId = $state<string | null>(null);
    let reviewThemeCandidate = $state<{ id: string; name: string } | null>(null);
    let showReviewModal = $state(false);

    const textbooks = new TextbookController();
    const stage = new AvatarStageController(
        () => settingsState.activeVrm,
        () => settingsState.ttsServerUrl,
        () => settingsState.targetLanguage,
        () => settingsState.ttsProvider
    );
    const chat = new AvatarChatController(
        () => ({ activeTextbook: textbooks.active, activePage: textbooks.activePage, activeThemeId, mapFollowMode }),
        () => viewMode,
        text => stage.speak(text, muteTts)
    );
    const voice = new VoiceCaptureController(
        () => settingsState.activeModel,
        () => settingsState.asrServerUrl,
        text => chat.setInput(text),
        audio => chat.send(audio)
    );
    const handwriting = new HandwritingController(
        () => settingsState.targetLanguage,
        text => chat.appendInput(text)
    );

    function setViewMode(mode: ViewMode) {
        viewMode = mode;
        setTimeout(stage.resize, 50);
    }

    async function loadCurriculumReview() {
        try {
            const curriculum: any = await invoke('get_curriculum', { language: settingsState.targetLanguage });
            activeThemeId = curriculum.active_theme_id;
            const tier = getTierFromXP(curriculum.total_xp);
            const today = new Date().toDateString();
            if (tier <= 1 || localStorage.getItem('lastReviewPromptDate') === today) return;
            const past = CURRICULUM_TIERS.filter(item => item.level < tier);
            if (!past.length) return;
            const randomTier = past[Math.floor(Math.random() * past.length)];
            reviewThemeCandidate = randomTier.themes[Math.floor(Math.random() * randomTier.themes.length)];
            showReviewModal = true;
            localStorage.setItem('lastReviewPromptDate', today);
        } catch (error) {
            console.error('No curriculum found', error);
        }
    }

    function acceptReview(theme: { id: string; name: string }) {
        activeThemeId = theme.id;
        mapFollowMode = true;
        showReviewModal = false;
    }

    onMount(async () => {
        await Promise.all([textbooks.load(), chat.load(), loadCurriculumReview()]);
        timeTracker.startTracking();
    });

    $effect(() => {
        settingsState.targetLanguage;
        handwriting.syncLanguage();
    });

    $effect(() => {
        handwriting.show;
        [0, 80, 160, 240, 320].forEach(delay => setTimeout(stage.resize, delay));
    });

    onDestroy(() => {
        timeTracker.flushTime();
        timeTracker.stopTracking();
        voice.dispose();
    });
</script>

<div class="tutor-shell">
    {#if showReviewModal && reviewThemeCandidate}
        <ReviewModal theme={reviewThemeCandidate} onSkip={() => showReviewModal = false} onAccept={acceptReview} />
    {/if}

    <div class="tutor-view-control"><ViewModeControls mode={viewMode} onChange={setViewMode} /></div>

    {#if stage.isLoading}
        <div class="tutor-loading"><div class="h-9 w-9 animate-spin rounded-full border-2 border-accent border-t-transparent"></div></div>
    {/if}

    <div class="stage-column {viewMode === 'avatar' ? 'avatar-only' : handwriting.show ? 'handwriting-open' : ''} {viewMode === 'chat' ? 'hidden' : ''}">
        <AvatarStagePanel
            {stage} {voice} {viewMode}
            isChatting={chat.isChatting}
            hasTextbook={Boolean(textbooks.active)}
        />
        <TextbookViewport {textbooks} />
    </div>

    <ChatPanel
        {chat} {voice} {handwriting} {textbooks} {viewMode} {mapFollowMode} {muteTts}
        onToggleMap={() => mapFollowMode = !mapFollowMode}
        onToggleMute={() => muteTts = !muteTts}
    />
</div>

<style>
    .tutor-shell { position:relative; display:flex; width:100%; height:100%; min-height:0; flex-direction:column; gap:.5rem; overflow:hidden; padding:.5rem; }
    .stage-column { display:flex; min-width:0; min-height:0; height:40vh; flex:0 0 auto; flex-direction:column; gap:.5rem; transition:height 180ms ease; }
    .stage-column.avatar-only { height:100%; flex:1; }
    .stage-column.handwriting-open { height:22vh; }
    .tutor-view-control { position:absolute; z-index:30; top:.55rem; left:50%; transform:translateX(-50%); }
    .tutor-loading { position:absolute; inset:0; z-index:40; display:grid; place-items:center; background:color-mix(in oklch,var(--pv-canvas) 78%,transparent); backdrop-filter:blur(8px); }
    @media (min-width:768px) { .tutor-shell { flex-direction:row; gap:0; padding:0; } .stage-column { width:44%; max-width:44%; height:auto; flex:0 0 44%; gap:0; } .stage-column.avatar-only { width:100%; max-width:none; flex:1; } .stage-column.handwriting-open { height:auto; } .tutor-view-control { top:.6rem; } }
</style>
