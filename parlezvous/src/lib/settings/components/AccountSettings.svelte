<script lang="ts">
    import { profileState, saveDisplayName } from '$lib/state/profile.svelte';
</script>

<div class="settings-row">
    <label for="display-name">Name</label>
    <input id="display-name" class="settings-field" bind:value={profileState.displayName} onchange={() => saveDisplayName()} maxlength="48" autocomplete="nickname" />
</div>
