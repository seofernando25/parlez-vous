import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { saveSettings, settingsState } from './settings.svelte';

export interface ManagedAiStatus {
    supported: boolean;
    runtimeInstalled: boolean;
    chatModelInstalled: boolean;
    embeddingModelInstalled: boolean;
    visionModelInstalled: boolean;
    running: boolean;
    ready: boolean;
}

export const managedAiState = $state({
    supported: false,
    installed: false,
    visionInstalled: false,
    running: false,
    ready: false,
    installing: false,
    installingVision: false,
    stage: '',
    progress: -1,
    checked: false
});

let progressListenerStarted = false;

function applyStatus(status: ManagedAiStatus) {
    managedAiState.supported = status.supported;
    managedAiState.installed = status.runtimeInstalled && status.chatModelInstalled && status.embeddingModelInstalled;
    managedAiState.visionInstalled = status.visionModelInstalled;
    managedAiState.running = status.running;
    managedAiState.ready = status.ready;
    managedAiState.checked = true;
}

export async function refreshManagedAi() {
    try {
        const status = await invoke<ManagedAiStatus>('get_managed_ai_status');
        applyStatus(status);
        if (managedAiState.installed && !managedAiState.running) {
            await invoke('start_managed_ai');
            await waitUntilReady();
        }
    } catch (error) {
        managedAiState.checked = true;
        managedAiState.ready = false;
        console.warn('Managed AI unavailable:', error);
    }
}

export async function installManagedAi() {
    if (managedAiState.installing) return;
    ensureProgressListener();
    managedAiState.installing = true;
    managedAiState.stage = 'Preparing';
    managedAiState.progress = -1;
    await selectManagedAi();
    try {
        await invoke('install_managed_ai');
        await waitUntilReady();
    } finally {
        managedAiState.installing = false;
    }
}

export async function installManagedVision() {
    if (managedAiState.installingVision) return;
    ensureProgressListener();
    managedAiState.installingVision = true;
    managedAiState.stage = 'Preparing vision';
    managedAiState.progress = -1;
    try {
        await invoke('install_managed_vision');
        await waitUntilReady();
        await refreshManagedAi();
    } finally {
        managedAiState.installingVision = false;
    }
}

export async function useManagedAi() {
    await selectManagedAi();
    await refreshManagedAi();
}

async function selectManagedAi() {
    settingsState.aiProvider = 'managed';
    settingsState.aiBaseUrl = 'http://127.0.0.1:11435/v1';
    settingsState.aiApiKey = '';
    settingsState.activeModel = 'parlezvous-chat';
    settingsState.embeddingModel = 'parlezvous-embed';
    await saveSettings();
}

async function waitUntilReady() {
    for (let attempt = 0; attempt < 40; attempt += 1) {
        await new Promise(resolve => setTimeout(resolve, 350));
        const status = await invoke<ManagedAiStatus>('get_managed_ai_status');
        applyStatus(status);
        if (status.ready) return;
    }
    throw new Error('Local AI did not become ready in time.');
}

function ensureProgressListener() {
    if (progressListenerStarted) return;
    progressListenerStarted = true;
    void listen<{ stage: string; downloaded: number; total: number }>('managed_ai_progress', event => {
        managedAiState.stage = event.payload.stage;
        managedAiState.progress = event.payload.total > 0
            ? Math.round(event.payload.downloaded / event.payload.total * 100)
            : -1;
    });
}
