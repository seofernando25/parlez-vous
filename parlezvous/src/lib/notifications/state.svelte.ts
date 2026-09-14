import toast from 'svelte-french-toast';
import { isPermissionGranted, requestPermission } from '@tauri-apps/plugin-notification';
import type { NotificationSettings } from '$lib/types/notifications';

const STORAGE_KEY = 'parlezvous_notifications';

export const defaultNotificationSettings: NotificationSettings = {
    enabled: true,
    srs_reviews: true,
    daily_streak_reminder: true,
    study_break_alerts: true,
    evening_winddown: true,
    milestone_alerts: true,
    sound_enabled: true,
    frequency_minutes: 60
};

export const notificationState = $state<{
    settings: NotificationSettings;
    permission: NotificationPermission | 'unsupported';
    isInitialized: boolean;
}>({ settings: { ...defaultNotificationSettings }, permission: 'default', isInitialized: false });

export function loadNotificationSettings(): NotificationSettings {
    if (typeof window === 'undefined') return defaultNotificationSettings;
    try {
        const raw = localStorage.getItem(STORAGE_KEY);
        if (raw) notificationState.settings = { ...defaultNotificationSettings, ...JSON.parse(raw) };
    } catch (error) {
        console.warn('Failed to load notification settings:', error);
    }
    void resolvePermission();
    notificationState.isInitialized = true;
    return notificationState.settings;
}

async function resolvePermission() {
    try {
        notificationState.permission = await isPermissionGranted() ? 'granted' : 'default';
    } catch {
        try {
            const NativeNotification = (window as any).Notification;
            notificationState.permission = typeof NativeNotification === 'function' ? NativeNotification.permission || 'default' : 'default';
        } catch {
            notificationState.permission = 'default';
        }
    }
}

export function saveNotificationSettings(settings: NotificationSettings): void {
    notificationState.settings = { ...settings };
    if (typeof window === 'undefined') return;
    try {
        localStorage.setItem(STORAGE_KEY, JSON.stringify(notificationState.settings));
        toast.success('Notification preferences saved!');
    } catch (error) {
        console.warn('Failed to save notification settings:', error);
    }
}

export async function requestNotificationPermission(): Promise<boolean> {
    try {
        let granted = await isPermissionGranted();
        if (!granted) granted = await requestPermission() === 'granted';
        notificationState.permission = granted ? 'granted' : 'denied';
        if (granted) toast.success('System notifications enabled for ParlezVous!');
        else toast.error('Notification permission was denied or dismissed');
        return granted;
    } catch (error) {
        console.error('Tauri notification permission error:', error);
        try {
            const NativeNotification = (window as any).Notification;
            if (typeof NativeNotification === 'function') {
                const permission = await NativeNotification.requestPermission();
                notificationState.permission = permission;
                if (permission === 'granted') {
                    toast.success('System notifications enabled for ParlezVous!');
                    return true;
                }
            }
        } catch {}
        toast.error('Notification permission could not be granted');
        return false;
    }
}
