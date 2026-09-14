import { invoke } from '@tauri-apps/api/core';
import { settingsState } from './settings.svelte';

export const aiProviderState = $state({
    isHealthy: true,
    hasChecked: false,
    models: [] as string[]
});

export async function checkAiHealth() {
    try {
        aiProviderState.isHealthy = await invoke<boolean>('check_ai_health');
        aiProviderState.hasChecked = true;
        if (aiProviderState.isHealthy) await fetchAiModels();
    } catch (error) {
        console.error('Failed to check AI endpoint:', error);
        aiProviderState.isHealthy = false;
        aiProviderState.hasChecked = true;
    }
}

export async function fetchAiModels() {
    try {
        aiProviderState.models = await invoke<string[]>('list_ai_models');
        if (!settingsState.activeModel.trim() && aiProviderState.models.length) {
            settingsState.activeModel = aiProviderState.models[0];
            const { saveSettings } = await import('./settings.svelte');
            await saveSettings();
        }
    } catch (error) {
        console.error('Failed to list AI models:', error);
        aiProviderState.models = [];
    }
}
