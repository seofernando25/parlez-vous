import { invoke } from '@tauri-apps/api/core';
import toast from 'svelte-french-toast';
import { settingsState } from '$lib/state/settings.svelte';
import { playSmartTTS } from '$lib/tts';
import { drawingCoordinates, preprocessStrokes, type Point } from '$lib/handwriting/canvas';
import { fallbackAlphabet } from './fallback';
import { SCRIPT_INFO, scriptForTargetLanguage, type AlphabetItem, type ScriptType } from './types';

export class AlphabetPracticeController {
    canvas = $state<HTMLCanvasElement>(undefined as any);
    result = $state<string | null>(null);
    isSubmitting = $state(false);
    sessionCorrect = $state(0);
    sessionTotal = $state(0);
    activeScript = $state<ScriptType>('russian');
    letters = $state<AlphabetItem[]>([]);
    currentIndex = $state(0);
    isUppercase = $state(true);
    showGuide = $state(true);
    allJamo = $state<string[]>([]);
    allStrokes = $state<Point[][]>([]);

    private ctx?: CanvasRenderingContext2D;
    private isDrawing = false;
    private currentStroke: Point[] = [];

    get currentItem() { return this.letters[this.currentIndex] ?? null; }
    get targetChar() {
        if (!this.currentItem) return '';
        return this.activeScript === 'korean' ? this.currentItem.char : this.isUppercase ? this.currentItem.uppercase : this.currentItem.lowercase;
    }
    get isCorrect() { return this.result !== null && this.result.trim().toLowerCase() === this.targetChar.trim().toLowerCase(); }

    async init(canvas: HTMLCanvasElement) {
        this.canvas = canvas;
        this.ctx = canvas.getContext('2d', { willReadFrequently: true })!;
        this.resetCanvas();
        try { this.allJamo = await invoke<string[]>('get_all_jamo'); }
        catch { this.allJamo = ['ㅏ', 'ㅐ', 'ㅂ', 'ㅃ', 'ㅊ', 'ㄷ', 'ㅔ', 'ㅓ', 'ㅡ', 'ㄱ']; }
        await this.loadAlphabet(scriptForTargetLanguage(settingsState.targetLanguage || ''));
    }

    async loadAlphabet(script: ScriptType) {
        this.activeScript = script;
        this.currentIndex = 0;
        this.clearCanvas();
        try {
            const fetched = await invoke<AlphabetItem[]>('get_alphabet_letters', { script });
            if (fetched?.length) { this.letters = fetched; return; }
        } catch (error) { console.warn('Using built-in alphabet table:', error); }
        this.letters = fallbackAlphabet(script, this.allJamo);
    }

    resetCanvas() {
        if (!this.canvas || !this.ctx) return;
        this.ctx.fillStyle = '#fff';
        this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);
        this.ctx.lineWidth = 14;
        this.ctx.lineCap = 'round';
        this.ctx.lineJoin = 'round';
        this.ctx.strokeStyle = '#18181b';
    }

    startDrawing = (event: MouseEvent | TouchEvent) => {
        if (!this.ctx || !this.canvas) return;
        this.isDrawing = true;
        const point = drawingCoordinates(this.canvas, event);
        this.currentStroke = [point];
        this.ctx.beginPath(); this.ctx.moveTo(point.x, point.y);
        event.preventDefault();
    };
    draw = (event: MouseEvent | TouchEvent) => {
        if (!this.isDrawing || !this.ctx || !this.canvas) return;
        const point = drawingCoordinates(this.canvas, event);
        this.currentStroke.push(point);
        this.ctx.lineTo(point.x, point.y); this.ctx.stroke(); event.preventDefault();
    };
    stopDrawing = () => {
        if (!this.isDrawing || !this.ctx) return;
        this.ctx.closePath(); this.isDrawing = false;
        if (this.currentStroke.length) this.allStrokes = [...this.allStrokes, [...this.currentStroke]];
    };
    clearCanvas = () => { this.resetCanvas(); this.result = null; this.allStrokes = []; };

    async submit() {
        if (this.isSubmitting || !this.targetChar) return;
        this.isSubmitting = true;
        try {
            const prediction = await invoke<string>('infer_character', {
                pixels: preprocessStrokes(this.allStrokes), vocabId: 1, targetText: this.targetChar
            });
            this.result = prediction;
            this.sessionTotal++;
            if (this.isCorrect) {
                this.sessionCorrect++;
                toast.success(`${this.targetChar} ${this.currentItem?.romanization ? `(${this.currentItem.romanization})` : ''}`);
            }
        } catch (error) { console.error('Inference error:', error); toast.error(String(error)); }
        finally { this.isSubmitting = false; }
    }

    next() { if (this.letters.length) { this.currentIndex = (this.currentIndex + 1) % this.letters.length; this.clearCanvas(); } }
    previous() { if (this.letters.length) { this.currentIndex = (this.currentIndex - 1 + this.letters.length) % this.letters.length; this.clearCanvas(); } }
    random() {
        if (this.letters.length <= 1) return;
        let next = Math.floor(Math.random() * this.letters.length);
        while (next === this.currentIndex) next = Math.floor(Math.random() * this.letters.length);
        this.currentIndex = next; this.clearCanvas();
    }
    select(index: number) { this.currentIndex = index; this.clearCanvas(); }
    setUppercase(value: boolean) { this.isUppercase = value; this.clearCanvas(); }
    async playAudio() { if (this.targetChar) await playSmartTTS(this.targetChar, settingsState.ttsServerUrl, undefined, SCRIPT_INFO[this.activeScript].langCode, settingsState.ttsProvider); }
}
