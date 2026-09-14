export type ScriptType = 'russian' | 'ukrainian' | 'korean' | 'latin';

export interface AlphabetItem {
    char: string;
    uppercase: string;
    lowercase: string;
    name: string;
    romanization: string;
    pronunciation: string;
    script: string;
    is_vowel: boolean;
}

export const SCRIPT_INFO: Record<ScriptType, { title: string; langCode: string; icon: string }> = {
    russian: { title: 'Russian Cyrillic', langCode: 'ru', icon: '🇷🇺' },
    ukrainian: { title: 'Ukrainian Cyrillic', langCode: 'uk', icon: '🇺🇦' },
    korean: { title: 'Korean Hangul', langCode: 'ko', icon: '🇰🇷' },
    latin: { title: 'Latin', langCode: 'en', icon: '🔤' }
};

export function scriptForTargetLanguage(language: string): ScriptType {
    const value = language.trim().toLowerCase();
    if (value.includes('korean')) return 'korean';
    if (value.includes('ukrain')) return 'ukrainian';
    if (value.includes('russ') || value.includes('cyrillic')) return 'russian';
    if (['french', 'span', 'germ', 'ital'].some(name => value.includes(name))) return 'latin';
    return 'russian';
}
