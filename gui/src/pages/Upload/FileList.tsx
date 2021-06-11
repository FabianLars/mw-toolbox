import React from 'react';

const FileList = ({
    children,
    placeholder,
}: {
    children: JSX.Element[];
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
                userSelect: children.length === 0 ? 'none' : 'text',
                overflowY: 'auto',
                whiteSpace: 'pre-line',
                opacity: children.length === 0 ? '0.5' : 'initial',
            }}
        >
            {children.length === 0 ? placeholder : children}
        </div>
    );
};

export default FileList;
