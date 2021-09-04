import React from 'react';
import classes from './Badge.module.css';

type Props = {
    children: React.ReactNode;
    type?: 'success' | 'error';
};

const Badge = ({ children, type }: Props): JSX.Element => {
    return <span className={`${classes.badge} ${type ? classes[type] : ''}`}>{children}</span>;
};

export default Badge;
