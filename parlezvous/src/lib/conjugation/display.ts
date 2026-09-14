import type { ConjugationExercise } from './types';
const escapeRegExp = (value: string) => value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');

function editDistance(a: string, b: string) {
    if (!a.length) return b.length;
    if (!b.length) return a.length;
    const matrix = Array.from({ length: a.length + 1 }, () => Array(b.length + 1).fill(0));
    for (let i = 0; i <= a.length; i++) matrix[i][0] = i;
    for (let j = 0; j <= b.length; j++) matrix[0][j] = j;
    for (let i = 1; i <= a.length; i++) for (let j = 1; j <= b.length; j++) {
        matrix[i][j] = Math.min(matrix[i][j - 1] + 1, matrix[i - 1][j] + 1, matrix[i - 1][j - 1] + (a[i - 1] === b[j - 1] ? 0 : 1));
    }
    return matrix[a.length][b.length];
}

function maskSplitAnswer(exercise: ConjugationExercise) {
    const parts = exercise.answer.trim().split(/\s+/);
    if (parts.length <= 1) return null;
    let pattern = '';
    for (let i = 0; i < parts.length; i++) {
        pattern += `(^|[\\s.,!?;:'])${escapeRegExp(parts[i])}(?=[\\s.,!?;:']|$)`;
        if (i < parts.length - 1) pattern += '(.*?)';
    }
    const match = exercise.sentence.match(new RegExp(pattern, 'i'));
    if (!match) return null;
    let result = '', group = 1;
    for (let i = 0; i < parts.length; i++) {
        result += match[group++] + '___';
        if (i < parts.length - 1) result += match[group++];
    }
    return exercise.sentence.substring(0, match.index) + result + exercise.sentence.substring(match.index! + match[0].length);
}

function maskFuzzyAnswer(exercise: ConjugationExercise) {
    const tokens: { text: string; start: number; end: number }[] = [];
    let offset = 0;
    for (const text of exercise.sentence.split(/([ \t\n.,!?;:'"()]+)/)) if (text) {
        tokens.push({ text, start: offset, end: offset + text.length }); offset += text.length;
    }
    const target = exercise.answer.trim().toLowerCase();
    const maxDistance = Math.max(1, Math.floor(target.length * 0.3));
    let best = { distance: Infinity, start: -1, end: -1 };
    for (let start = 0; start < tokens.length; start++) {
        if (!/[a-zàâçéèêëîïôûùüÿñæœ]/i.test(tokens[start].text)) continue;
        for (let end = start; end < tokens.length; end++) {
            if (!/[a-zàâçéèêëîïôûùüÿñæœ]/i.test(tokens[end].text)) continue;
            const text = exercise.sentence.substring(tokens[start].start, tokens[end].end).trim().toLowerCase();
            if (Math.abs(text.length - target.length) > Math.max(3, target.length * 0.5)) continue;
            const distance = editDistance(text, target);
            if (distance <= maxDistance && distance < best.distance) best = { distance, start: tokens[start].start, end: tokens[end].end };
        }
    }
    return best.start < 0 ? null : exercise.sentence.substring(0, best.start) + '___' + exercise.sentence.substring(best.end);
}

export function maskConjugation(exercise: ConjugationExercise) {
    const exact = new RegExp(escapeRegExp(exercise.answer), 'i');
    if (exact.test(exercise.sentence)) return exercise.sentence.replace(exact, '___');
    return maskSplitAnswer(exercise) ?? maskFuzzyAnswer(exercise) ?? exercise.sentence.replace(new RegExp(escapeRegExp(exercise.verb), 'i'), '___');
}
