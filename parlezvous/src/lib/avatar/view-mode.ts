import type { ViewMode } from './types';

export interface TutorPanes {
    stage: boolean;
    chat: boolean;
}

export function panesForViewMode(mode: ViewMode): TutorPanes {
    return {
        stage: mode !== 'chat',
        chat: mode !== 'avatar'
    };
}
