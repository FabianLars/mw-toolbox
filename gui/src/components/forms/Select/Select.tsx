import { listen } from '@tauri-apps/api/event';
import { useEffect, useRef } from 'react';
import cls from './Select.module.css';

type Props = {
    label?: string;
    className?: string;
    id?: string;
    name?: string;
    isDisabled?: boolean;
    placeholder?: string;
    value?: string | number;
    onChange?: React.ChangeEventHandler<HTMLSelectElement>;
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
    const selRef = useRef<HTMLSelectElement>(null);

    useEffect(() => {
        if (window.OS !== 'windows') return;

        const u = listen('tauri://move', () => selRef.current?.blur());
        return () => {
            u.then((f) => f());
        };
    }, []);

    return (
        <select
            aria-label={label}
            className={`${cls.select} ${className} ${window.OS}`}
            id={id}
            name={name}
            disabled={isDisabled}
            onChange={onChange}
            value={value}
            ref={selRef}
        >
            {placeholder && <option value="">{placeholder}</option>}
            {children}
        </select>
    );
};

export default Select;
