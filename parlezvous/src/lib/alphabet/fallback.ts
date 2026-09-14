import type { AlphabetItem, ScriptType } from './types';

const RUSSIAN = [
['А','а','a','[a]',1],['Б','б','b','[b]',0],['В','в','v','[v]',0],['Г','г','g','[ɡ]',0],['Д','д','d','[d]',0],['Е','е','ye','[je]',1],['Ё','ё','yo','[jo]',1],['Ж','ж','zh','[ʐ]',0],['З','з','z','[z]',0],['И','и','i','[i]',1],['Й','й','y','[j]',0],['К','к','k','[k]',0],['Л','л','l','[l]',0],['М','м','m','[m]',0],['Н','н','n','[n]',0],['О','о','o','[o]',1],['П','п','p','[p]',0],['Р','р','r','[r]',0],['С','с','s','[s]',0],['Т','т','t','[t]',0],['У','у','u','[u]',1],['Ф','ф','f','[f]',0],['Х','х','kh','[x]',0],['Ц','ц','ts','[ts]',0],['Ч','ч','ch','[tɕ]',0],['Ш','ш','sh','[ʂ]',0],['Щ','щ','shch','[ɕː]',0],['Ъ','ъ','ʺ','[silent]',0],['Ы','ы','y','[ɨ]',1],['Ь','ь','ʹ','[palatal]',0],['Э','э','e','[ɛ]',1],['Ю','ю','yu','[ju]',1],['Я','я','ya','[ja]',1]
] as const;
const UKRAINIAN = [
['А','а','a','[a]',1],['Б','б','b','[b]',0],['В','в','v','[w/v]',0],['Г','г','h','[ɦ]',0],['Ґ','ґ','g','[ɡ]',0],['Д','д','d','[d]',0],['Е','е','e','[ɛ]',1],['Є','є','ye','[je]',1],['Ж','ж','zh','[ʒ]',0],['З','з','z','[z]',0],['И','и','y','[ɪ]',1],['І','і','i','[i]',1],['Ї','ї','yi','[ji]',1],['Й','й','y','[j]',0],['К','к','k','[k]',0],['Л','л','l','[l]',0],['М','м','m','[m]',0],['Н','н','n','[n]',0],['О','о','o','[ɔ]',1],['П','п','p','[p]',0],['Р','р','r','[r]',0],['С','с','s','[s]',0],['Т','т','t','[t]',0],['У','у','u','[u]',1],['Ф','ф','f','[f]',0],['Х','х','kh','[x]',0],['Ц','ц','ts','[ts]',0],['Ч','ч','ch','[tʃ]',0],['Ш','ш','sh','[ʃ]',0],['Щ','щ','shch','[ʃtʃ]',0],['Ь','ь','ʹ','[palatal]',0],['Ю','ю','yu','[ju]',1],['Я','я','ya','[ja]',1]
] as const;
const ROMANIZATION: Record<string,string> = {'ㅏ':'a','ㅐ':'ae','ㅂ':'b','ㅃ':'bb','ㅊ':'ch','ㄷ':'d','ㅔ':'e','ㅓ':'eo','ㅡ':'eu','ㄱ':'g','ㄲ':'gg','ㅎ':'h','ㅣ':'i','ㅈ':'j','ㅋ':'k','ㅁ':'m','ㄴ':'n','ㅇ':'ng','ㅗ':'o','ㅍ':'p','ㄹ':'r','ㅅ':'s','ㅆ':'ss','ㅌ':'t','ㅜ':'u','ㅑ':'ya','ㅒ':'yae','ㅖ':'ye','ㅛ':'yo','ㅠ':'yu'};
const VOWELS = new Set(['a','ae','ya','yae','eo','e','ye','yo','u','yu','eu','i','o']);
const DEFAULT_JAMO = ['ㅏ','ㅐ','ㅂ','ㅃ','ㅊ','ㄷ','ㅔ','ㅓ','ㅡ','ㄱ','ㄲ','ㅎ','ㅣ','ㅈ','ㅋ','ㅁ','ㄴ','ㅇ','ㅗ','ㅍ','ㄹ','ㅅ','ㅆ','ㅌ','ㅜ','ㅑ','ㅒ','ㅖ','ㅛ','ㅠ'];

function fromCyrillic(rows: readonly (readonly [string,string,string,string,number])[], script: string): AlphabetItem[] {
    return rows.map(([upper,lower,romanization,pronunciation,vowel]) => ({ char: upper, uppercase: upper, lowercase: lower, name: upper, romanization, pronunciation, script, is_vowel: Boolean(vowel) }));
}

export function fallbackAlphabet(script: ScriptType, jamo: string[] = []): AlphabetItem[] {
    if (script === 'russian') return fromCyrillic(RUSSIAN, 'cyrillic');
    if (script === 'ukrainian') return fromCyrillic(UKRAINIAN, 'ukrainian');
    if (script === 'korean') return (jamo.length ? jamo : DEFAULT_JAMO).map(char => {
        const romanization = ROMANIZATION[char] ?? '';
        return { char, uppercase: char, lowercase: char, name: char, romanization, pronunciation: `[${romanization}]`, script: 'hangul', is_vowel: VOWELS.has(romanization) };
    });
    return 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'.split('').map(char => ({ char, uppercase: char, lowercase: char.toLowerCase(), name: char, romanization: char.toLowerCase(), pronunciation: `[${char.toLowerCase()}]`, script: 'latin', is_vowel: 'AEIOU'.includes(char) }));
}
