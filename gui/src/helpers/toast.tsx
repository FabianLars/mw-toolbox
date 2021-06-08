import { UseToastOptions } from '@chakra-ui/react';
import React from 'react';

const errorToast = (error: { code: string; description: string }) => {
    return {
        title: `Something went wrong! ${error.code}-Error`,
        description: <span style={{ wordBreak: 'break-word' }}>{error.description}</span>,
        status: 'error',
        duration: 5000,
        isClosable: true,
    } as UseToastOptions;
};

const successToast = (title?: string, description?: string) => {
    return {
        title,
        description,
        status: 'success',
        duration: 1000,
        isClosable: true,
    } as UseToastOptions;
};

export { errorToast, successToast };
