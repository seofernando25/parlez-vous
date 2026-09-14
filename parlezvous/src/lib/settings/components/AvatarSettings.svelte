<script lang="ts">
    import { settingsState } from '$lib/state/settings.svelte';
    import { BUILT_IN_VRMS } from '$lib/vrm';
    let { onSave }: { onSave: () => void } = $props();
</script>
<div class="settings-row">
    <label for="avatar-model">Avatar</label>
    <select id="avatar-model" class="settings-field" bind:value={settingsState.activeVrm} onchange={onSave}>
        {#each BUILT_IN_VRMS as vrm}<option value={vrm}>{vrm.replace('.vrm', '')}</option>{/each}
    </select>
</div>
