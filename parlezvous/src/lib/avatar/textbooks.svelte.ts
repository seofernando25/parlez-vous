import { convertFileSrc, invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

export class TextbookController {
    isIngesting = $state(false);
    books = $state<string[]>([]);
    active = $state<string | null>(null);
    src = $state<string | null>(null);
    hasError = $state(false);
    activePage = $state<number | null>(null);
    private directory = $state('');

    async load() {
        try {
            this.books = await invoke<string[]>('list_textbooks');
            if (!this.directory) this.directory = await invoke<string>('get_textbook_dir');
        } catch (error) {
            console.error('Failed to load textbooks:', error);
        }
    }

    select(name: string | null) {
        this.active = name;
        if (!name || !this.directory) {
            this.src = null;
            this.hasError = false;
            return;
        }
        try {
            this.src = convertFileSrc(`${this.directory}/${name}`);
            this.hasError = false;
        } catch (error) {
            console.error('Failed to resolve textbook path:', error);
            this.src = null;
            this.hasError = true;
        }
    }

    dismiss() { this.select(null); }

    async upload() {
        try {
            const selected = await open({ multiple: false, filters: [{ name: 'Textbooks', extensions: ['pdf'] }] });
            if (!selected) return;
            const filePath = typeof selected === 'string' ? selected : (selected as any).path;
            this.isIngesting = true;
            await invoke('upload_and_ingest_textbook', { filePath, model: 'nomic-embed-text-v2-moe' });
            await this.load();
            const name = typeof selected === 'string' ? selected.split(/[/\\]/).pop() || null : (selected as any).name || null;
            this.select(name);
        } catch (error) {
            console.error('Upload/Ingestion failed:', error);
            alert(`Failed to upload: ${error}`);
        } finally {
            this.isIngesting = false;
        }
    }
}
