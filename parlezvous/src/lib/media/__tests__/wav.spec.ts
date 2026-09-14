import { describe, expect, it } from 'vitest';
import { encodeMonoPcm16Wav, mergeFloat32Buffers } from '../wav';

describe('WAV media helpers', () => {
    it('merges recorder buffers without changing order', () => {
        const merged = mergeFloat32Buffers([new Float32Array([0, 0.5]), new Float32Array([-0.5])], 3);
        expect(Array.from(merged)).toEqual([0, 0.5, -0.5]);
    });

    it('encodes a mono 16-bit PCM WAV header', () => {
        const wav = encodeMonoPcm16Wav(new Float32Array([0, 1, -1]), 16_000);
        const bytes = new Uint8Array(wav.buffer);
        expect(String.fromCharCode(...bytes.slice(0, 4))).toBe('RIFF');
        expect(String.fromCharCode(...bytes.slice(8, 12))).toBe('WAVE');
        expect(wav.getUint32(24, true)).toBe(16_000);
        expect(wav.getUint32(40, true)).toBe(6);
    });
});
