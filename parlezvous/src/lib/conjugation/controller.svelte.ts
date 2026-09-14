import { invoke } from '@tauri-apps/api/core';
import { tick } from 'svelte';
import toast from 'svelte-french-toast';
import { settingsState } from '$lib/state/settings.svelte';
import { maskConjugation } from './display';
import type { ConjugationExercise, ConjugationResponse, HistoryEntry } from './types';

export class ConjugationController {
    isGenerating = $state(false);
    exercise = $state<ConjugationExercise | null>(null);
    historyId = $state<number | null>(null);
    userAnswer = $state('');
    hasSubmitted = $state(false);
    isCorrect = $state(false);
    history = $state<HistoryEntry[]>([]);
    timerSeconds = $state(0);
    timerMax = $state(120);
    timerActive = $state(false);
    inputEl = $state<HTMLInputElement>();
    pageEl = $state<HTMLDivElement>();
    private timerInterval: ReturnType<typeof setInterval> | null = null;

    get displaySentence() { return this.exercise ? maskConjugation(this.exercise) : ''; }
    get recentScore() { return this.history.slice(-10).filter(item => item.correct).length; }
    get timerProgress() { return this.timerMax > 0 ? this.timerSeconds / this.timerMax : 0; }
    get timerColor() { return this.timerProgress > .5 ? 'var(--pv-accent)' : this.timerProgress > .2 ? 'var(--pv-warning)' : 'var(--pv-danger)'; }
    get formattedTime() { const m = Math.floor(this.timerSeconds / 60); return `${m}:${(this.timerSeconds % 60).toString().padStart(2, '0')}`; }

    private timerDuration() { return Math.round(180 - (150 * Math.max(0, Math.min(10, this.recentScore))) / 10); }
    private startTimer() {
        this.stopTimer(); this.timerMax = this.timerDuration(); this.timerSeconds = this.timerMax; this.timerActive = true;
        this.timerInterval = setInterval(() => {
            this.timerSeconds--;
            if (this.timerSeconds > 0) return;
            if (!this.hasSubmitted && this.exercise) this.checkAnswer();
            void this.generate();
        }, 1000);
    }
    private stopTimer() { if (this.timerInterval) clearInterval(this.timerInterval); this.timerInterval = null; this.timerActive = false; }

    async generate() {
        if (!settingsState.activeModel) return void toast.error('Please select an active model in settings.');
        this.stopTimer(); this.isGenerating = true; this.exercise = null; this.historyId = null; this.userAnswer = ''; this.hasSubmitted = false;
        try {
            const result = await invoke<ConjugationResponse>('generate_conjugation_exercise', { language: settingsState.targetLanguage, model: settingsState.activeModel });
            this.exercise = result.exercise; this.historyId = result.history_id;
        } catch (error) { console.error('Failed to generate exercise:', error); toast.error(`Failed to generate exercise: ${error}`); }
        finally { this.isGenerating = false; }
        if (this.exercise) { await tick(); this.inputEl?.focus(); this.startTimer(); }
    }

    checkAnswer() {
        if (!this.exercise || this.hasSubmitted) return;
        this.hasSubmitted = true;
        this.isCorrect = this.userAnswer.trim().toLowerCase() === this.exercise.answer.trim().toLowerCase();
        this.history = [...this.history, { exercise: this.exercise, userAnswer: this.userAnswer.trim(), correct: this.isCorrect }].slice(-10);
        if (this.historyId !== null) invoke('record_conjugation_result', { historyId: this.historyId, correct: this.isCorrect }).catch(error => console.error('Failed to record result:', error));
        tick().then(() => this.pageEl?.focus());
    }

    handleInputKeydown = (event: KeyboardEvent) => {
        if (event.key === 'Enter' && !this.hasSubmitted && this.userAnswer.trim()) { event.preventDefault(); this.checkAnswer(); }
    };
    handlePageKeydown = (event: KeyboardEvent) => {
        if (event.key === 'Enter' && this.hasSubmitted && document.activeElement !== this.inputEl) { event.preventDefault(); void this.generate(); }
    };
    dispose() { this.stopTimer(); invoke('cancel_conjugation_generation').catch(error => console.error('Failed to cancel generation:', error)); }
}
