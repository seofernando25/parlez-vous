<script lang="ts">
    import { useSettingsController } from '$lib/settings/context';
    import ModelSettings from '$lib/settings/components/ModelSettings.svelte';
    import OnDeviceSettings from '$lib/settings/components/OnDeviceSettings.svelte';
    const controller = useSettingsController();
</script>

<section class="settings-panel">
    <h1>AI & Speech</h1>
    <p class="settings-lede">Models, speech and on-device assets.</p>
    <div class="settings-group"><ModelSettings isAndroid={controller.isAndroid} onSave={() => controller.save()} /><OnDeviceSettings {controller} /></div>
</section>
