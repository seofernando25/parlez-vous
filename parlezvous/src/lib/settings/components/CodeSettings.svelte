<script lang="ts">
    import { settingsState } from '$lib/state/settings.svelte';
    let { categories, onSave }: { categories: string[]; onSave: () => void } = $props();
</script>
<div>
    <div class="settings-row"><label for="programming-language">Code language</label><input id="programming-language" class="settings-field" bind:value={settingsState.targetProgrammingLanguage} onchange={onSave} placeholder="Python" /></div>
    <div class="settings-row"><label for="coding-category">Code topics</label><select id="coding-category" class="settings-field" bind:value={settingsState.codingThemeCategory} onchange={onSave}><option value="All">All</option>{#each categories as category}<option value={category}>{category}</option>{/each}</select></div>
</div>
