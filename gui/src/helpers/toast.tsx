import { Badge, toast } from '@/components';
import React from 'react';

const errorToast = (error: { code: string; description: string }): void => {
    // TODO: log description to status bar or something
    console.log('error', error.code, error.description);
    toast.show(
        <>
            <Badge type="error">ERROR</Badge>
            {`Code: ${error.code}`}
        </>,
    );
};

const successToast = (message?: string, description?: string): void => {
    // TODO: log description to status bar or something
    console.log('success', message, description);
    toast.show(
        <>
            <Badge type="success">SUCCES</Badge>
            {`${message}`}
        </>,
    );
};

export { errorToast, successToast };
