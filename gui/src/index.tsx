import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import App from './App';
import './index.css';

declare global {
    interface Window {
        OS: string;
        __TAURI__: Record<string, unknown>;
    }
}

const container = document.getElementById('root') as HTMLDivElement;
const root = createRoot(container);
root.render(
    <StrictMode>
        <App />
    </StrictMode>,
);
