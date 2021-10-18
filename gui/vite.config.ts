import { defineConfig } from 'vite';
import { resolve } from 'path';
import reactRefresh from '@vitejs/plugin-react-refresh';

// https://vitejs.dev/config/
export default defineConfig({
    plugins: process.env.NODE_ENV !== 'test' ? [reactRefresh()] : [],
    resolve: {
        alias: {
            '@': resolve(__dirname, 'src'),
        },
    },
    build: {
        target: 'es2020',
        minify: 'esbuild',
    },
    esbuild: {
        jsxInject: `import React from 'react'`,
    },
});
