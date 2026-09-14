import { beforeEach, describe, expect, it, vi } from 'vitest';
import { invoke } from '@tauri-apps/api/core';
import { aiProviderState, checkAiHealth, fetchAiModels } from '../aiProvider.svelte';
import { settingsState } from '../settings.svelte';

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));

describe('AI provider state', () => {
    beforeEach(() => {
        vi.clearAllMocks();
        aiProviderState.isHealthy = true;
        aiProviderState.hasChecked = false;
        aiProviderState.models = [];
        settingsState.activeModel = 'existing-model';
    });

    it('checks the generic endpoint and loads its models', async () => {
        vi.mocked(invoke).mockImplementation(async command => {
            if (command === 'check_ai_health') return true;
            if (command === 'list_ai_models') return ['model-b', 'model-a'];
            return null;
        });
        await checkAiHealth();
        expect(invoke).toHaveBeenCalledWith('check_ai_health');
        expect(invoke).toHaveBeenCalledWith('list_ai_models');
        expect(aiProviderState.isHealthy).toBe(true);
        expect(aiProviderState.models).toEqual(['model-b', 'model-a']);
        expect(settingsState.activeModel).toBe('existing-model');
    });

    it('does not overwrite a custom model when discovery fails', async () => {
        vi.mocked(invoke).mockRejectedValue(new Error('no model catalog'));
        await fetchAiModels();
        expect(settingsState.activeModel).toBe('existing-model');
        expect(aiProviderState.models).toEqual([]);
    });
});
