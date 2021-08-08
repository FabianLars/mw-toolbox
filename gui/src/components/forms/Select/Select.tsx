import { getCurrent, PhysicalSize } from '@tauri-apps/api/window';
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
            className={`${classes.select} ${className} ${window.OS ?? ''}`}
            id={id}
            name={name}
            disabled={isDisabled}
            onChange={onChange}
            value={value}
            // TODO: remove this once MS gets their shit together
            onMouseOver={
                window.OS == 'windows'
                    ? async () => {
                          console.log('test');

                          const window = getCurrent();
                          const size = await window.innerSize();
                          const tempSize = new PhysicalSize(size.width + 1, size.height);
                          const currSize = new PhysicalSize(size.width, size.height);
                          await window.setSize(tempSize);
                          await window.setSize(currSize);
                      }
                    : undefined
            }
        >
            {placeholder && <option value="">{placeholder}</option>}
            {children}
        </select>
    );
};

export default Select;
