import { invoke } from '@tauri-apps/api/core';
import { settingsState } from './settings.svelte';
import { getTierFromXP, getRandomThemeAndSubthemeForTier } from '../curriculum';
import toast from 'svelte-french-toast';
import { getModelCapabilities } from '$lib/ai/capabilities';

export type LanguageQueueItem = {
    question_type: string;
    question_data: string; // JSON string from the backend
};

export const languageSpeedrunState = $state({
    queue: [] as LanguageQueueItem[],
    isGenerating: false,
    isActive: false,
});

export async function initLanguageQueue() {
    languageSpeedrunState.isActive = true;

    try {
        const savedQueue = await invoke<LanguageQueueItem[]>('load_language_queue');
        languageSpeedrunState.queue = savedQueue || [];
    } catch (e) {
        console.error("Failed to load language queue", e);
    }

    // Start background generation if needed
    maintainLanguageQueue().catch(console.error);
}

export function stopLanguageQueue() {
    languageSpeedrunState.isActive = false;
}

export async function maintainLanguageQueue() {
    // Keep 3 items in the queue
    if (languageSpeedrunState.queue.length >= 3 || languageSpeedrunState.isGenerating) {
        return;
    }
    
    languageSpeedrunState.isGenerating = true;

    try {
        // Wait for settings to load before generating
        while (!settingsState.isLoaded || !settingsState.skillLevel) {
            await new Promise(r => setTimeout(r, 100));
        }

        while (languageSpeedrunState.queue.length < 3) {
            if (!languageSpeedrunState.isActive) {
                break;
            }
            
            // Get active theme from the curriculum state, but we don't have it directly here.
            // Actually, we can get it via `settingsState` or we can just pass a fallback.
            // For now, let's just use a default theme if none is active. 
            // Wait, we need the active theme. Let's see if we have `curriculumState`.
            // Wait, there's `get_curriculum` in tauri commands.
            // To simplify, let's just pass "General conversation" if we can't easily fetch it.
            // I'll fetch the curriculum theme or default.
            let activeTheme = "General Conversation";
            let activeSubtheme = "Casual chat";
            try {
                const curriculum: any = await invoke('get_curriculum', { language: settingsState.targetLanguage });
                const tier = getTierFromXP(curriculum.total_xp || 0);
                const randomTopic = getRandomThemeAndSubthemeForTier(tier);
                activeTheme = randomTopic.theme;
                activeSubtheme = randomTopic.subtheme;
            } catch(e) {}
            
const capabilities = getModelCapabilities(settingsState.activeModel);
            const puzzleType = capabilities.supportsKeystonePuzzles && Math.random() > 0.5 ? "keystone" : "speedrun";
            
            console.log(`[maintainLanguageQueue] Requesting puzzle... (Queue size: ${languageSpeedrunState.queue.length}, Model: ${settingsState.activeModel})`);
            const rawJson = await invoke<string>('generate_language_puzzle', {
                language: settingsState.targetLanguage,
                model: settingsState.activeModel,
                skillLevel: settingsState.skillLevel,
                activeTheme: activeTheme,
                activeSubtheme: activeSubtheme,
                puzzleType: puzzleType
            });
            console.log(`[maintainLanguageQueue] Received raw puzzle JSON length: ${rawJson.length}`);
            
            languageSpeedrunState.queue.push({
                question_type: puzzleType,
                question_data: rawJson
            });
            
            // Save to DB incrementally
            await invoke('save_language_queue', { queue: $state.snapshot(languageSpeedrunState.queue) });
        }
    } catch (e) {
        console.error("Failed to generate language puzzle", e);
        toast.error(`Puzzle generation failed: ${e}`);
    } finally {
        languageSpeedrunState.isGenerating = false;
    }
}

export async function popLanguagePuzzle(): Promise<LanguageQueueItem | null> {
    if (languageSpeedrunState.queue.length === 0) {
        maintainLanguageQueue().catch(console.error);
        return null; // the UI should show a loading state
    }
    
    const puzzle = languageSpeedrunState.queue.shift()!;
    await invoke('save_language_queue', { queue: $state.snapshot(languageSpeedrunState.queue) });
    
    // trigger background generation to refill the queue
    maintainLanguageQueue().catch(console.error);
    
    return puzzle;
}
