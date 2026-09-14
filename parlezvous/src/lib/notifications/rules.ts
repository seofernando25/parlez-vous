import { invoke } from '@tauri-apps/api/core';
import { settingsState } from '$lib/state/settings.svelte';
import { timeTracker } from '$lib/state/timeTracker.svelte';
import { sendParlezNotification } from './delivery';
import { notificationState } from './state.svelte';

const lastAlertTimestamps: Record<string, number> = {};

function coolingDown(key: string, now: number, duration: number) {
    return now - (lastAlertTimestamps[key] || 0) <= duration;
}
function markSent(key: string, now: number) { lastAlertTimestamps[key] = now; }

export async function evaluateStudyNotifications(): Promise<void> {
    if (!notificationState.settings.enabled) return;
    const now = Date.now();
    const targetLanguage = settingsState.targetLanguage || 'Target Language';
    const configuredCooldown = (notificationState.settings.frequency_minutes || 60) * 60_000;

    if (await maybeNotifySrs(now, targetLanguage, configuredCooldown)) return;
    if (maybeNotifyDailyStreak(now, targetLanguage)) return;
    if (maybeNotifyStudyBreak(now, targetLanguage)) return;
    maybeNotifyEveningReview(now, targetLanguage);
}

async function maybeNotifySrs(now: number, language: string, configuredCooldown: number) {
    if (!notificationState.settings.srs_reviews) return false;
    if (coolingDown('srs_reviews', now, Math.max(configuredCooldown, 2 * 60 * 60_000))) return false;
    try {
        const vocabulary = await invoke<any[]>('get_all_vocabulary').catch(() => []);
        if (!vocabulary?.length) return false;
        markSent('srs_reviews', now);
        return sendParlezNotification({
            title: 'ParlezVous: Flashcards Ready!',
            body: `📚 You have ${vocabulary.length} vocabulary words ready for review in ${language}.`,
            category: 'srs_reviews', icon: '📚'
        });
    } catch { return false; }
}

function maybeNotifyDailyStreak(now: number, language: string) {
    if (!notificationState.settings.daily_streak_reminder) return false;
    const hour = new Date().getHours();
    if (hour < 14 || hour > 19 || coolingDown('daily_streak', now, 18 * 60 * 60_000) || timeTracker.activeSeconds >= 120) return false;
    markSent('daily_streak', now);
    return sendParlezNotification({
        title: 'ParlezVous: Daily Practice Reminder!',
        body: `🔥 Keep your study streak alive with 5 minutes of ${language}.`,
        category: 'daily_streak_reminder', icon: '🔥'
    });
}

function maybeNotifyStudyBreak(now: number, language: string) {
    if (!notificationState.settings.study_break_alerts || timeTracker.activeSeconds < 1800 || coolingDown('study_break', now, 30 * 60_000)) return false;
    markSent('study_break', now);
    return sendParlezNotification({
        title: 'ParlezVous: Time for a Quick Break!',
        body: `☕ Great focus on ${language}. Rest your eyes, stretch, and grab some water.`,
        category: 'study_break_alerts', icon: '☕'
    });
}

function maybeNotifyEveningReview(now: number, language: string) {
    if (!notificationState.settings.evening_winddown) return false;
    const date = new Date();
    const inWindow = date.getHours() === 21 || (date.getHours() === 22 && date.getMinutes() <= 30);
    if (!inWindow || coolingDown('evening_consolidation', now, 18 * 60 * 60_000)) return false;
    markSent('evening_consolidation', now);
    return sendParlezNotification({
        title: 'ParlezVous: Evening Consolidation!',
        body: `🌙 Review today's ${language} vocabulary before bed.`,
        category: 'evening_winddown', icon: '🌙'
    });
}
