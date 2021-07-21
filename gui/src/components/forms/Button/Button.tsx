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
    children: React.ReactNode;
    ref?: React.RefObject<HTMLButtonElement>;
    colorScheme?: 'blue' | 'red' | 'default';
};

const Button = ({
    className = '',
    colorScheme = 'default',
    isLoading,
    isDisabled,
    onClick,
    loadingText,
    title,
    children,
    ref,
}: Props) => {
    return (
        <button
            ref={ref}
            type="button"
            disabled={isDisabled || isLoading}
            className={`${classes.button} ${className} ${classes[colorScheme]}`}
            title={title}
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
};

export default Button;
