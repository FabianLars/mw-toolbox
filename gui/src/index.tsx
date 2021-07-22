import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';
import './index.css';

declare global {
    interface Window {
        __TAURI__: {};
    }
}

ReactDOM.render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
    document.getElementById('root'),
);
