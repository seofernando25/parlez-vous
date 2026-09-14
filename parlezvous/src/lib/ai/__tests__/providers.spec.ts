import { describe, expect, it } from 'vitest';
import { providerPreset } from '../providers';

describe('provider presets', () => {
    it('maps local and hosted OpenAI-compatible endpoints', () => {
        expect(providerPreset('ollama').baseUrl).toBe('http://localhost:11434/v1');
        expect(providerPreset('lmstudio').baseUrl).toBe('http://localhost:1234/v1');
        expect(providerPreset('openrouter').baseUrl).toBe('https://openrouter.ai/api/v1');
        expect(providerPreset('openai').baseUrl).toBe('https://api.openai.com/v1');
        expect(providerPreset('unknown').id).toBe('custom');
    });
});
