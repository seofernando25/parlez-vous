<script lang="ts">
    import { onDestroy, onMount } from 'svelte';
    import AiGate from '$lib/components/AiGate.svelte';
    import { invoke, convertFileSrc } from '@tauri-apps/api/core';
    import { ImagePlus, Send, RotateCcw, LoaderCircle } from 'lucide-svelte';
    import { settingsState, loadSettings } from '$lib/state/settings.svelte';
    import { timeTracker } from '$lib/state/timeTracker.svelte';
    import toast from 'svelte-french-toast';
    import { marked } from 'marked';

    let userDescription = $state('');
    let isGrading = $state(false);
    let feedback = $state<{ correction?: string | null; comment: string } | null>(null);
    let selectedImageUri = $state<string | null>(null);
    let displayImageUrl = $state<string | null>(null);

    onMount(async () => { await loadSettings(); timeTracker.startTracking(); });
    onDestroy(() => { timeTracker.flushTime(); timeTracker.stopTracking(); });

    async function pickImage() {
        try {
            if (/Android/i.test(navigator.userAgent)) {
                const response = await invoke<{ path: string }>('plugin:litert|pick_gallery_image');
                if (!response.path) return;
                selectedImageUri = response.path;
                displayImageUrl = convertFileSrc(response.path);
                feedback = null;
                userDescription = '';
                return;
            }

            const fileInput = document.createElement('input');
            fileInput.type = 'file';
            fileInput.accept = 'image/*';
            fileInput.onchange = () => {
                const file = fileInput.files?.[0];
                if (!file) return;
                displayImageUrl = URL.createObjectURL(file);
                const reader = new FileReader();
                reader.onloadend = () => {
                    selectedImageUri = reader.result as string;
                    feedback = null;
                    userDescription = '';
                };
                reader.readAsDataURL(file);
            };
            fileInput.click();
        } catch (error) {
            if (error !== 'Image selection cancelled by user') toast.error(`Could not select image: ${error}`);
        }
    }

    async function submitDescription() {
        if (!userDescription.trim() || !selectedImageUri) return;
        isGrading = true;
        try {
            const prompt = `I am practicing ${settingsState.targetLanguage}. I am describing the attached image. Here is my description: "${userDescription}". Please correct my grammar and tell me if I successfully described the visual contents.`;
            const response = await invoke<{ idealized_correction?: string | null; response: string }>('chat_with_avatar', {
                history: [{ role: 'user', content: prompt }],
                model: settingsState.activeModel,
                language: settingsState.targetLanguage,
                activeTextbook: null,
                activePage: null,
                activeTheme: null,
                activeSubtheme: null,
                audioBase64: null,
                imageUri: selectedImageUri
            });
            feedback = { correction: response.idealized_correction, comment: response.response };
        } catch (error) { toast.error(`Review failed: ${error}`); }
        finally { isGrading = false; }
    }

    function resetTask() {
        selectedImageUri = null;
        displayImageUrl = null;
        userDescription = '';
        feedback = null;
    }
</script>

<AiGate feature="Vision practice" capability="vision">
<div class="app-page app-page--medium">
    <header class="page-heading"><h1 class="page-title">Vision</h1></header>

    <div class="vision-layout">
        <section class="image-pane">
            {#if displayImageUrl}
                <img src={displayImageUrl} alt="Selected practice reference" />
                <button class="icon-button image-action" onclick={pickImage} title="Choose another image" aria-label="Choose another image"><ImagePlus size={18} /></button>
            {:else}
                <button class="image-empty" onclick={pickImage} aria-label="Choose an image"><ImagePlus size={30} /><span>Choose image</span></button>
            {/if}
        </section>

        <section class="description-pane" class:disabled={!displayImageUrl}>
            <textarea bind:value={userDescription} placeholder="Describe what you see…" aria-label="Image description" disabled={!displayImageUrl}></textarea>
            {#if !feedback}
                <button class="send-button" onclick={submitDescription} disabled={isGrading || !userDescription.trim() || !selectedImageUri} title="Review description" aria-label="Review description">
                    {#if isGrading}<LoaderCircle size={19} class="animate-spin" />{:else}<Send size={19} />{/if}
                </button>
            {/if}

            {#if feedback}
                <div class="feedback-pane">
                    {#if feedback.correction && feedback.correction !== 'null'}<p class="correction">{feedback.correction}</p>{/if}
                    <div class="feedback-copy">{@html marked.parse(feedback.comment)}</div>
                    <button class="icon-button" onclick={resetTask} title="Start over" aria-label="Start over"><RotateCcw size={17} /></button>
                </div>
            {/if}
        </section>
    </div>
</div>
</AiGate>

<style>
    .vision-layout { display:grid; gap:1rem; }
    .image-pane { position:relative; min-height:20rem; overflow:hidden; border:1px solid var(--pv-border); border-radius:1rem; background:var(--pv-surface); }
    .image-pane img { display:block; width:100%; height:100%; min-height:20rem; max-height:32rem; object-fit:cover; }
    .image-action { position:absolute; top:.65rem; right:.65rem; background:color-mix(in oklch,var(--pv-surface) 88%,transparent); backdrop-filter:blur(12px); }
    .image-empty { display:flex; width:100%; min-height:20rem; flex-direction:column; align-items:center; justify-content:center; gap:.65rem; border:0; background:transparent; color:var(--pv-subtle); cursor:pointer; }
    .image-empty span { font-size:.78rem; font-weight:700; }
    .description-pane { position:relative; min-width:0; }
    .description-pane.disabled { opacity:.55; }
    .description-pane textarea { width:100%; min-height:14rem; box-sizing:border-box; resize:vertical; border:1px solid var(--pv-border); border-radius:1rem; background:var(--pv-surface); color:var(--pv-foreground); padding:1rem 3.5rem 1rem 1rem; font:inherit; line-height:1.6; outline:none; }
    .description-pane textarea:focus { border-color:var(--pv-accent); }
    .send-button { position:absolute; top:.65rem; right:.65rem; display:grid; width:2.6rem; height:2.6rem; place-items:center; border:0; border-radius:.8rem; background:var(--pv-accent); color:var(--pv-on-accent); cursor:pointer; }
    .send-button:disabled { opacity:.35; cursor:not-allowed; }
    .feedback-pane { margin-top:1rem; border-top:1px solid var(--pv-border); padding-top:1rem; }
    .correction { margin:0 0 .8rem; border-left:2px solid var(--pv-success); padding-left:.8rem; color:var(--pv-foreground); line-height:1.55; }
    .feedback-copy { color:var(--pv-muted); font-size:.86rem; line-height:1.6; }
    .feedback-pane .icon-button { margin-top:.75rem; }
    @media (min-width:900px) { .vision-layout { grid-template-columns:minmax(0,1.2fr) minmax(18rem,.8fr); align-items:start; gap:1.5rem; } .description-pane textarea { min-height:20rem; } }
</style>
