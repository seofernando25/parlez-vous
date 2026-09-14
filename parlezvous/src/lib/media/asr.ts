export interface TranscriptionResponse {
    text?: string;
}

export async function transcribeWav(audio: Blob, endpoint: string): Promise<string> {
    const url = endpoint.trim();
    if (!url) {
        throw new Error('ASR server URL is not configured');
    }

    const formData = new FormData();
    formData.append('file', audio, 'speech.wav');
    formData.append('model', 'whisper-1');

    const response = await fetch(url, {
        method: 'POST',
        body: formData
    });

    if (!response.ok) {
        const detail = (await response.text()).trim();
        throw new Error(`ASR request failed (${response.status})${detail ? `: ${detail}` : ''}`);
    }

    const payload = (await response.json()) as TranscriptionResponse;
    return payload.text?.trim() ?? '';
}

export function isUsefulTranscription(text: string): boolean {
    const normalized = text.trim();
    return normalized.length > 1 && !normalized.toLowerCase().includes('thank you for watching');
}
