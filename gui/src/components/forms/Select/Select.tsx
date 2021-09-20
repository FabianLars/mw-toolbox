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

const fixPosition = async () => {
    if (window.OS !== 'windows') return;
    const w = getCurrent();
    const size = await w.innerSize();
    const tempSize = new PhysicalSize(size.width + 1, size.height);
    const currSize = new PhysicalSize(size.width, size.height);
    await w.setSize(tempSize);
    await w.setSize(currSize);
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
            className={`${classes.select} ${className} ${window.OS ?? ''}`}
            id={id}
            name={name}
            disabled={isDisabled}
            onChange={onChange}
            value={value}
            // TODO: remove this once MS gets their shit together
            onMouseOver={fixPosition}
        >
            {placeholder && <option value="">{placeholder}</option>}
            {children}
        </select>
    );
};

export default Select;
