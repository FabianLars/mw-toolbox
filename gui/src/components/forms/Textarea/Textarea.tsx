import React from 'react';
import classes from './Textarea.module.css';

type Props = {
    label?: string;
    className?: string;
    id?: string;
    name?: string;
    isDisabled?: boolean;
    placeholder?: string;
    readOnly?: boolean;
    value?: string | number;
    onBlur?: React.FocusEventHandler<HTMLTextAreaElement>;
    onChange?: React.ChangeEventHandler<HTMLTextAreaElement>;
};

const Textarea = ({
    label,
    className = '',
    id,
    name,
    isDisabled,
    placeholder,
    readOnly,
    value,
    onBlur,
    onChange,
}: Props) => {
    return (
        <textarea
            aria-label={label}
            placeholder={placeholder}
            className={`${classes.area} ${className}`}
            id={id}
            name={name}
            disabled={isDisabled}
            readOnly={readOnly}
            onBlur={onBlur}
            onChange={onChange}
            value={value}
        />
    );
};

export default Textarea;
