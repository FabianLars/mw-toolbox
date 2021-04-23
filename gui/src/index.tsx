import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';
import { ChakraProvider, extendTheme, ThemeConfig } from '@chakra-ui/react';

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

// Hot Module Replacement (HMR) - Remove this snippet to remove HMR.
// Learn more: https://snowpack.dev/concepts/hot-module-replacement
if (import.meta.hot) {
    import.meta.hot.accept();
}
