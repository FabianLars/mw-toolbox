import { Badge, toast } from '@/components';
import React from 'react';

const errorToast = (error: { code: string; description: string }) => {
    // TODO: log description to status bar or something
    toast.show(
        <>
            <Badge type="error">ERROR</Badge>
            {`Code: ${error.code}`}
        </>,
    );
};

const successToast = (message?: string, description?: string) => {
    // TODO: log description to status bar or something
    toast.show(
        <>
            <Badge type="success">SUCCES</Badge>
            {`${message}`}
        </>,
    );
};

export { errorToast, successToast };
