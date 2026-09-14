<script lang="ts">
    import { onMount } from 'svelte';
    import { Map, NotebookPen, MessagesSquare, Shapes, CircleUserRound, Settings, PanelLeftClose, PanelLeftOpen } from 'lucide-svelte';
    import { LEARNING_NAV, destinationForPath, isProfilePath, isSettingsPath } from '$lib/navigation';
    import { profileState } from '$lib/state/profile.svelte';

    let { pathname }: { pathname: string } = $props();
    let expanded = $state(true);
    const icons = { learn: Map, journal: NotebookPen, tutor: MessagesSquare, practice: Shapes };
    let active = $derived(destinationForPath(pathname));

    onMount(() => { expanded = localStorage.getItem('pv-sidebar-expanded') !== 'false'; });
    function toggle() { expanded = !expanded; localStorage.setItem('pv-sidebar-expanded', String(expanded)); }
</script>

<aside class="desktop-sidebar hidden md:flex" class:expanded aria-label="Primary navigation">
    <div class="sidebar-header">
        {#if expanded}<a class="brand-name" href="/map" aria-label="Parlez Vous">Parlez<span>Vous</span></a>{/if}
        <button class="sidebar-toggle" type="button" onclick={toggle} aria-label={expanded ? 'Collapse sidebar' : 'Expand sidebar'} title={expanded ? 'Collapse sidebar' : 'Expand sidebar'}>
            {#if expanded}<PanelLeftClose size={19} />{:else}<PanelLeftOpen size={19} />{/if}
        </button>
    </div>

    <nav class="sidebar-nav">
        {#each LEARNING_NAV as item}
            {@const Icon = icons[item.id as keyof typeof icons]}
            <a href={item.href} title={expanded ? undefined : item.label} aria-label={item.label} aria-current={active === item.id ? 'page' : undefined} class:active={active === item.id}>
                <Icon size={21} strokeWidth={active === item.id ? 2.35 : 2} />
                {#if expanded}<span>{item.label}</span>{/if}
            </a>
        {/each}
    </nav>

    <div class="sidebar-account">
        <a class="account-user" class:active={isProfilePath(pathname)} href="/profile" title={expanded ? undefined : profileState.displayName} aria-label={profileState.displayName}>
            <CircleUserRound size={21} />
            {#if expanded}<span>{profileState.displayName}</span>{/if}
        </a>
        <a class="account-settings" class:active={isSettingsPath(pathname)} href="/settings/general" title="Settings" aria-label="Settings"><Settings size={20} /></a>
    </div>
</aside>
