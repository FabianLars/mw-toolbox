import React from 'react';
import classes from './Spinner.module.css';

type Props = { className?: string };

const Spinner = ({ className = '' }: Props): JSX.Element => {
    return (
        <div className={`${classes.spinner} ${className}`}>
            {/* <span className={classes.span}>Loading...</span> */}
        </div>
    );
};

export default Spinner;
