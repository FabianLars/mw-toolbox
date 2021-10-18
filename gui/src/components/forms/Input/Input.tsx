import cls from './Input.module.css';

type Props = {
    label?: string;
    placeholder?: string;
    className?: string;
    id?: string;
    name?: string;
    isDisabled?: boolean;
    isInvalid?: boolean;
    isPassword?: boolean;
    isRequired?: boolean;
    value: string | number;
    onBlur?: React.FocusEventHandler<HTMLInputElement>;
    onChange: React.ChangeEventHandler<HTMLInputElement>;
};

const Input = ({
    label,
    className = '',
    id,
    isDisabled,
    isInvalid,
    isPassword,
    isRequired,
    name,
    placeholder,
    value,
    onBlur,
    onChange,
}: Props): JSX.Element => {
    return (
        <input
            aria-label={label}
            className={`${cls.input} ${className} ${window.OS}`}
            id={id}
            type={isPassword ? 'password' : undefined}
            name={name}
            placeholder={placeholder}
            required={isRequired}
            disabled={isDisabled}
            aria-invalid={isInvalid}
            aria-required={isRequired}
            onBlur={onBlur}
            onChange={onChange}
            value={value}
        />
    );
};

export default Input;
