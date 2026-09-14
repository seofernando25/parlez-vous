import toast from 'svelte-french-toast';
import { sendNotification } from '@tauri-apps/plugin-notification';
import type { ParlezNotificationOptions } from '$lib/types/notifications';
import { notificationState } from './state.svelte';

export function playParlezChime(): void {
    if (!notificationState.settings.sound_enabled || typeof window === 'undefined') return;
    try {
        const AudioContextCtor = window.AudioContext || (window as any).webkitAudioContext;
        if (!AudioContextCtor) return;
        const context = new AudioContextCtor();
        const now = context.currentTime;
        [523.25, 659.25, 783.99].forEach((frequency, index) => {
            const oscillator = context.createOscillator();
            const gain = context.createGain();
            oscillator.type = 'sine';
            oscillator.frequency.setValueAtTime(frequency, now + index * 0.1);
            gain.gain.setValueAtTime(0.15, now + index * 0.1);
            gain.gain.exponentialRampToValueAtTime(0.001, now + index * 0.1 + 0.35);
            oscillator.connect(gain);
            gain.connect(context.destination);
            oscillator.start(now + index * 0.1);
            oscillator.stop(now + index * 0.1 + 0.35);
        });
    } catch (error) {
        console.warn('[Notifications] Audio chime unavailable:', error);
    }
}

export function sendParlezNotification(options: ParlezNotificationOptions): boolean {
    if (!notificationState.settings.enabled) return false;
    if (options.category !== 'test' && notificationState.settings[options.category] === false) return false;

    playParlezChime();
    toast(options.body, {
        icon: options.icon || '💬',
        duration: 6000,
        style: 'border:1px solid color-mix(in oklch,var(--pv-accent) 40%,transparent);background:var(--pv-surface);color:var(--pv-foreground);font-size:13px;font-weight:600;'
    });

    try {
        sendNotification({ title: options.title, body: options.body });
    } catch (error) {
        console.warn('Native notification dispatch error:', error);
        try {
            const NativeNotification = (window as any).Notification;
            if (typeof NativeNotification === 'function' && NativeNotification.permission === 'granted') {
                new NativeNotification(options.title, {
                    body: options.body,
                    icon: '/icons/128x128.png',
                    tag: `parlezvous-${options.category}`
                });
            }
        } catch (fallbackError) {
            console.warn('Web notification fallback error:', fallbackError);
        }
    }
    return true;
}
