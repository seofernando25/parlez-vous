import { getContext, setContext } from 'svelte';
import type { SettingsController } from './controller.svelte';

const SETTINGS_CONTEXT = Symbol('settings-controller');

export function provideSettingsController(controller: SettingsController) {
    setContext(SETTINGS_CONTEXT, controller);
}

export function useSettingsController() {
    return getContext<SettingsController>(SETTINGS_CONTEXT);
}
