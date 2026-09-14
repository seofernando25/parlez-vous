export type ModelProvider = 'remote' | 'litert';

export interface ModelCapabilities {
    provider: ModelProvider;
    isOnDevice: boolean;
    acceptsAudio: boolean;
    acceptsImage: boolean;
    supportsKeystonePuzzles: boolean;
}

export function isLiteRtModel(model: string): boolean {
    return model.trim().toLowerCase().endsWith('.litertlm');
}

export function getModelCapabilities(model: string): ModelCapabilities {
    if (isLiteRtModel(model)) {
        return {
            provider: 'litert',
            isOnDevice: true,
            acceptsAudio: true,
            acceptsImage: true,
            supportsKeystonePuzzles: false
        };
    }

    return {
        provider: 'remote',
        isOnDevice: false,
        acceptsAudio: false,
        acceptsImage: false,
        supportsKeystonePuzzles: true
    };
}
