<script lang="ts">
    import '../app.css';
    import { onMount } from 'svelte';
    import { page } from '$app/stores';
    import { Toaster } from 'svelte-french-toast';
    import { checkAiHealth } from '$lib/state/aiProvider.svelte';
    import { refreshManagedAi } from '$lib/state/managedAi.svelte';
    import { settingsState } from '$lib/state/settings.svelte';
    import { loadSettings } from '$lib/state/settings.svelte';
    import { loadProfile } from '$lib/state/profile.svelte';
    import { loadTheme } from '$lib/state/theme.svelte';
    import { startNotificationTicker, loadNotificationSettings } from '$lib/services/notifications.svelte';
    import DesktopSidebar from '$lib/components/DesktopSidebar.svelte';
    import MobileTabBar from '$lib/components/MobileTabBar.svelte';

    let { children } = $props();
    let pathname = $derived($page.url.pathname);
    let isSetup = $derived(pathname.startsWith('/setup/'));

    onMount(() => {
        loadTheme();
        loadNotificationSettings();
        startNotificationTicker();
        void (async () => {
            await Promise.all([loadSettings(), loadProfile()]);
            if (settingsState.aiProvider === 'managed') await refreshManagedAi();
            else await checkAiHealth();
        })();
    });
</script>

<Toaster />
{#if isSetup}
    <main class="setup-shell">{@render children()}</main>
{:else}
    <div class="app-shell">
        <DesktopSidebar {pathname} />
        <main class="app-content">{@render children()}</main>
        <MobileTabBar {pathname} />
    </div>
{/if}

<style>
    .setup-shell { min-height:100dvh; background:var(--pv-canvas); color:var(--pv-foreground); }
</style>
