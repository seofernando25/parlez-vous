import { invoke } from '@tauri-apps/api/core';
import { isAndroidTauri } from '$lib/platform';
import { resolveTtsProviderUrls, type TtsProviderPolicy } from '$lib/media/tts-policy';

export let ttsAudioContext: AudioContext | null = null;
export let ttsPlaybackRate: number = 1.0;
let mediaElementSource: MediaElementAudioSourceNode | null = null;
let hiddenAudioElement: HTMLAudioElement | null = null;
export let lipsyncNodeRef: any = null;

// Initialize the shared audio context and the hidden audio element
export function initTTSAudio() {
    if (ttsAudioContext) return;
    ttsAudioContext = new AudioContext();

    hiddenAudioElement = document.createElement('audio');
    hiddenAudioElement.preservesPitch = true;
    hiddenAudioElement.style.display = 'none';
    document.body.appendChild(hiddenAudioElement);

    mediaElementSource = ttsAudioContext.createMediaElementSource(hiddenAudioElement);
    mediaElementSource.connect(ttsAudioContext.destination);
}

export function setLipSyncNode(node: any) {
    lipsyncNodeRef = node;
    if (mediaElementSource && lipsyncNodeRef) {
        mediaElementSource.connect(lipsyncNodeRef);
    }
}

export function setPlaybackRate(rate: number) {
    ttsPlaybackRate = rate;
    if (hiddenAudioElement) {
        hiddenAudioElement.playbackRate = rate;
    }
}

export function sanitizeTTSInput(text: string): string {
    // Strip markdown formatting (*, _, ~, `)
    let sanitized = text.replace(/[*_~`]/g, '');
    // Strip emojis
    sanitized = sanitized.replace(/[\u{1F600}-\u{1F64F}\u{1F300}-\u{1F5FF}\u{1F680}-\u{1F6FF}\u{1F700}-\u{1F77F}\u{1F780}-\u{1F7FF}\u{1F800}-\u{1F8FF}\u{1F900}-\u{1F9FF}\u{1FA00}-\u{1FA6F}\u{1FA70}-\u{1FAFF}\u{2600}-\u{26FF}\u{2700}-\u{27BF}]/gu, '');
    // Replace newlines with spaces
    sanitized = sanitized.replace(/\n/g, ' ');
    // Strip LLM control tokens
    sanitized = sanitized.replace(/<start_of_turn>\s*model\s*/g, '');
    sanitized = sanitized.replace(/<start_of_turn>/g, '');
    sanitized = sanitized.replace(/<end_of_turn>/g, '');
    return sanitized;
}

export function chunkSentences(text: string, locale: string = 'en'): string[] {
    if (!text.trim()) return [];

    // Only pass locale to Segmenter if it's a valid BCP-47 tag format (e.g., 'en', 'pt-BR')
    // Otherwise, pass undefined to use the system's default locale.
    const isValidBcp47 = /^[a-z]{2,3}(-[a-zA-Z0-9]+)*$/i.test(locale);
    const segmenterLocale = isValidBcp47 ? locale : undefined;

    try {
        const sentenceSegmenter = new Intl.Segmenter(segmenterLocale, { granularity: 'sentence' });
        const sentences = Array.from(sentenceSegmenter.segment(text))
            .map(s => s.segment.trim())
            .filter(s => s.length > 0);

        return pairSegments(sentences);
    } catch (e) {
        console.warn(`Intl.Segmenter failed for locale '${segmenterLocale}', falling back to 'en'`, e);
        const sentenceSegmenter = new Intl.Segmenter('en', { granularity: 'sentence' });
        const sentences = Array.from(sentenceSegmenter.segment(text))
            .map(s => s.segment.trim())
            .filter(s => s.length > 0);
        return pairSegments(sentences);
    }
}

function pairSegments(segments: string[]): string[] {
    const paired: string[] = [];
    for (let i = 0; i < segments.length; i += 2) {
        const chunk = i + 1 < segments.length
            ? segments[i] + ' ' + segments[i + 1]
            : segments[i];
        paired.push(chunk.trim());
    }
    return paired.filter(s => s.length > 0);
}

async function synthesizeAndPlayChunk(
    text: string,
    providerUrl: string,
    targetLang: string
): Promise<boolean> {
    try {
        const rawAudioBytes: number[] = await invoke('generate_tts_audio', {
            text,
            language: targetLang,
            voice: 'sohee',
            speed: ttsPlaybackRate,
            url: providerUrl
        });

        if (rawAudioBytes.length === 0 || !hiddenAudioElement) return false;

        const blob = new Blob([new Uint8Array(rawAudioBytes)], { type: 'audio/wav' });
        const objectUrl = URL.createObjectURL(blob);

        try {
            hiddenAudioElement.src = objectUrl;
            hiddenAudioElement.playbackRate = ttsPlaybackRate;
            hiddenAudioElement.load();

            const finished = new Promise<void>((resolve) => {
                hiddenAudioElement!.onended = () => resolve();
                hiddenAudioElement!.onerror = () => resolve();
            });

            await hiddenAudioElement.play();
            await finished;
            return true;
        } catch (error) {
            console.warn(`[TTS] Playback failed for ${providerUrl}:`, error);
            return false;
        } finally {
            hiddenAudioElement.onended = null;
            hiddenAudioElement.onerror = null;
            URL.revokeObjectURL(objectUrl);
        }
    } catch (error) {
        console.warn(`[TTS] Synthesis failed for ${providerUrl}:`, error);
        return false;
    }
}

export async function playTTS(
    text: string,
    ttsServerUrl: string,
    onAnim?: (animCode: string, delayMs: number) => void,
    targetLang: string = 'en',
    fallbackServerUrl?: string
): Promise<boolean> {
    if (!ttsAudioContext) initTTSAudio();

    if (ttsAudioContext?.state === 'suspended') {
        await ttsAudioContext.resume();
    }

    const sanitized = sanitizeTTSInput(text);
    const animRegex = /[<\[]anim:([a-zA-Z0-9_-]+)[>\]]/g;
    const textWithAnchors = sanitized.replace(animRegex, '|||ANIM_$1|||');
    const chunks = chunkSentences(textWithAnchors, targetLang);
    const providerUrls = [ttsServerUrl, fallbackServerUrl]
        .filter((url): url is string => Boolean(url))
        .filter((url, index, values) => values.indexOf(url) === index);

    let playedAnyChunk = false;

    for (const chunk of chunks) {
        if (!chunk.trim()) continue;

        const anchorRegex = /\|\|\|ANIM_([a-zA-Z0-9_-]+)\|\|\|/g;
        const animations: string[] = [];
        let match: RegExpExecArray | null;
        while ((match = anchorRegex.exec(chunk)) !== null) animations.push(match[1]);

        const cleanChunk = chunk.replace(anchorRegex, '').trim();
        if (!cleanChunk) continue;

        let playedChunk = false;
        for (const providerUrl of providerUrls) {
            playedChunk = await synthesizeAndPlayChunk(cleanChunk, providerUrl, targetLang);
            if (!playedChunk) continue;

            playedAnyChunk = true;
            if (onAnim && animations.length > 0) onAnim(animations[0], 0);
            break;
        }

        if (!playedChunk) {
            console.error('[TTS] Unable to synthesize/play chunk with any configured provider:', cleanChunk);
        }
    }

    return playedAnyChunk;
}

export async function playSmartTTS(
    text: string,
    ttsServerUrl: string,
    onAnim?: (animCode: string, delayMs: number) => void,
    targetLang: string = 'en',
    policy: TtsProviderPolicy = 'auto'
) {
    const providers = resolveTtsProviderUrls(policy, isAndroidTauri(), ttsServerUrl);
    if (providers.length === 0) {
        console.warn('[TTS] No provider is configured for the selected policy.');
        return;
    }
    const [primary, fallback] = providers;
    await playTTS(text, primary, onAnim, targetLang, fallback);
}
