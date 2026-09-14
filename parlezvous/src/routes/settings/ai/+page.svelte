<script lang="ts">
    import { useSettingsController } from '$lib/settings/context';
    import ManagedAiSettings from '$lib/settings/components/ManagedAiSettings.svelte';
    import OnDeviceSettings from '$lib/settings/components/OnDeviceSettings.svelte';
    const controller = useSettingsController();
</script>

<section class="settings-panel">
    <h1>AI & Speech</h1>
    <div class="settings-group">{#if !controller.isAndroid}<ManagedAiSettings />{:else}<OnDeviceSettings {controller} />{/if}</div>
</section>
