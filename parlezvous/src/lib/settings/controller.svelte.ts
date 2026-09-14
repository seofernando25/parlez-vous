import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import toast from 'svelte-french-toast';
import { getThemes } from '$lib/coding/utils';
import { loadNotificationSettings } from '$lib/services/notifications.svelte';
import { fetchAiModels } from '$lib/state/aiProvider.svelte';
import { refreshManagedAi } from '$lib/state/managedAi.svelte';
import { settingsState } from '$lib/state/settings.svelte';
import { loadSettings, saveSettings } from '$lib/state/settings.svelte';
import { isAndroidTauri } from '$lib/platform';

export class SettingsController {
    isSaving = $state(false);
    isAndroid = $state(false);
    showAdvancedMobile = $state(false);
    codingCategories = $state<string[]>([]);
    litertExists = $state(false);
    litertDownloading = $state(false);
    litertProgress = $state(-1);
    supertonicReady = $state(false);
    supertonicDownloading = $state(false);
    supertonicProgress = $state(-1);
    tokenizerExists = $state(false);
    tokenizerDownloading = $state(false);
    private unlisteners: Promise<UnlistenFn>[] = [];

    async init() {
        this.isAndroid = isAndroidTauri();
        loadNotificationSettings();
        await loadSettings();
        await Promise.all([settingsState.aiProvider === 'managed' ? refreshManagedAi() : fetchAiModels(), this.checkLitert(), this.checkSupertonic(), this.checkTokenizer()]);
        getThemes().then(themes => this.codingCategories = Object.keys(themes))
            .catch(error => console.error('Failed to load coding themes', error));
        this.unlisteners = [
            listen('download_progress', event => this.handleProgress('litert', event.payload as any)),
            listen('supertonic_download_progress', event => this.handleProgress('supertonic', event.payload as any))
        ];
    }

    dispose() { for (const unlisten of this.unlisteners) unlisten.then(fn => fn()); }

    async save() {
        this.isSaving = true;
        try { await saveSettings(); }
        catch (error) { toast.error(`Could not save settings: ${error}`); }
        finally { this.isSaving = false; }
    }

    async checkLitert() {
        try {
            const result = await invoke<{ exists: boolean; isDownloading: boolean }>('plugin:litert|check_model_exists', { payload: { modelPath: 'gemma-4-E2B-it.litertlm' } });
            this.litertExists = result.exists;
            this.litertDownloading = result.isDownloading;
        } catch (error) { console.error('Failed to check LiteRT model:', error); }
    }

    async downloadLitert() {
        try { await saveSettings(); } catch (error) { console.warn('Failed to save HF token:', error); }
        this.litertDownloading = true; this.litertProgress = 0;
        try {
            const { settingsState } = await import('$lib/state/settings.svelte');
            await invoke('plugin:litert|download_model', { payload: { modelPath: 'gemma-4-E2B-it.litertlm', token: settingsState.huggingFaceToken } });
            toast.success('Download queued in background!');
        } catch (error) { this.litertDownloading = false; toast.error(`Failed to start LiteRT download: ${error}`); }
    }

    async purgeLitert() {
        try {
            await invoke('plugin:litert|purge_model', { payload: { modelPath: 'gemma-4-E2B-it.litertlm' } });
            toast.success('Model purged successfully'); await this.checkLitert();
        } catch (error) { toast.error(`Failed to purge model: ${error}`); }
    }

    async checkSupertonic() {
        try {
            const result = await invoke<{ exists: boolean; isDownloading: boolean }>('plugin:supertonic|is_supertonic_ready');
            this.supertonicReady = result.exists; this.supertonicDownloading = result.isDownloading;
        } catch (error) { console.error('Failed to check Supertonic model:', error); }
    }

    async downloadSupertonic() {
        this.supertonicDownloading = true; this.supertonicProgress = 0;
        try {
            await invoke('plugin:supertonic|download_supertonic_models', { payload: { modelPath: '', downloadUrl: '', token: null } });
            toast.success('Supertonic models download queued!');
        } catch (error) { this.supertonicDownloading = false; toast.error(`Failed to start Supertonic download: ${error}`); }
    }

    async purgeSupertonic() {
        try { await invoke('plugin:supertonic|purge_supertonic_models'); toast.success('Supertonic models purged'); await this.checkSupertonic(); }
        catch (error) { toast.error(`Failed to purge Supertonic models: ${error}`); }
    }

    async checkTokenizer() {
        try { this.tokenizerExists = await invoke<boolean>('check_tokenizer_exists'); }
        catch (error) { console.error('Failed to check tokenizer:', error); }
    }

    async downloadTokenizer() {
        this.tokenizerDownloading = true;
        try { await invoke('download_tokenizer'); toast.success('Tokenizer downloaded!'); await this.checkTokenizer(); }
        catch (error) { toast.error(`Failed to download tokenizer: ${error}`); }
        finally { this.tokenizerDownloading = false; }
    }

    async purgeTokenizer() {
        try { await invoke('delete_tokenizer'); toast.success('Tokenizer purged'); await this.checkTokenizer(); }
        catch (error) { toast.error(`Failed to purge tokenizer: ${error}`); }
    }

    private handleProgress(kind: 'litert' | 'supertonic', payload: { downloaded: number; total: number; state?: string }) {
        const done = payload.state === 'SUCCEEDED';
        const failed = payload.state === 'FAILED' || payload.state === 'CANCELLED';
        if (kind === 'litert') {
            this.litertDownloading = !done && !failed;
            this.litertProgress = payload.total > 0 ? Math.round(payload.downloaded / payload.total * 100) : -1;
            if (done) { this.litertProgress = -1; void this.checkLitert(); toast.success('LiteRT model downloaded!'); }
        } else {
            this.supertonicDownloading = !done && !failed;
            this.supertonicProgress = payload.total > 0 ? Math.round(payload.downloaded / payload.total * 100) : -1;
            if (done) { this.supertonicProgress = -1; void this.checkSupertonic(); toast.success('Supertonic models downloaded!'); }
        }
        if (failed) toast.error(`Download ${payload.state!.toLowerCase()}.`);
    }
}
