import { browser } from '$app/environment';

export type ThemeMode = 'system' | 'light' | 'dark';

const STORAGE_KEY = 'parlezvous-theme';

export const themeState = $state({
    mode: 'system' as ThemeMode,
    isLoaded: false
});

function isThemeMode(value: string | null): value is ThemeMode {
    return value === 'system' || value === 'light' || value === 'dark';
}

function applyTheme(mode: ThemeMode) {
    if (!browser) return;
    if (mode === 'system') {
        document.documentElement.removeAttribute('data-theme');
    } else {
        document.documentElement.dataset.theme = mode;
    }
}

export function loadTheme() {
    if (!browser) return;
    const stored = localStorage.getItem(STORAGE_KEY);
    themeState.mode = isThemeMode(stored) ? stored : 'system';
    applyTheme(themeState.mode);
    themeState.isLoaded = true;
}

export function setTheme(mode: ThemeMode) {
    themeState.mode = mode;
    if (browser) {
        localStorage.setItem(STORAGE_KEY, mode);
        applyTheme(mode);
    }
}
