import type { TtsProviderPolicy } from '$lib/media/tts-policy';
import { invoke } from '@tauri-apps/api/core';

export const settingsState = $state({
    targetLanguage: 'Korean',
    ttsServerUrl: 'http://localhost:5050/v1/audio/speech',
    asrServerUrl: 'http://localhost:8000/v1/audio/transcriptions',
    ollamaServerUrl: 'http://localhost:11434',
    embeddingModel: 'nomic-embed-text-v2-moe:latest',
    activeModel: 'gemma4-context:latest',
    skillLevel: 'Beginner',
    huggingFaceToken: '',
    litertAccelerator: 'Auto',
    litertMaxTokens: 1024,
    targetProgrammingLanguage: 'python',
    codingThemeCategory: 'All',
    activeVrm: 'avatar.vrm',
    supertonicVoiceStyle: 'voice_styles/F1.json',
    ttsProvider: 'auto' as TtsProviderPolicy,
    isLoaded: false
});

export async function loadSettings() {
    try {
        const settings = await invoke<{ target_language: string, tts_server_url: string, asr_server_url: string, ollama_server_url: string, embedding_model: string, active_model: string, huggingface_token: string | null, litert_accelerator: string, litert_max_tokens: number, target_programming_language: string, coding_theme_category: string, active_vrm: string, supertonic_voice_style: string, tts_provider: TtsProviderPolicy }>('get_settings');
        settingsState.targetLanguage = settings.target_language;
        settingsState.ttsServerUrl = settings.tts_server_url;
        settingsState.asrServerUrl = settings.asr_server_url;
        settingsState.ollamaServerUrl = settings.ollama_server_url;
        settingsState.embeddingModel = settings.embedding_model;
        settingsState.activeModel = settings.active_model;
        settingsState.huggingFaceToken = settings.huggingface_token || '';
        settingsState.litertAccelerator = settings.litert_accelerator;
        settingsState.litertMaxTokens = settings.litert_max_tokens;
        settingsState.targetProgrammingLanguage = settings.target_programming_language;
        settingsState.codingThemeCategory = settings.coding_theme_category;
        settingsState.activeVrm = settings.active_vrm;
        settingsState.supertonicVoiceStyle = settings.supertonic_voice_style;
        settingsState.ttsProvider = settings.tts_provider || 'auto';

        const skillLevel = await invoke<string>('get_user_skill_level');
        settingsState.skillLevel = skillLevel;

        settingsState.isLoaded = true;
    } catch (e) {
        console.error('Failed to load settings:', e);
        // Fallbacks are already in state
        settingsState.isLoaded = true;
    }
}

export async function saveSettings() {
    try {
        await invoke('update_settings', {
            settings: {
                target_language: settingsState.targetLanguage,
                tts_server_url: settingsState.ttsServerUrl,
                asr_server_url: settingsState.asrServerUrl,
                ollama_server_url: settingsState.ollamaServerUrl,
                embedding_model: settingsState.embeddingModel,
                active_model: settingsState.activeModel,
                huggingface_token: settingsState.huggingFaceToken.trim() !== '' ? settingsState.huggingFaceToken : null,
                litert_accelerator: settingsState.litertAccelerator,
                litert_max_tokens: Number(settingsState.litertMaxTokens) || 5000,
                target_programming_language: settingsState.targetProgrammingLanguage,
                coding_theme_category: settingsState.codingThemeCategory,
                active_vrm: settingsState.activeVrm,
                supertonic_voice_style: settingsState.supertonicVoiceStyle,
                tts_provider: settingsState.ttsProvider
            }
        });

        await invoke('set_user_skill_level', {
            level: settingsState.skillLevel
        });
    } catch (e) {
        console.error('Failed to save settings:', e);
        throw e;
    }
}
