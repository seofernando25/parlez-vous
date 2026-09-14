import { invoke } from '@tauri-apps/api/core';
import { settingsState } from '$lib/state/settings.svelte.ts';
import { isAndroidTauri } from '$lib/platform';
import { isLiteRtModel } from '$lib/ai/capabilities';

export const ollamaState = $state({
    isHealthy: true,
    hasChecked: false,
    models: [] as string[]
});

export async function checkOllamaHealth() {
    try {
        const isHealthy = await invoke<boolean>('check_ollama_health');
        ollamaState.isHealthy = isHealthy;
        ollamaState.hasChecked = true;
        
        if (isHealthy) {
            await fetchOllamaModels();
        }
    } catch (e) {
        console.error('Failed to check Ollama health:', e);
        ollamaState.isHealthy = false;
        ollamaState.hasChecked = true;
    }
}

export async function fetchOllamaModels() {
    try {
        const rawModels = await invoke<string[]>('list_ollama_models');
        const models = rawModels;
        ollamaState.models = models;
        const isAndroid = isAndroidTauri();
        const isLitert = isLiteRtModel(settingsState.activeModel);

        if (rawModels.includes(settingsState.activeModel) || (isAndroid && isLitert)) {
            // Already a valid Ollama model
        } else if (models.length > 0) {
            // It's not a valid Ollama model.
            // Overwrite if it is not a Litert model, or if we are on Desktop (meaning Litert is invalid here)
            if (!isLitert || !isAndroid) {
                settingsState.activeModel = models[0];
                import('$lib/state/settings.svelte.ts').then(m => m.saveSettings());
            }
        } else if (isAndroid) {
            if (!isLitert) {
                settingsState.activeModel = "gemma-4-E2B-it.litertlm";
                import('$lib/state/settings.svelte.ts').then(m => m.saveSettings());
            }
        }
    } catch (e) {
        console.error('Failed to list Ollama models:', e);
    }
}
