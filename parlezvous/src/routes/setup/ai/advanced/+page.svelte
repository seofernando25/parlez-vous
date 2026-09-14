<script lang="ts">
    import { goto } from '$app/navigation';
    import { page } from '$app/stores';
    import { ArrowLeft, Check, LoaderCircle, RefreshCw } from 'lucide-svelte';
    import { AI_PROVIDER_PRESETS, providerPreset, type AiProviderPreset } from '$lib/ai/providers';
    import { aiProviderState, checkAiHealth, fetchAiModels } from '$lib/state/aiProvider.svelte';
    import { saveSettings, settingsState } from '$lib/state/settings.svelte';

    const providers = AI_PROVIDER_PRESETS.filter(provider => provider.id !== 'managed');
    let returnTo = $derived($page.url.searchParams.get('return') || '/map');
    let provider = $state<AiProviderPreset>(settingsState.aiProvider === 'managed' ? 'lmstudio' : settingsState.aiProvider as AiProviderPreset);
    let baseUrl = $state(settingsState.aiProvider === 'managed' ? 'http://localhost:1234/v1' : settingsState.aiBaseUrl);
    let apiKey = $state(settingsState.aiProvider === 'managed' ? '' : settingsState.aiApiKey);
    let model = $state(settingsState.aiProvider === 'managed' ? '' : settingsState.activeModel);
    let connecting = $state(false);
    let error = $state('');
    let initialized = $state(false);

    $effect(() => {
        if (initialized || !settingsState.isLoaded) return;
        provider = settingsState.aiProvider === 'managed' ? 'lmstudio' : settingsState.aiProvider as AiProviderPreset;
        const preset = providerPreset(provider);
        baseUrl = settingsState.aiProvider === 'managed' ? (preset.baseUrl || '') : settingsState.aiBaseUrl;
        apiKey = settingsState.aiProvider === 'managed' ? '' : settingsState.aiApiKey;
        model = settingsState.aiProvider === 'managed' ? '' : settingsState.activeModel;
        initialized = true;
    });

    function changeProvider() {
        const preset = providerPreset(provider);
        if (preset.baseUrl) baseUrl = preset.baseUrl;
        if (preset.local) apiKey = '';
        error = '';
    }

    async function discover() {
        error = '';
        await applyDraft();
        await checkAiHealth();
        if (!aiProviderState.isHealthy) {
            error = 'Could not reach that endpoint.';
            return;
        }
        await fetchAiModels();
        if (!model && aiProviderState.models.length) model = aiProviderState.models[0];
    }

    async function connect() {
        connecting = true;
        error = '';
        try {
            await applyDraft();
            await checkAiHealth();
            if (!aiProviderState.isHealthy) {
                error = 'Could not connect. Check that the service is running and the endpoint is correct.';
                return;
            }
            if (!model && aiProviderState.models.length) model = aiProviderState.models[0];
            if (!model.trim()) {
                error = 'Choose or enter a model first.';
                return;
            }
            settingsState.activeModel = model.trim();
            await saveSettings();
            await goto(returnTo);
        } finally {
            connecting = false;
        }
    }

    async function applyDraft() {
        settingsState.aiProvider = provider;
        settingsState.aiBaseUrl = baseUrl.trim();
        settingsState.aiApiKey = apiKey.trim();
        if (model.trim()) settingsState.activeModel = model.trim();
        await saveSettings();
    }
</script>

<div class="setup-page">
    <header>
        <a href={`/setup/ai?return=${encodeURIComponent(returnTo)}`} class="back" aria-label="Back"><ArrowLeft size={18} /></a>
        <div><h1>Connect AI</h1><p>Use a service already running on this computer or an API you trust.</p></div>
    </header>

    <form class="connection" onsubmit={(event) => { event.preventDefault(); void connect(); }}>
        <label>
            <span>Provider</span>
            <select bind:value={provider} onchange={changeProvider}>
                {#each providers as item}<option value={item.id}>{item.label}</option>{/each}
            </select>
        </label>

        <label>
            <span>Endpoint</span>
            <input bind:value={baseUrl} spellcheck="false" autocomplete="url" />
        </label>

        <label>
            <span>API key <small>optional for local services</small></span>
            <input bind:value={apiKey} type="password" autocomplete="off" placeholder={provider === 'openrouter' || provider === 'openai' ? 'Required' : 'Optional'} />
        </label>

        <label>
            <span>Model</span>
            <div class="model-row">
                <input bind:value={model} list="setup-ai-models" spellcheck="false" placeholder="Choose or enter a model id" />
                <button class="icon-button" type="button" onclick={discover} aria-label="Discover models" title="Discover models"><RefreshCw size={17} /></button>
            </div>
            <datalist id="setup-ai-models">{#each aiProviderState.models as item}<option value={item}></option>{/each}</datalist>
        </label>

        {#if error}<p class="error">{error}</p>{/if}
        {#if aiProviderState.hasChecked && aiProviderState.isHealthy && !error}<p class="connected"><Check size={15} /> Connected</p>{/if}

        <button class="primary" type="submit" disabled={connecting || !baseUrl.trim()}>
            {#if connecting}<LoaderCircle size={17} class="animate-spin" />{:else}<Check size={17} />{/if}
            Use this AI
        </button>
    </form>

    <p class="privacy">ParlezVous talks to this endpoint directly. API credentials stay in the app’s local settings.</p>
</div>

<style>
    .setup-page { width:min(100%,39rem); margin:0 auto; padding:clamp(1.25rem,4vw,3rem); }
    header { display:flex; align-items:flex-start; gap:.9rem; margin-bottom:2rem; }
    h1 { margin:0; font-size:clamp(1.7rem,4vw,2.2rem); letter-spacing:-.04em; }
    header p { margin:.35rem 0 0; color:var(--pv-muted); line-height:1.5; }
    .back { display:grid; width:2.3rem; height:2.3rem; flex:0 0 2.3rem; place-items:center; border-radius:.7rem; color:var(--pv-muted); }
    .connection { display:grid; gap:0; border-block:1px solid var(--pv-border); }
    label { display:grid; grid-template-columns:9rem minmax(0,1fr); align-items:center; gap:1rem; min-height:4.25rem; border-bottom:1px solid var(--pv-border); }
    label:last-of-type { border-bottom:0; }
    label > span { font-size:.86rem; font-weight:700; }
    label small { display:block; margin-top:.12rem; color:var(--pv-subtle); font-size:.65rem; font-weight:500; }
    input,select { width:100%; min-width:0; box-sizing:border-box; border:1px solid var(--pv-border-strong); border-radius:.7rem; background:var(--pv-surface); color:var(--pv-foreground); padding:.62rem .72rem; font:inherit; font-size:.82rem; outline:none; }
    input:focus,select:focus { border-color:var(--pv-accent); }
    .model-row { display:grid; grid-template-columns:minmax(0,1fr) 2.5rem; gap:.45rem; align-items:center; }
    .icon-button { display:grid; width:2.5rem; height:2.5rem; place-items:center; border:1px solid var(--pv-border); border-radius:.7rem; background:var(--pv-surface-raised); color:var(--pv-muted); cursor:pointer; }
    .primary { display:flex; justify-self:start; align-items:center; gap:.45rem; margin-top:1.25rem; border:0; border-radius:.75rem; background:var(--pv-accent); color:var(--pv-on-accent); padding:.7rem .9rem; font:inherit; font-size:.8rem; font-weight:800; cursor:pointer; }
    .primary:disabled { opacity:.5; cursor:default; }
    .error,.connected { display:flex; align-items:center; gap:.35rem; margin:.8rem 0 -.25rem; font-size:.75rem; }
    .error { color:var(--pv-danger); }.connected { color:var(--pv-success); }
    .privacy { margin:1rem 0 0; color:var(--pv-subtle); font-size:.7rem; line-height:1.5; }
    @media (max-width:600px) { label { grid-template-columns:1fr; gap:.4rem; padding:.8rem 0; } }
</style>
