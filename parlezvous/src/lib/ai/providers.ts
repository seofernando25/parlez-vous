export type AiProviderPreset = 'managed' | 'ollama' | 'lmstudio' | 'openrouter' | 'openai' | 'custom';

export interface ProviderPreset {
    id: AiProviderPreset;
    label: string;
    baseUrl?: string;
    local: boolean;
}

export const AI_PROVIDER_PRESETS: ProviderPreset[] = [
    { id: 'managed', label: 'On this device', baseUrl: 'http://127.0.0.1:11435/v1', local: true },
    { id: 'ollama', label: 'Ollama', baseUrl: 'http://localhost:11434/v1', local: true },
    { id: 'lmstudio', label: 'LM Studio', baseUrl: 'http://localhost:1234/v1', local: true },
    { id: 'openrouter', label: 'OpenRouter', baseUrl: 'https://openrouter.ai/api/v1', local: false },
    { id: 'openai', label: 'OpenAI', baseUrl: 'https://api.openai.com/v1', local: false },
    { id: 'custom', label: 'OpenAI-compatible', local: false }
];

export function providerPreset(id: string): ProviderPreset {
    return AI_PROVIDER_PRESETS.find(provider => provider.id === id) ?? AI_PROVIDER_PRESETS.at(-1)!;
}
