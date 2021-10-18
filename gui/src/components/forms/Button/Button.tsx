import { Spinner } from '@/components/feedback';
import { forwardRef } from 'react';

import cls from './Button.module.css';

type Props = {
    className?: string;
    isLoading?: boolean;
    isDisabled?: boolean;
    onClick?: () => void;
    loadingText?: string;
    title?: string;
    ['aria-label']?: string;
    children: React.ReactNode;
    colorScheme?: 'blue' | 'red' | 'default';
};

const Button = forwardRef((props: Props, ref) => {
    const {
        className = '',
        colorScheme = 'default',
        isLoading,
        isDisabled,
        onClick,
        loadingText,
        title,
        children,
    } = props;

    return (
        <button
            ref={ref as React.ForwardedRef<HTMLButtonElement>}
            type="button"
            disabled={isDisabled || isLoading}
            className={`${cls.button} ${className} ${cls[colorScheme]}`}
            title={title}
            aria-label={props['aria-label']}
            onClick={onClick}
        >
            {isLoading ? (
                <>
                    <Spinner className={cls.spinner} />
                    {loadingText}
                </>
            ) : (
                children
            )}
        </button>
    );
});

Button.displayName = 'MwtButton';

export default Button;
