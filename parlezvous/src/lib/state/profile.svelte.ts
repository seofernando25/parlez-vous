import { invoke } from '@tauri-apps/api/core';

interface ProfileDto {
    skill_level: string;
    tier: number;
    active_seconds: number;
    display_name: string;
}

export const profileState = $state({
    displayName: 'Learner',
    tier: 1,
    activeSeconds: 0,
    isLoaded: false
});

export async function loadProfile() {
    try {
        const profile = await invoke<ProfileDto>('get_profile');
        profileState.displayName = profile.display_name || 'Learner';
        profileState.tier = profile.tier;
        profileState.activeSeconds = profile.active_seconds;
    } catch (error) {
        console.error('Failed to load profile:', error);
    } finally {
        profileState.isLoaded = true;
    }
}

export async function saveDisplayName() {
    const displayName = profileState.displayName.trim() || 'Learner';
    profileState.displayName = displayName;
    await invoke('set_profile_display_name', { displayName });
}
