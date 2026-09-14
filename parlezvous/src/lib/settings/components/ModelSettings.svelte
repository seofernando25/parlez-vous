<script lang="ts">
    import { RefreshCw } from 'lucide-svelte';
    import { aiProviderState, checkAiHealth, fetchAiModels } from '$lib/state/aiProvider.svelte';
    import { AI_PROVIDER_PRESETS, providerPreset } from '$lib/ai/providers';
    import { settingsState } from '$lib/state/settings.svelte';
    let { isAndroid, onSave }: { isAndroid: boolean; onSave: () => void } = $props();
    const voices = ['F1','F2','F3','F4','F5','M1','M2','M3','M4','M5'];

    async function changeProvider() {
        const preset = providerPreset(settingsState.aiProvider);
        if (preset.baseUrl) settingsState.aiBaseUrl = preset.baseUrl;
        await onSave();
        await checkAiHealth();
    }

    async function endpointChanged() {
        await onSave();
        await checkAiHealth();
    }

    async function credentialsChanged() {
        await onSave();
        await checkAiHealth();
    }
</script>

<div>
    <div class="settings-row">
        <label for="ai-provider">Provider</label>
        <select id="ai-provider" class="settings-field" bind:value={settingsState.aiProvider} onchange={changeProvider}>
            {#each AI_PROVIDER_PRESETS as provider}<option value={provider.id}>{provider.label}</option>{/each}
        </select>
    </div>

    {#if settingsState.aiProvider === 'managed' && !isAndroid}
        <div class="settings-row">
            <span>Local runtime</span>
            <span class="managed-value">Managed automatically</span>
        </div>
    {:else}
        <div class="settings-row">
            <label for="ai-base-url">Endpoint</label>
            <input id="ai-base-url" class="settings-field" bind:value={settingsState.aiBaseUrl} onchange={endpointChanged} spellcheck="false" />
        </div>
        <div class="settings-row">
            <label for="ai-api-key">API key</label>
            <input id="ai-api-key" type="password" class="settings-field" bind:value={settingsState.aiApiKey} onchange={credentialsChanged} placeholder="Optional for local servers" autocomplete="off" />
        </div>
        <div class="settings-row">
            <label for="active-model">Model</label>
            <div class="model-field">
                <input id="active-model" class="settings-field" list="ai-models" bind:value={settingsState.activeModel} onchange={onSave} spellcheck="false" />
                <datalist id="ai-models">
                    {#if isAndroid}<option value="gemma-4-E2B-it.litertlm">On-device Gemma</option>{/if}
                    {#each aiProviderState.models as model}<option value={model}></option>{/each}
                </datalist>
                <button class="icon-button" type="button" onclick={fetchAiModels} title="Refresh models" aria-label="Refresh models"><RefreshCw size={16} /></button>
            </div>
        </div>
    {/if}

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

<style>
    .model-field { display:grid; grid-template-columns:minmax(0,1fr) 2.5rem; gap:.45rem; align-items:center; }
    .managed-value { color:var(--pv-muted); font-size:.8rem; text-align:right; }
</style>
