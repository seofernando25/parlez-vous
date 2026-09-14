import { describe, expect, it } from 'vitest';
import { resolveTtsProviderUrls } from '../tts-policy';

describe('TTS provider policy', () => {
    it('always uses the configured server on desktop', () => {
        expect(resolveTtsProviderUrls('auto', false, 'http://localhost:5050')).toEqual(['http://localhost:5050']);
        expect(resolveTtsProviderUrls('supertonic', false, 'http://localhost:5050')).toEqual(['http://localhost:5050']);
    });

    it('resolves Android auto/local/server modes explicitly', () => {
        expect(resolveTtsProviderUrls('auto', true, 'http://host:5050')).toEqual(['supertonic://local', 'http://host:5050']);
        expect(resolveTtsProviderUrls('supertonic', true, 'http://host:5050')).toEqual(['supertonic://local']);
        expect(resolveTtsProviderUrls('server', true, 'http://host:5050')).toEqual(['http://host:5050']);
    });
});
