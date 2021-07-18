import React from 'react';
import classes from './Select.module.css';

type Props = {
    label?: string;
    className?: string;
    id?: string;
    name?: string;
    isDisabled?: boolean;
    placeholder?: string;
    value?: string | number;
    onChange: React.ChangeEventHandler<HTMLSelectElement>;
    children: React.ReactNode;
};

const Select = ({
    label,
    className = '',
    id,
    name,
    isDisabled,
    placeholder,
    value,
    onChange,
    children,
}: Props) => {
    return (
        <select
            aria-label={label}
            className={`${classes.select} ${className}`}
            id={id}
            name={name}
            disabled={isDisabled}
            onChange={onChange}
            value={value}
        >
            {placeholder && <option value="">{placeholder}</option>}
            {children}
        </select>
    );
};

export default Select;
