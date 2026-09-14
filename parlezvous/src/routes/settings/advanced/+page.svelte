<script lang="ts">
    import { useSettingsController } from '$lib/settings/context';
    import AdvancedSettings from '$lib/settings/components/AdvancedSettings.svelte';
    import ModelSettings from '$lib/settings/components/ModelSettings.svelte';
    import CodeSettings from '$lib/settings/components/CodeSettings.svelte';
    const controller = useSettingsController();
</script>
<section class="settings-panel">
    <h1>Advanced</h1>
    <p class="settings-lede">Servers, credentials and inference controls.</p>
    <div class="settings-group"><ModelSettings isAndroid={controller.isAndroid} onSave={() => controller.save()} /></div>
    <div class="settings-group"><CodeSettings categories={controller.codingCategories} onSave={() => controller.save()} /></div>
    <div class="settings-group"><AdvancedSettings {controller} onSave={() => controller.save()} /></div>
</section>
