<script lang="ts">
    import AiGate from '$lib/components/AiGate.svelte';
    import SpeedrunGame from '$lib/components/SpeedrunGame.svelte';
    import { languageSpeedrunState, initLanguageQueue, popLanguagePuzzle, stopLanguageQueue } from '$lib/state/languageSpeedrun.svelte';
</script>

<AiGate feature="Sprint">
<SpeedrunGame 
    store={languageSpeedrunState}
    initQueue={initLanguageQueue}
    popPuzzle={popLanguagePuzzle}
    stopQueue={stopLanguageQueue}
    isLanguageGame={true}
/>
</AiGate>
