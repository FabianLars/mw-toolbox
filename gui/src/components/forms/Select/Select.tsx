import React from 'react';
import classes from './Select.module.css';
import { getCurrent, PhysicalSize } from '@tauri-apps/api/window';

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
}: Props): JSX.Element => {
    return (
        <select
            aria-label={label}
            className={`${classes.select} ${className} ${window.OS}`}
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
