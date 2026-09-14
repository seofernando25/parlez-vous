import { settingsState } from '$lib/state/settings.svelte';
import { sendParlezNotification } from './delivery';
import { evaluateStudyNotifications } from './rules';
import { loadNotificationSettings } from './state.svelte';

let tickerInterval: ReturnType<typeof setInterval> | null = null;

export function sendTestNotification(): void {
    sendParlezNotification({
        title: 'ParlezVous: Notification System Active!',
        body: `✨ Alerts are operational for your ${settingsState.targetLanguage || 'French'} studies.`,
        category: 'test', icon: '✨'
    });
}

export function startNotificationTicker(): void {
    if (typeof window === 'undefined') return;
    loadNotificationSettings();
    if (tickerInterval) clearInterval(tickerInterval);
    tickerInterval = setInterval(() => void evaluateStudyNotifications().catch(error => {
        console.warn('[Notifications] Error in ticker evaluation:', error);
    }), 60_000);
}
