import React from 'react';
import classes from './Label.module.css';

type Props = {
    className?: string;
    children: React.ReactNode;
    htmlFor: string;
    isDisabled?: boolean;
    isRequired?: boolean;
};

const Label = ({ children, className = '', htmlFor, isDisabled = false, isRequired }: Props) => {
    return (
        <label
            htmlFor={htmlFor}
            className={`${classes.label} ${isDisabled && classes.disabled} ${className}`}
        >
            {children}
            {isRequired && (
                <span className={classes.indicator} aria-hidden>
                    *
                </span>
            )}
        </label>
    );
};

export default Label;
