import { invoke } from '@tauri-apps/api/core';
import { drawingCoordinates, preprocessStrokes, type Point } from '$lib/handwriting/canvas';
import {
    composeHangul, FINALS, INITIALS, RUSSIAN_ALPHABET, scriptForLanguage,
    supportsWordBuilding, UKRAINIAN_ALPHABET, VOWELS
} from './alphabet';
import type { HandwritingScript } from '../types';

export class HandwritingController {
    show = $state(false);
    currentSlot = $state<'initial' | 'vowel' | 'final'>('initial');
    blockInitial = $state<string | null>(null);
    blockVowel = $state<string | null>(null);
    blockFinal = $state<string | null>(null);
    composedWord = $state('');
    isUppercase = $state(false);
    lastRecognizedLetter = $state<string | null>(null);
    drawingCanvas = $state<HTMLCanvasElement>(undefined as any);
    drawingCtx = $state<CanvasRenderingContext2D>(undefined as any);
    allStrokes = $state<Point[][]>([]);
    status = $state<string | null>(null);
    isRecognizing = $state(false);

    private drawing = false;
    private currentStroke: Point[] = [];
    private previousScript: HandwritingScript | null = null;

    constructor(
        private getLanguage: () => string,
        private appendInput: (text: string) => void
    ) {}

    get activeScript() { return scriptForLanguage(this.getLanguage()); }
    get isSupported() { return supportsWordBuilding(this.getLanguage()); }
    get composedHangul() { return composeHangul(this.blockInitial, this.blockVowel, this.blockFinal); }
    get alphabet() { return this.activeScript === 'russian' ? RUSSIAN_ALPHABET : UKRAINIAN_ALPHABET; }

    syncLanguage() {
        const script = this.activeScript;
        if (script === this.previousScript) return;
        this.previousScript = script;
        this.clearWord();
        this.clearHangulBlock();
    }

    initCanvas = () => {
        if (!this.drawingCanvas) return;
        this.drawingCtx = this.drawingCanvas.getContext('2d', { willReadFrequently: true })!;
        this.clearCanvas();
    };

    clearCanvas = () => {
        if (!this.drawingCtx || !this.drawingCanvas) return;
        this.drawingCtx.fillStyle = '#ffffff';
        this.drawingCtx.fillRect(0, 0, this.drawingCanvas.width, this.drawingCanvas.height);
        this.drawingCtx.lineWidth = 14;
        this.drawingCtx.lineCap = 'round';
        this.drawingCtx.lineJoin = 'round';
        this.drawingCtx.strokeStyle = '#1a1a1a';
        this.allStrokes = [];
    };

    startDrawing = (event: MouseEvent | TouchEvent) => {
        if (!this.drawingCtx || !this.drawingCanvas) return;
        this.drawing = true;
        const point = drawingCoordinates(this.drawingCanvas, event);
        this.currentStroke = [point];
        this.drawingCtx.beginPath();
        this.drawingCtx.moveTo(point.x, point.y);
        event.preventDefault();
    };

    draw = (event: MouseEvent | TouchEvent) => {
        if (!this.drawing || !this.drawingCtx || !this.drawingCanvas) return;
        const point = drawingCoordinates(this.drawingCanvas, event);
        this.currentStroke.push(point);
        this.drawingCtx.lineTo(point.x, point.y);
        this.drawingCtx.stroke();
        event.preventDefault();
    };

    stopDrawing = () => {
        if (!this.drawing || !this.drawingCtx) return;
        this.drawingCtx.closePath();
        this.drawing = false;
        if (this.currentStroke.length) this.allStrokes = [...this.allStrokes, [...this.currentStroke]];
    };

    recognize = async () => {
        if (!this.allStrokes.length) return;
        this.isRecognizing = true;
        this.status = null;
        try {
            const char = (await invoke<string>('infer_character', {
                pixels: preprocessStrokes(this.allStrokes),
                vocabId: 1,
                targetText: this.activeScript,
                script: this.activeScript
            })).match(/^(\S+)/)?.[1];
            if (!char) return;
            this.applyRecognition(char);
        } catch (error) {
            this.status = `Recognition failed: ${error}`;
        } finally {
            this.isRecognizing = false;
            setTimeout(() => { if (this.status) this.status = null; }, 2500);
        }
    };

    private applyRecognition(char: string) {
        if (this.activeScript !== 'korean') {
            const formatted = this.isUppercase ? char.toUpperCase() : char.toLowerCase();
            this.composedWord += formatted;
            this.lastRecognizedLetter = formatted;
            this.status = `+${formatted}`;
            this.clearCanvas();
            return;
        }
        if (this.currentSlot === 'initial') {
            if (!INITIALS.includes(char)) return void (this.status = `${char} not an initial`);
            this.blockInitial = char; this.currentSlot = 'vowel'; this.status = `Initial: ${char}`;
        } else if (this.currentSlot === 'vowel') {
            if (!VOWELS.includes(char)) return void (this.status = `${char} not a vowel`);
            this.blockVowel = char; this.currentSlot = 'final'; this.status = `Vowel: ${char}`;
        } else {
            if (!FINALS.includes(char) || !char) return void (this.status = `${char} not a final`);
            this.blockFinal = char; this.status = `Final: ${char}`;
        }
        this.clearCanvas();
    }

    setSlot(slot: 'initial' | 'vowel' | 'final') { this.currentSlot = slot; this.clearCanvas(); }
    toggleCase() { this.isUppercase = !this.isUppercase; }
    appendLetter(char: string) {
        const formatted = this.isUppercase ? char.toUpperCase() : char.toLowerCase();
        this.composedWord += formatted; this.lastRecognizedLetter = formatted;
    }
    backspaceWord() { this.composedWord = this.composedWord.slice(0, -1); }
    appendWordSpace() { this.composedWord += ' '; }
    appendInputSpace() { this.appendInput(' '); }

    clearWord() {
        this.composedWord = ''; this.lastRecognizedLetter = null; this.clearCanvas(); this.status = null;
    }
    commitWord() {
        if (!this.composedWord.trim()) return;
        this.appendInput(this.composedWord); this.clearWord();
    }
    clearHangulBlock() {
        this.blockInitial = null; this.blockVowel = null; this.blockFinal = null;
        this.currentSlot = 'initial'; this.clearCanvas(); this.status = null;
    }
    commitHangulBlock() {
        if (!this.composedHangul) return;
        this.appendInput(this.composedHangul); this.clearHangulBlock();
    }
}
