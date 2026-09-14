import { isLiteRtModel } from './capabilities';
import { isAndroidTauri } from '$lib/platform';
import { aiProviderState } from '$lib/state/aiProvider.svelte';
import { managedAiState } from '$lib/state/managedAi.svelte';
import { settingsState } from '$lib/state/settings.svelte';

export function isAiReady(): boolean {
    if (isAndroidTauri() && isLiteRtModel(settingsState.activeModel)) return true;
    if (settingsState.aiProvider === 'managed') return managedAiState.ready;
    return aiProviderState.isHealthy && Boolean(settingsState.activeModel.trim());
}
