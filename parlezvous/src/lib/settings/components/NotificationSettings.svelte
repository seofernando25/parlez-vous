<script lang="ts">
    import { Send } from 'lucide-svelte';
    import { notificationState, requestNotificationPermission, saveNotificationSettings, sendTestNotification } from '$lib/services/notifications.svelte';
    const options = [
        ['srs_reviews', 'Flashcard reminders'], ['daily_streak_reminder', 'Daily reminder'], ['study_break_alerts', 'Study breaks'],
        ['evening_winddown', 'Evening review'], ['milestone_alerts', 'Milestones'], ['sound_enabled', 'Sound']
    ] as const;
    const save = () => saveNotificationSettings(notificationState.settings);
</script>

<div>
    <label class="settings-row toggle-row"><span>Notifications</span><input type="checkbox" bind:checked={notificationState.settings.enabled} onchange={save} aria-label="Enable notifications" /></label>
    {#if notificationState.settings.enabled}
        {#each options as option}
            <label class="settings-row toggle-row"><span>{option[1]}</span><input type="checkbox" bind:checked={notificationState.settings[option[0]]} onchange={save} /></label>
        {/each}
        <div class="settings-row"><label for="notification-cooldown">Cooldown</label><select id="notification-cooldown" class="settings-field" bind:value={notificationState.settings.frequency_minutes} onchange={save}><option value={30}>30 minutes</option><option value={60}>1 hour</option><option value={120}>2 hours</option><option value={240}>4 hours</option></select></div>
        <div class="notification-actions">
            {#if notificationState.permission !== 'granted'}<button onclick={requestNotificationPermission}>Enable system access</button>{/if}
            <button class="icon-button" onclick={sendTestNotification} title="Test notification" aria-label="Test notification"><Send size={16} /></button>
        </div>
    {/if}
</div>

<style>
    .toggle-row input { justify-self:end; width:1.05rem; height:1.05rem; accent-color:var(--pv-accent); }
    .notification-actions { display:flex; align-items:center; justify-content:flex-end; gap:.5rem; padding-top:1rem; }
    .notification-actions > button:first-child:not(.icon-button) { border:0; background:transparent; color:var(--pv-accent-strong); font-size:.78rem; font-weight:750; cursor:pointer; }
</style>
