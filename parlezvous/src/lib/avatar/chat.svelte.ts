import { invoke } from '@tauri-apps/api/core';
import { tick } from 'svelte';
import { getRandomThemeAndSubthemeForTier, getTierFromXP } from '$lib/curriculum';
import { settingsState } from '$lib/state/settings.svelte';
import type { AvatarChatMessage, ChatContext, ViewMode } from './types';

export class AvatarChatController {
    history = $state<AvatarChatMessage[]>([]);
    input = $state('');
    isChatting = $state(false);
    inputRef = $state<HTMLTextAreaElement | null>(null);
    scrollContainer = $state<HTMLDivElement | null>(null);

    constructor(
        private getContext: () => ChatContext,
        private getViewMode: () => ViewMode,
        private speak: (text: string) => Promise<void>
    ) {}

    async load() {
        try {
            const history = await invoke<AvatarChatMessage[]>('get_chat_history');
            this.history = Array.isArray(history) ? history : [];
        } catch (error) {
            console.error('Failed to load chat history:', error);
        }
    }

    async clear() {
        try {
            await invoke('clear_chat_history');
            this.history = [];
        } catch (error) {
            console.error('Failed to clear chat:', error);
        }
    }

    appendInput(text: string) {
        const separator = this.input && !this.input.endsWith(' ') && text.trim() ? ' ' : '';
        this.input += separator + text;
    }

    setInput(text: string) { this.input = text; }

    async send(audioBase64?: string) {
        if (!this.input.trim() && !audioBase64) return;
        const userMessage: AvatarChatMessage = {
            role: 'user',
            content: this.input.trim() || '🎤 [Audio Message]',
            audioBase64
        };
        this.history = [...this.history, userMessage];
        await this.persistUserMessage(userMessage);
        this.input = '';
        this.isChatting = true;
        this.resetInputFocus(audioBase64);
        await this.scrollToBottom(false);

        try {
            const context = this.getContext();
            const topic = await this.resolveTopic(context);
            const response = await invoke<{
                response: string;
                idealized_correction?: string;
                context_summary?: string;
            }>('chat_with_avatar', {
                history: this.historyPayload(),
                model: settingsState.activeModel,
                language: settingsState.targetLanguage,
                activeTextbook: context.activeTextbook,
                activePage: context.activePage,
                activeTheme: topic.theme,
                activeSubtheme: topic.subtheme,
                audioBase64: audioBase64 || null
            });
            await this.applyCorrection(userMessage, response.idealized_correction);
            if (response.context_summary) await this.replaceWithSummary(userMessage, response.response, response.context_summary);
            else await this.appendAssistant(response.response);
            await this.scrollToBottom(true);
            await this.speak(response.response);
        } catch (error) {
            console.error('Chat failed:', error);
            this.history = [...this.history, { role: 'assistant', content: `Sorry, I encountered an error. ${error}` }];
        } finally {
            this.isChatting = false;
        }
    }

    private async persistUserMessage(message: AvatarChatMessage) {
        try {
            message.id = await this.persist(message);
        } catch (error) {
            console.error('Failed to persist user chat message:', error);
        }
    }

    private async applyCorrection(message: AvatarChatMessage, correction?: string) {
        if (!correction || correction === 'null' || !correction.trim()) return;
        message.correction = correction;
        if (!message.id) return;
        try {
            await invoke('update_chat_message_correction', { id: message.id, correction });
        } catch (error) {
            console.error('Failed to persist chat correction:', error);
        }
    }

    private async replaceWithSummary(user: AvatarChatMessage, response: string, summary: string) {
        this.history = [
            { role: 'system', content: `Context Compressed: ${summary}` },
            user,
            { role: 'assistant', content: response }
        ];
        try {
            await invoke('clear_chat_history');
            const persisted: AvatarChatMessage[] = [];
            for (const message of this.history) persisted.push({ ...message, id: await this.persist(message) });
            this.history = persisted;
        } catch (error) {
            console.error('Failed to sync summary:', error);
        }
    }

    private async appendAssistant(content: string) {
        const message: AvatarChatMessage = { role: 'assistant', content };
        this.history = [...this.history, message];
        try { message.id = await this.persist(message); }
        catch (error) { console.error('Failed to persist assistant chat message:', error); }
    }

    private persist(message: AvatarChatMessage) {
        return invoke<number>('save_chat_message', {
            role: message.role,
            content: message.content,
            correction: message.correction || null,
            audioBase64: message.audioBase64 || null
        });
    }

    private historyPayload() {
        let audioCount = 0;
        return this.history.slice().reverse().map(message => {
            const payload: any = { role: message.role, content: message.content };
            if (message.audioBase64 && audioCount++ < 2) payload.audio_base64 = message.audioBase64;
            return payload;
        }).reverse();
    }

    private async resolveTopic(context: ChatContext) {
        let theme = context.mapFollowMode ? context.activeThemeId : null;
        let subtheme: string | null = null;
        try {
            const curriculum: any = await invoke('get_curriculum', { language: settingsState.targetLanguage });
            const random = getRandomThemeAndSubthemeForTier(getTierFromXP(curriculum.total_xp || 0));
            if (!context.mapFollowMode) theme = random.theme;
            subtheme = random.subtheme;
        } catch (error) {
            console.error('Failed to fetch tier for avatar chat:', error);
        }
        return { theme, subtheme };
    }

    private resetInputFocus(audioBase64?: string) {
        if (this.inputRef) this.inputRef.style.height = 'auto';
        const mode = this.getViewMode();
        if (audioBase64 || (mode === 'split' && window.innerWidth < 768)) setTimeout(() => this.inputRef?.blur(), 10);
        else if (mode === 'chat') setTimeout(() => this.inputRef?.focus({ preventScroll: true }), 10);
    }

    private async scrollToBottom(smooth: boolean) {
        await tick();
        this.scrollContainer?.scrollTo({
            top: this.scrollContainer.scrollHeight,
            behavior: smooth ? 'smooth' : 'auto'
        });
    }
}
