import { defineConfig } from 'vitest/config';
import { sveltekit } from '@sveltejs/kit/vite';

export default defineConfig({
    plugins: [sveltekit()],
    resolve: {
        conditions: ['mode=test', 'browser']
    },
    test: {
        environment: 'jsdom',
        setupFiles: ['./src/setupTest.ts'],
        globals: true,
    },
});
