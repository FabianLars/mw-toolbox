import React from 'react';
import classes from './Divider.module.css';

type Props = {
    orientation?: 'horizontal' | 'vertical';
};

const Divider = ({ orientation = 'horizontal' }) => {
    return (
        <hr
            aria-orientation={orientation as Props['orientation']}
            className={`${classes.hr} ${classes[orientation]}`}
        />
    );
};

export default Divider;
