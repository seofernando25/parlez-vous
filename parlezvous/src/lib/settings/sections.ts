import { SlidersHorizontal, Sparkles, Bell, Wrench } from 'lucide-svelte';

export const SETTINGS_SECTIONS = [
    { href: '/settings/general', label: 'General', icon: SlidersHorizontal },
    { href: '/settings/ai', label: 'AI & Speech', icon: Sparkles },
    { href: '/settings/notifications', label: 'Notifications', icon: Bell },
    { href: '/settings/advanced', label: 'Advanced', icon: Wrench }
] as const;

export function settingsSection(pathname: string) {
    return SETTINGS_SECTIONS.find(section => pathname === section.href)?.label ?? 'Settings';
}
