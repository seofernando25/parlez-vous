export type PrimaryDestination = 'learn' | 'journal' | 'tutor' | 'practice' | 'profile';

export interface NavItem {
    id: PrimaryDestination;
    href: string;
    label: string;
}

export const LEARNING_NAV: NavItem[] = [
    { id: 'learn', href: '/map', label: 'Learn' },
    { id: 'journal', href: '/journal', label: 'Journal' },
    { id: 'tutor', href: '/avatar', label: 'Tutor' },
    { id: 'practice', href: '/practice', label: 'Practice' }
];

export const MOBILE_NAV: NavItem[] = [
    ...LEARNING_NAV,
    { id: 'profile', href: '/profile', label: 'Profile' }
];

const PRACTICE_ROUTES = new Set(['/practice', '/flashcards', '/conjugator', '/canvas', '/vision', '/language-game', '/coding']);
const PROFILE_ROUTES = new Set(['/profile', '/stats', '/calendar']);

export function destinationForPath(pathname: string): PrimaryDestination | null {
    if (pathname === '/map' || pathname === '/') return 'learn';
    if (pathname === '/journal') return 'journal';
    if (pathname === '/avatar') return 'tutor';
    if (PRACTICE_ROUTES.has(pathname)) return 'practice';
    if (PROFILE_ROUTES.has(pathname) || pathname.startsWith('/settings')) return 'profile';
    return null;
}

export function isSettingsPath(pathname: string) { return pathname === '/settings' || pathname.startsWith('/settings/'); }
export function isProfilePath(pathname: string) { return pathname === '/profile' || pathname === '/stats' || pathname === '/calendar'; }
