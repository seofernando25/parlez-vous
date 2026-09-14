import { readdirSync, readFileSync, statSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';

const MAX_LOC = 300;
const repoRoot = resolve(import.meta.dir, '..', '..');
const roots = [
    'parlezvous/src',
    'parlezvous/src-tauri/src',
    'parlezvous/plugins/tauri-plugin-litert/src',
    'parlezvous/plugins/tauri-plugin-litert/android/src/main/java',
    'parlezvous/plugins/tauri-plugin-supertonic/src',
    'parlezvous/plugins/tauri-plugin-supertonic/android/src/main/java',
    'website/src'
].map(path => join(repoRoot, path));
const extensions = new Set(['.ts', '.js', '.svelte', '.css', '.rs', '.kt', '.kts']);

function filesUnder(directory: string): string[] {
    if (!statSync(directory).isDirectory()) return [];
    return readdirSync(directory).flatMap(name => {
        const path = join(directory, name);
        return statSync(path).isDirectory() ? filesUnder(path) : [path];
    });
}

const oversized = roots.flatMap(filesUnder).filter(path => {
    const dot = path.lastIndexOf('.');
    return dot >= 0 && extensions.has(path.slice(dot));
}).map(path => ({
    path: relative(repoRoot, path),
    lines: readFileSync(path, 'utf8').split(/\r?\n/).length
})).filter(file => file.lines > MAX_LOC).sort((a, b) => b.lines - a.lines);

if (oversized.length) {
    console.error(`Source files must be <= ${MAX_LOC} LOC:`);
    for (const file of oversized) console.error(`  ${file.lines.toString().padStart(4)}  ${file.path}`);
    process.exit(1);
}
console.log(`LOC invariant passed: all authored source files are <= ${MAX_LOC} lines.`);
