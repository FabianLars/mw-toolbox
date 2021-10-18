import { useEffect, useState } from 'react';
import cls from './Toast.module.css';

type Props = {
    children: React.ReactNode;
    destroy: () => void;
};

const Toast = ({ children, destroy }: Props): JSX.Element => {
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
        setTimeout(() => {
            setShow(true);
        }, 100);
    }, []);

    return (
        <div
            onClick={() => destroy()}
            role="alert"
            className={`${cls.toast} ${show ? cls.visible : ''}`}
        >
            {children}
        </div>
    );
};

export default Toast;
