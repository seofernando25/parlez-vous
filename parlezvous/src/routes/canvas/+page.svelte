<script lang="ts">
    import { AlphabetPracticeController } from '$lib/alphabet/controller.svelte';
    import DrawingPractice from '$lib/alphabet/components/DrawingPractice.svelte';
    import LetterCard from '$lib/alphabet/components/LetterCard.svelte';
    import ScriptTabs from '$lib/alphabet/components/ScriptTabs.svelte';
    const practice = new AlphabetPracticeController();
</script>

<div class="app-page app-page--narrow alphabet-page">
    <header class="page-heading"><h1 class="page-title">Alphabet</h1></header>
    <ScriptTabs active={practice.activeScript} onSelect={script => practice.loadAlphabet(script)} />
    <LetterCard {practice} />
    <DrawingPractice {practice} />
</div>

<style>
    .alphabet-page { display:flex; flex-direction:column; gap:1rem; align-items:center; }
    .alphabet-page :global(> *) { width:min(100%,28rem); }
    .alphabet-page :global(.page-heading) { width:100%; }
</style>
