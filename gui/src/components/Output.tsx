import React from 'react';

const Output = ({
    children,
    placeholder,
}: {
    children?: string | JSX.Element | JSX.Element[];
    placeholder?: string;
}): JSX.Element => {
    return (
        <div
            style={{
                flex: '1',
                width: '100%',
                padding: '8px 16px',
                border: '1px solid',
                borderColor: 'rgba(255, 255, 255, 0.16)',
                borderRadius: '6px',
                userSelect: 'text',
                overflowY: 'auto',
                whiteSpace: 'pre-line',
                opacity: !children ? '0.5' : 'initial',
            }}
        >
            {children || placeholder || ''}
        </div>
    );
};

export default Output;
