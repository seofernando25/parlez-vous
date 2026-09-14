<script lang="ts">
    import { Map, NotebookPen, MessagesSquare, Shapes, CircleUserRound } from 'lucide-svelte';
    import { MOBILE_NAV, destinationForPath } from '$lib/navigation';
    let { pathname }: { pathname: string } = $props();
    const icons = { learn: Map, journal: NotebookPen, tutor: MessagesSquare, practice: Shapes, profile: CircleUserRound };
    let active = $derived(destinationForPath(pathname));
</script>

<nav class="mobile-tab-bar md:hidden" aria-label="Primary navigation">
    {#each MOBILE_NAV as item}
        {@const Icon = icons[item.id]}
        <a
            href={item.href}
            aria-label={item.label}
            title={item.label}
            aria-current={active === item.id ? 'page' : undefined}
            class:active={active === item.id}
        >
            <Icon size={23} strokeWidth={active === item.id ? 2.4 : 2} />
        </a>
    {/each}
</nav>
