import cls from './Label.module.css';

type Props = {
    className?: string;
    children: React.ReactNode;
    htmlFor: string;
    isDisabled?: boolean;
    isRequired?: boolean;
};

const Label = ({
    children,
    className = '',
    htmlFor,
    isDisabled = false,
    isRequired,
}: Props): JSX.Element => {
    return (
        <label
            htmlFor={htmlFor}
            className={`${cls.label} ${isDisabled ? cls.disabled : ''} ${className}`}
        >
            {children}
            {isRequired && (
                <span className={cls.indicator} aria-hidden>
                    *
                </span>
            )}
        </label>
    );
};

export default Label;
