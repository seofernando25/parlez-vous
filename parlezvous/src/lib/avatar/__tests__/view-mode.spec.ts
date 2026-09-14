import { describe, expect, it } from 'vitest';
import { panesForViewMode } from '../view-mode';

describe('Tutor view modes', () => {
    it('maps each mode to the intended panes', () => {
        expect(panesForViewMode('split')).toEqual({ stage: true, chat: true });
        expect(panesForViewMode('avatar')).toEqual({ stage: true, chat: false });
        expect(panesForViewMode('chat')).toEqual({ stage: false, chat: true });
    });
});
