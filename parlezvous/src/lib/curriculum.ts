import { TIERS as FOUNDATION_TIERS } from './curriculum/tiers-foundation';
import { TIERS as ADVANCED_TIERS } from './curriculum/tiers-advanced';
export { XP_THRESHOLDS, type Theme, type Tier } from './curriculum/meta';
import { XP_THRESHOLDS, type Theme, type Tier } from './curriculum/meta';

export const CURRICULUM_TIERS: Tier[] = [...FOUNDATION_TIERS, ...ADVANCED_TIERS];

export function getTierFromXP(xp: number): number {
    let currentLevel = 1;
    for (let i = 1; i <= 7; i++) {
        if (xp >= XP_THRESHOLDS[i as keyof typeof XP_THRESHOLDS]) currentLevel = i;
        else break;
    }
    return currentLevel;
}

export function getThemeById(themeId: string): Theme | undefined {
    for (const tier of CURRICULUM_TIERS) {
        const theme = tier.themes.find(item => item.id === themeId);
        if (theme) return theme;
    }
}

export function getRandomThemeAndSubthemeForTier(level: number): { theme: string; subtheme: string } {
    const tier = CURRICULUM_TIERS.find(item => item.level === level) || CURRICULUM_TIERS[0];
    const theme = tier.themes[Math.floor(Math.random() * tier.themes.length)];
    return { theme: theme.id, subtheme: theme.subthemes[Math.floor(Math.random() * theme.subthemes.length)] };
}
