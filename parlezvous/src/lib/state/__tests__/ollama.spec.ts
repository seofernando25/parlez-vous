import { describe, it, expect, vi, beforeEach } from 'vitest';
import { invoke } from '@tauri-apps/api/core';
import { checkOllamaHealth, ollamaState } from '../ollama.svelte.ts';

describe('Ollama State', () => {
    beforeEach(() => {
        vi.clearAllMocks();
        ollamaState.isHealthy = false;
        ollamaState.hasChecked = false;
    });

    it('should update state to healthy when backend returns true', async () => {
        vi.mocked(invoke).mockResolvedValueOnce(true).mockResolvedValueOnce([]);

        await checkOllamaHealth();

        expect(invoke).toHaveBeenCalledWith('check_ollama_health');
        expect(ollamaState.isHealthy).toBe(true);
        expect(ollamaState.hasChecked).toBe(true);
    });

    it('should update state to unhealthy when backend returns false', async () => {
        vi.mocked(invoke).mockResolvedValueOnce(false);

        await checkOllamaHealth();

        expect(ollamaState.isHealthy).toBe(false);
        expect(ollamaState.hasChecked).toBe(true);
    });

    it('should update state to unhealthy when backend throws an error', async () => {
        vi.mocked(invoke).mockRejectedValueOnce(new Error('Network error'));

        await checkOllamaHealth();

        expect(ollamaState.isHealthy).toBe(false);
        expect(ollamaState.hasChecked).toBe(true);
    });
});
