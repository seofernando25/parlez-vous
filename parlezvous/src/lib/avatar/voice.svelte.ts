import toast from 'svelte-french-toast';
import { getModelCapabilities } from '$lib/ai/capabilities';
import { isUsefulTranscription, transcribeWav } from '$lib/media/asr';
import { blobToBase64, createWavBlob } from '$lib/media/wav';

export class VoiceCaptureController {
    isActive = $state(false);
    state = $state<'inactive' | 'speaking' | 'processing'>('inactive');

    private stream: MediaStream | null = null;
    private context: AudioContext | null = null;
    private processor: ScriptProcessorNode | null = null;
    private buffers: Float32Array[] = [];
    private recordingLength = 0;

    constructor(
        private getModel: () => string,
        private getAsrUrl: () => string,
        private setInput: (text: string) => void,
        private sendMessage: (audioBase64?: string) => Promise<void>
    ) {}

    toggle = async () => this.isActive ? this.stop() : this.start();

    async start() {
        try {
            this.stream = await navigator.mediaDevices.getUserMedia({
                audio: { echoCancellation: false, noiseSuppression: false, autoGainControl: false }
            });
            this.context = new AudioContext({ sampleRate: 16000 });
            this.processor = this.context.createScriptProcessor(4096, 1, 1);
            const source = this.context.createMediaStreamSource(this.stream);
            const gain = this.context.createGain();
            gain.gain.value = 0;
            source.connect(this.processor);
            this.processor.connect(gain);
            gain.connect(this.context.destination);
            this.buffers = [];
            this.recordingLength = 0;
            this.state = 'speaking';
            this.isActive = true;
            this.processor.onaudioprocess = event => {
                if (this.state !== 'speaking') return;
                const input = event.inputBuffer.getChannelData(0);
                this.buffers.push(new Float32Array(input));
                this.recordingLength += input.length;
            };
        } catch (error: any) {
            console.error('Microphone access denied or failed', error);
            toast.error(`Could not access microphone: ${error.message || error.name || 'Unknown error'}`);
        }
    }

    async stop() {
        if (!this.isActive) return;
        this.isActive = false;
        this.state = 'processing';
        this.processor?.disconnect();
        this.processor = null;
        this.stream?.getTracks().forEach(track => track.stop());
        this.stream = null;
        await this.processAudio();
        await this.context?.close();
        this.context = null;
        this.state = 'inactive';
    }

    private async processAudio() {
        if (!this.recordingLength || !this.context) return;
        const blob = createWavBlob(this.buffers, this.recordingLength, this.context.sampleRate);
        this.buffers = [];
        this.recordingLength = 0;
        const capabilities = getModelCapabilities(this.getModel());
        try {
            if (capabilities.acceptsAudio) {
                await this.sendMessage(await blobToBase64(blob));
                return;
            }
            const text = await transcribeWav(blob, this.getAsrUrl());
            if (isUsefulTranscription(text)) {
                this.setInput(text);
                await this.sendMessage();
            }
        } catch (error: any) {
            const source = capabilities.acceptsAudio ? 'On-device audio' : 'ASR';
            console.error(`${source} Error:`, error);
            toast.error(`${source} Error: ${error.message || error}`);
        }
    }

    dispose() {
        this.processor?.disconnect();
        this.stream?.getTracks().forEach(track => track.stop());
        this.context?.close();
        this.processor = null;
        this.stream = null;
        this.context = null;
        this.isActive = false;
        this.state = 'inactive';
    }
}
