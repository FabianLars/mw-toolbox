import { StrictMode } from 'react';
import ReactDOM from 'react-dom';
import App from './App';
import './index.css';

declare global {
    interface Window {
        OS: string;
        //__TAURI__: {};
    }
}

ReactDOM.render(
    <StrictMode>
        <App />
    </StrictMode>,
    document.getElementById('root'),
);
