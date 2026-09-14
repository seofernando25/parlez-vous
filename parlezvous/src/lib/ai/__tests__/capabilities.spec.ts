import { describe, expect, it } from 'vitest';
import { getModelCapabilities, isLiteRtModel } from '../capabilities';

describe('model capabilities', () => {
    it('recognizes LiteRT model files without substring guesses', () => {
        expect(isLiteRtModel('gemma-4-E2B-it.litertlm')).toBe(true);
        expect(isLiteRtModel('MODEL.LITERTLM')).toBe(true);
        expect(isLiteRtModel('my-litert-experiment')).toBe(false);
    });

    it('routes audio and puzzle capabilities by provider', () => {
        expect(getModelCapabilities('gemma-4-E2B-it.litertlm')).toMatchObject({
            provider: 'litert',
            isOnDevice: true,
            acceptsAudio: true,
            supportsKeystonePuzzles: false
        });

        expect(getModelCapabilities('gemma4-context:latest')).toMatchObject({
            provider: 'ollama',
            isOnDevice: false,
            acceptsAudio: false,
            supportsKeystonePuzzles: true
        });
    });
});
