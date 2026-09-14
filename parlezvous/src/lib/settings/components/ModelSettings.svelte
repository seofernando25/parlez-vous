<script lang="ts">
    import { ollamaState, fetchOllamaModels } from '$lib/state/ollama.svelte';
    import { settingsState } from '$lib/state/settings.svelte';
    let { isAndroid, onSave }: { isAndroid: boolean; onSave: () => void } = $props();
    const voices = ['F1','F2','F3','F4','F5','M1','M2','M3','M4','M5'];
</script>

<div>
    <div class="settings-row">
        <label for="active-model">Model</label>
        <div class="flex gap-2">
            <select id="active-model" class="settings-field" bind:value={settingsState.activeModel} onchange={onSave}>
                {#if isAndroid}<optgroup label="On device"><option value="gemma-4-E2B-it.litertlm">Gemma 4 E2B</option></optgroup>{/if}
                {#if ollamaState.models.length}<optgroup label="Ollama">{#each ollamaState.models as model}<option value={model}>{model}</option>{/each}</optgroup>{/if}
            </select>
            {#if ollamaState.models.length === 0}<button class="icon-button" type="button" onclick={fetchOllamaModels} title="Refresh models" aria-label="Refresh models">↻</button>{/if}
        </div>
    </div>
    {#if isAndroid}
        <div class="settings-row">
            <label for="tts-provider">Speech</label>
            <select id="tts-provider" class="settings-field" bind:value={settingsState.ttsProvider} onchange={onSave}>
                <option value="auto">Auto</option><option value="supertonic">On device</option><option value="server">Server</option>
            </select>
        </div>
        <div class="settings-row">
            <label for="voice-style">Voice</label>
            <select id="voice-style" class="settings-field" bind:value={settingsState.supertonicVoiceStyle} onchange={onSave}>
                {#each voices as voice}<option value={`voice_styles/${voice}.json`}>{voice}</option>{/each}
            </select>
        </div>
    {/if}

</div>
