import { FocusableElement } from '@/helpers/types';
import React, { useCallback, useEffect, useRef } from 'react';
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

        return () => document.removeEventListener('keydown', keyDown);
    }, []);

    return ReactDOM.createPortal(
        <>
            <div className={classes.overlay} ref={overlayRef} onClick={closeModal} />
            <FocusLock returnFocus>
                <section
                    id="modal"
                    className={classes.modal}
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
