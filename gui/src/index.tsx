import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';
import { ChakraProvider, extendTheme, ThemeConfig } from '@chakra-ui/react';
import './index.css';

declare global {
    interface Window {
        __TAURI__: {};
    }
}

const config: ThemeConfig = {
    useSystemColorMode: false,
    initialColorMode: 'dark',
};

const customTheme = extendTheme({ config });

ReactDOM.render(
    <React.StrictMode>
        <ChakraProvider theme={customTheme}>
            <App />
        </ChakraProvider>
    </React.StrictMode>,
    document.getElementById('root'),
);
