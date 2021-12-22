/// <reference types="vitest" />
/// <reference types="vite/client" />

import { defineConfig } from 'vite';
import { resolve } from 'path';
import react from '@vitejs/plugin-react';

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [react()],
    resolve: {
        alias: {
            '@': resolve(__dirname, 'src'),
        },
    },
    build: {
        target: 'es2020',
        minify: 'esbuild',
    },
    test: {
        // environment: 'happy-dom', // doesn't work :(   TypeError: Cannot read properties of undefined (reading 'length')
        environment: 'jsdom',
        watch: false,
        setupFiles: ['./test/setup.ts'],
        root: './',
        global: true,
        deps: {
            inline: ['react-focus-lock'],
        },
    },
});
