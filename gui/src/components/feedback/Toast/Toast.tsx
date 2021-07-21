import React, { useEffect, useState } from 'react';
import classes from './Toast.module.css';

type Props = {
    children: React.ReactNode;
    destroy: () => void;
};

const Toast = ({ children, destroy }: Props) => {
    const [show, setShow] = useState(false);

    useEffect(() => {
        const timer = setTimeout(() => {
            setShow(false);
            setTimeout(() => {
                destroy();
            }, 1000);
        }, 5000);

        return () => clearTimeout(timer);
    }, [destroy]);

    useEffect(() => {
        setShow(true);
    }, []);

    return (
        <div role="alert" className={`${classes.toast} ${show ? classes.visible : ''}`}>
            {children}
        </div>
    );
};

export default Toast;
