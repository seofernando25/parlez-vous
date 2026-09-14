import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, waitFor } from '@testing-library/svelte';
import { invoke } from '@tauri-apps/api/core';
import FlashcardsPage from '../+page.svelte';

vi.mock('@tauri-apps/api/core', () => ({
    invoke: vi.fn()
}));

describe('Flashcards Page', () => {
    beforeEach(() => {
        vi.clearAllMocks();
    });

    it('should fetch and display vocabulary on mount', async () => {
        const mockVocab = [
            { id: 1, target_text: 'pomme', native_text: 'apple', is_character: false },
            { id: 2, target_text: 'chat', native_text: 'cat', is_character: false }
        ];
        
        vi.mocked(invoke).mockResolvedValueOnce(mockVocab);

        const { getByText, queryByText } = render(FlashcardsPage);

        // Initially shows loading state (or quickly renders)
        expect(invoke).toHaveBeenCalledWith('get_all_vocabulary');

        // Wait for the mock items to appear in the DOM
        await waitFor(() => {
            expect(getByText('pomme')).toBeInTheDocument();
            expect(getByText('apple')).toBeInTheDocument();
            expect(getByText('chat')).toBeInTheDocument();
            expect(getByText('cat')).toBeInTheDocument();
        });

        // The total words count should be correct
        expect(getByText('2')).toBeInTheDocument();
    });

    it('should display empty state when no vocabulary exists', async () => {
        vi.mocked(invoke).mockResolvedValueOnce([]);

        const { getByText } = render(FlashcardsPage);

        await waitFor(() => {
            expect(getByText('No saved words yet.')).toBeInTheDocument();
        });
        expect(getByText('0')).toBeInTheDocument(); // total words
    });
});
