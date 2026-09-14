<script lang="ts">
    import { settingsState } from '$lib/state/settings.svelte';
    import { ollamaState } from '$lib/state/ollama.svelte';
    import type { SettingsController } from '../controller.svelte';
    import SystemStatus from './SystemStatus.svelte';
    let { controller, onSave }: { controller: SettingsController; onSave: () => void } = $props();
</script>

<div>
    <div class="settings-row"><label for="ollama-url">Ollama URL</label><input id="ollama-url" class="settings-field" bind:value={settingsState.ollamaServerUrl} onchange={onSave} /></div>
    <div class="settings-row"><label for="asr-url">ASR URL</label><input id="asr-url" class="settings-field" bind:value={settingsState.asrServerUrl} onchange={onSave} /></div>
    <div class="settings-row"><label for="tts-url">TTS URL</label><input id="tts-url" class="settings-field" bind:value={settingsState.ttsServerUrl} onchange={onSave} /></div>
    <div class="settings-row"><label for="embedding-model">Embedding model</label><select id="embedding-model" class="settings-field" bind:value={settingsState.embeddingModel} onchange={onSave}>{#each ollamaState.models as model}<option value={model}>{model}</option>{/each}</select></div>
    <div class="settings-row"><label for="hf-token">HuggingFace token</label><input id="hf-token" type="password" class="settings-field" bind:value={settingsState.huggingFaceToken} onchange={onSave} placeholder="hf_…" /></div>
    {#if controller.isAndroid}
        <div class="settings-row"><label for="accelerator">Accelerator</label><select id="accelerator" class="settings-field" bind:value={settingsState.litertAccelerator} onchange={onSave}><option>Auto</option><option>CPU</option><option>GPU</option><option>NPU</option></select></div>
        <div class="settings-row"><label for="max-tokens">Max tokens</label><input id="max-tokens" type="number" min="256" max="4096" step="128" class="settings-field" bind:value={settingsState.litertMaxTokens} onchange={onSave} /></div>
    {/if}
    <div class="mt-6"><SystemStatus visible={true} /></div>
</div>
