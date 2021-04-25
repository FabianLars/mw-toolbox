import React, { render } from 'preact';
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

render(
    <ChakraProvider theme={customTheme}>
        <App />
    </ChakraProvider>,
    document.getElementById('root')!,
);
