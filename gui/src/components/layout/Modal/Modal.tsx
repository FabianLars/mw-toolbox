import { FocusableElement } from '@/helpers/types';
import React, { useEffect, useRef, useState } from 'react';
import ReactDOM from 'react-dom';
import FocusLock from 'react-focus-lock';
import classes from './Modal.module.css';

type Props = {
    header?: React.ReactNode;
    body?: React.ReactNode;
    footer?: React.ReactNode;
    isOpen?: boolean;
    onClose: () => void;
    initialFocusRef?: React.RefObject<FocusableElement>;
};

const Modal = ({ header, body, footer, isOpen, onClose, initialFocusRef }: Props) => {
    if (!isOpen) return null;

    const [show, setShow] = useState(false);

    const overlayRef = useRef(null);

    const closeModal = (e: any) => {
        if (overlayRef.current === e.target) {
            onClose();
        }
    };
    const keyDown = (e: KeyboardEvent) => {
        if (e.key === 'Escape' && isOpen) onClose();
    };

    useEffect(() => {
        document.addEventListener('keydown', keyDown);

        initialFocusRef?.current?.focus();

        setTimeout(() => {
            setShow(true);
        }, 1);

        return () => document.removeEventListener('keydown', keyDown);
    }, []);

    return ReactDOM.createPortal(
        <>
            <div
                className={`${classes.overlay} ${show ? classes.visible : ''}`}
                ref={overlayRef}
                onClick={closeModal}
            />
            <FocusLock returnFocus>
                <section
                    id="modal"
                    className={`${classes.modal} ${show ? classes.visible : ''}`}
                    role="dialog"
                    tabIndex={-1}
                    aria-modal="true"
                    aria-labelledby={`modal-header`}
                    aria-describedby={`modal-body`}
                >
                    <header id="modal-header" className={classes.header}>
                        {header}
                    </header>
                    <div id="modal-body" className={classes.body}>
                        {body}
                    </div>
                    <footer className={classes.footer}>{footer}</footer>
                </section>
            </FocusLock>
        </>,
        document.getElementById('modal-portal') as HTMLDivElement,
    );
};

export default Modal;
