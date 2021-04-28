import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';
import { ChakraProvider, extendTheme, ThemeConfig } from '@chakra-ui/react';

declare global {
    interface Window {
        __TAURI__: {};
    }
}

const config: ThemeConfig = {
    useSystemColorMode: false,
    initialColorMode: 'dark',
};

const styles = {
    global: {
        button: {
            minWidth: 'unset !important',
        },
    },
};

const customTheme = extendTheme({ config, styles });

ReactDOM.render(
    <React.StrictMode>
        <ChakraProvider theme={customTheme}>
            <App />
        </ChakraProvider>
    </React.StrictMode>,
    document.getElementById('root'),
);
