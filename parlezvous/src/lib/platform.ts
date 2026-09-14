export function isTauriRuntime(): boolean {
    return typeof window !== 'undefined' && Boolean((window as any).__TAURI_INTERNALS__);
}

export function isAndroidTauri(): boolean {
    return isTauriRuntime()
        && typeof navigator !== 'undefined'
        && navigator.userAgent.toLowerCase().includes('android');
}
