import cls from './Checkbox.module.css';

type Props = {
    className?: string;
    id: string;
    name?: string;
    isChecked?: boolean;
    isDisabled?: boolean;
    onBlur?: React.FocusEventHandler<HTMLInputElement>;
    onChange?: React.ChangeEventHandler<HTMLInputElement>;
    children: React.ReactNode;
};

const Checkbox = ({
    className = '',
    id,
    name,
    isChecked,
    isDisabled,
    onBlur,
    onChange,
    children,
}: Props): JSX.Element => {
    return (
        <div className={`${cls.wrapper} ${isDisabled ? cls.disabled : ''} ${className}`}>
            <input
                id={id}
                name={name}
                type="checkbox"
                checked={isChecked}
                onBlur={onBlur}
                onChange={onChange}
                aria-disabled={isDisabled}
                disabled={isDisabled}
            />
            <label className={cls.label} htmlFor={id}>
                {children}
            </label>
        </div>
    );
};

export default Checkbox;
