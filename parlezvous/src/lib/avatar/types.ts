export type ViewMode = 'split' | 'avatar' | 'chat';
export type HandwritingScript = 'korean' | 'russian' | 'ukrainian';

export type AvatarChatMessage = {
    id?: number;
    role: string;
    content: string;
    correction?: string | null;
    audioBase64?: string;
};

export type ChatContext = {
    activeTextbook: string | null;
    activePage: number | null;
    activeThemeId: string | null;
    mapFollowMode: boolean;
};
