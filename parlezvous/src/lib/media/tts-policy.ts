export type TtsProviderPolicy = 'auto' | 'supertonic' | 'server';

export function resolveTtsProviderUrls(
    policy: TtsProviderPolicy,
    isAndroid: boolean,
    serverUrl: string
): string[] {
    const server = serverUrl.trim();
    if (!isAndroid || policy === 'server') return server ? [server] : [];
    if (policy === 'supertonic') return ['supertonic://local'];
    return server && !server.startsWith('supertonic')
        ? ['supertonic://local', server]
        : ['supertonic://local'];
}
