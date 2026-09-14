<script lang="ts">
    import { BookOpen, Languages, ScanText, Image, Zap, Braces } from 'lucide-svelte';
    const tools = [
        { href: '/flashcards', label: 'Vocabulary', icon: BookOpen },
        { href: '/conjugator', label: 'Conjugation', icon: Languages },
        { href: '/canvas', label: 'Alphabet', icon: ScanText },
        { href: '/vision', label: 'Vision', icon: Image },
        { href: '/language-game', label: 'Sprint', icon: Zap },
        { href: '/coding', label: 'Code', icon: Braces }
    ];
</script>

<div class="app-page app-page--medium">
    <header class="page-heading">
        <h1 class="page-title">Practice</h1>
    </header>

    <div class="practice-grid">
        {#each tools as tool}
            {@const Icon = tool.icon}
            <a href={tool.href} class="practice-tile">
                <Icon size={27} strokeWidth={1.9} />
                <span>{tool.label}</span>
            </a>
        {/each}
    </div>
</div>

<style>
    .practice-grid { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 0.75rem; }
    .practice-tile {
        display: flex; min-height: 8.5rem; flex-direction: column; align-items: center; justify-content: center; gap: 0.75rem;
        border: 1px solid var(--pv-border); border-radius: 1rem; background: var(--pv-surface); color: var(--pv-foreground);
        font-size: 0.9rem; font-weight: 700; text-decoration: none; transition: background 140ms ease, border-color 140ms ease, transform 100ms ease;
    }
    .practice-tile :global(svg) { color: var(--pv-accent-strong); }
    .practice-tile:hover { background: var(--pv-surface-raised); border-color: var(--pv-border-strong); }
    .practice-tile:active { transform: scale(0.98); }
    @media (min-width: 720px) { .practice-grid { grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 1rem; } .practice-tile { min-height: 10rem; } }
</style>
