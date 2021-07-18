import { defineConfig } from 'vite';
import { resolve } from 'path';
import reactRefresh from '@vitejs/plugin-react-refresh';

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [reactRefresh()],
    resolve: {
        alias: {
            '@': resolve(__dirname, 'src'),
        },
    },
    build: {
        target: 'es2018',
        minify: 'esbuild',
    },
});
