<script lang="ts">
    import { useSettingsController } from '$lib/settings/context';
    import AccountSettings from '$lib/settings/components/AccountSettings.svelte';
    import LearningSettings from '$lib/settings/components/LearningSettings.svelte';
    import AppearanceSettings from '$lib/settings/components/AppearanceSettings.svelte';
    import AvatarSettings from '$lib/settings/components/AvatarSettings.svelte';
    const controller = useSettingsController();
</script>

<section class="settings-panel">
    <h1>General</h1>
    <p class="settings-lede">Personalize how you learn.</p>
    <div class="settings-group"><AccountSettings /><LearningSettings onSave={() => controller.save()} /><AvatarSettings onSave={() => controller.save()} /></div>
    <div class="settings-group appearance-group"><span class="group-label">Appearance</span><AppearanceSettings /></div>
</section>
