import { Spinner } from '@/components/feedback';
import React from 'react';
import classes from './Button.module.css';

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

const Button = React.forwardRef((props: Props, ref) => {
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
            className={`${classes.button} ${className} ${classes[colorScheme]}`}
            title={title}
            aria-label={props['aria-label']}
            onClick={onClick}
        >
            {isLoading ? (
                <>
                    <Spinner className={classes.spinner} />
                    {loadingText}
                </>
            ) : (
                children
            )}
        </button>
    );
});

export default Button;
