<script lang="ts">
    import AiGate from '$lib/components/AiGate.svelte';
    import SpeedrunGame from '$lib/components/SpeedrunGame.svelte';
    import { codingState, initCodingQueue, popCodingPuzzle, stopCodingQueue } from '$lib/state/coding.svelte';
</script>

<AiGate feature="Code practice">
<SpeedrunGame 
    store={codingState}
    initQueue={initCodingQueue}
    popPuzzle={popCodingPuzzle}
    stopQueue={stopCodingQueue}
/>
</AiGate>
