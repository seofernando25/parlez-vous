import { invoke } from '@tauri-apps/api/core';
import { getThemes, getRandomTheme } from '../coding/utils';
import { settingsState } from './settings.svelte';
import toast from 'svelte-french-toast';
import { getModelCapabilities } from '$lib/ai/capabilities';

export type CodingQueueItem = {
    question_type: string;
    question_data: string; // JSON string from the backend
};

export const codingState = $state({
    queue: [] as CodingQueueItem[],
    isGenerating: false,
    isActive: false,
    themes: {} as Record<string, string[]>,
    themesLoaded: false,
});

export async function initCodingQueue() {
    codingState.isActive = true;
    if (!codingState.themesLoaded) {
        try {
            codingState.themes = await getThemes();
            codingState.themesLoaded = true;
        } catch (e) {
            console.error("Failed to load themes", e);
        }
    }

    try {
        const savedQueue = await invoke<CodingQueueItem[]>('load_coding_queue');
        codingState.queue = savedQueue || [];
    } catch (e) {
        console.error("Failed to load coding queue", e);
    }

    // Start background generation if needed
    maintainCodingQueue().catch(console.error);
}

export function stopCodingQueue() {
    codingState.isActive = false;
}

export async function maintainCodingQueue() {
    // Keep 3 items in the queue
    if (codingState.queue.length >= 3 || codingState.isGenerating) {
        return;
    }
    
    codingState.isGenerating = true;

    try {
        // Wait for settings to load before generating
        while (!settingsState.isLoaded) {
            await new Promise(r => setTimeout(r, 100));
        }

        while (codingState.queue.length < 3) {
            if (!codingState.isActive) {
                break;
            }
            const theme = getRandomTheme(codingState.themes, settingsState.codingThemeCategory);
            
const capabilities = getModelCapabilities(settingsState.activeModel);
            const puzzleType = capabilities.supportsKeystonePuzzles && Math.random() > 0.5 ? "keystone" : "speedrun";
            
            console.log(`[maintainCodingQueue] Requesting puzzle... (Queue size: ${codingState.queue.length}, Model: ${settingsState.activeModel})`);
            const rawJson = await invoke<string>('generate_coding_puzzle', {
                language: settingsState.targetProgrammingLanguage,
                model: settingsState.activeModel,
                theme: theme,
                puzzleType: puzzleType
            });
            console.log(`[maintainCodingQueue] Received raw puzzle JSON length: ${rawJson.length}`);
            
            codingState.queue.push({
                question_type: puzzleType,
                question_data: rawJson
            });
            
            // Save to DB incrementally
            await invoke('save_coding_queue', { queue: $state.snapshot(codingState.queue) });
        }
    } catch (e) {
        console.error("Failed to generate coding puzzle", e);
        toast.error(`Puzzle generation failed: ${e}`);
    } finally {
        codingState.isGenerating = false;
    }
}

export async function popCodingPuzzle(): Promise<CodingQueueItem | null> {
    if (codingState.queue.length === 0) {
        maintainCodingQueue().catch(console.error);
        return null; // the UI should show a loading state
    }
    
    const puzzle = codingState.queue.shift()!;
    await invoke('save_coding_queue', { queue: $state.snapshot(codingState.queue) });
    
    // trigger background generation to refill the queue
    maintainCodingQueue().catch(console.error);
    
    return puzzle;
}
