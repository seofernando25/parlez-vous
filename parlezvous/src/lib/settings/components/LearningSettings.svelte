<script lang="ts">
    import { settingsState } from '$lib/state/settings.svelte';
    let { onSave }: { onSave: () => void } = $props();
</script>

<div>
    <div class="settings-row">
        <label for="target-language">Learning language</label>
        <input id="target-language" class="settings-field" bind:value={settingsState.targetLanguage} onchange={onSave} placeholder="French" />
    </div>
    <div class="settings-row">
        <label for="skill-level">Level</label>
        <select id="skill-level" class="settings-field" bind:value={settingsState.skillLevel} onchange={onSave}>
            <option>Beginner</option><option>Intermediate</option><option>Advanced</option><option>Fluent</option>
        </select>
    </div>
    <div class="settings-row">
        <label for="tutor-tone">Tutor style</label>
        <select id="tutor-tone" class="settings-field" bind:value={settingsState.tutorTone} onchange={onSave}>
            <option value="chill">Chill</option>
            <option value="balanced">Balanced</option>
            <option value="focused">Focused</option>
        </select>
    </div>
</div>
