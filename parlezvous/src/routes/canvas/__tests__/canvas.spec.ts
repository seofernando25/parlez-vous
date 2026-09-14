import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import { invoke } from '@tauri-apps/api/core';
import CanvasPage from '../+page.svelte';

describe('Canvas Page', () => {
    beforeEach(() => {
        vi.clearAllMocks();

        // Mock get_all_jamo to return a known set
        vi.mocked(invoke).mockImplementation(async (cmd: string, _args?: any) => {
            if (cmd === 'get_all_jamo') {
                return ['ㅏ', 'ㅂ', 'ㄱ'];
            }
            if (cmd === 'get_alphabet_letters') {
                return [{
                    char: 'ㅏ', uppercase: 'ㅏ', lowercase: 'ㅏ', name: 'ㅏ',
                    romanization: 'a', pronunciation: '[a]', script: 'hangul', is_vowel: true
                }];
            }
            if (cmd === 'infer_character') {
                return 'ㅏ'; // Mock prediction
            }
            return null;
        });
        
        // Mock canvas context
        HTMLCanvasElement.prototype.getContext = vi.fn().mockReturnValue({
            fillStyle: '',
            fillRect: vi.fn(),
            beginPath: vi.fn(),
            moveTo: vi.fn(),
            lineTo: vi.fn(),
            stroke: vi.fn(),
            closePath: vi.fn(),
            lineWidth: 0,
            lineCap: '',
            lineJoin: '',
            strokeStyle: '',
            getImageData: vi.fn().mockReturnValue({
                data: new Uint8ClampedArray(280 * 280 * 4).fill(255) // all white
            }),
            putImageData: vi.fn(),
            drawImage: vi.fn()
        } as any);
        
        // Mock document.createElement for temp canvases inside preprocessCanvas
        const originalCreateElement = document.createElement.bind(document);
        vi.spyOn(document, 'createElement').mockImplementation((tagName) => {
            if (tagName === 'canvas') {
                return {
                    width: 0,
                    height: 0,
                    getContext: vi.fn().mockReturnValue({
                        getImageData: vi.fn().mockReturnValue({
                            data: new Uint8ClampedArray(20 * 20 * 4).fill(255)
                        }),
                        putImageData: vi.fn(),
                        drawImage: vi.fn()
                    })
                } as any;
            }
            return originalCreateElement(tagName);
        });
    });

    it('should preprocess canvas and submit a 784-length numeric array', async () => {
        const { getByRole, getAllByText } = render(CanvasPage);

        // Wait for both startup IPC calls so the derived target character exists.
        await vi.waitFor(() => {
            expect(invoke).toHaveBeenCalledWith('get_all_jamo');
            expect(invoke).toHaveBeenCalledWith('get_alphabet_letters', { script: 'korean' });
            expect(getAllByText('ㅏ').length).toBeGreaterThan(0);
        });

        const submitBtn = getByRole('button', { name: 'Check drawing' });
        await fireEvent.click(submitBtn);

        // Should call infer_character with a jamo target
        expect(invoke).toHaveBeenCalledWith('infer_character', expect.objectContaining({
            pixels: expect.any(Array),
            vocabId: 1,
            targetText: expect.any(String)
        }));
        
        const calls = vi.mocked(invoke).mock.calls;
        const inferCall = calls.find(c => c[0] === 'infer_character');
        expect(inferCall).toBeDefined();
        const callArgs = inferCall![1] as any;
        expect(callArgs.pixels).toHaveLength(784); // 28x28 grid
        expect(typeof callArgs.pixels[0]).toBe('number');
    });
});
