import { defineConfig } from 'vite';
import preact from '@preact/preset-vite';

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [preact()],
    esbuild: {
        jsxFactory: 'h',
        jsxFragment: 'Fragment',
    },
    build: {
        target: 'es2019',
        minify: 'esbuild',
        outDir: 'build/',
    },
});
