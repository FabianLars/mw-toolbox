// FIXME:
/* eslint-disable react-hooks/rules-of-hooks */

import type { FocusableElement } from '@/helpers/types';
import { JSX, useEffect, useRef, useState } from 'react';
import ReactDOM from 'react-dom';
import FocusLock from 'react-focus-lock';
import cls from './Modal.module.css';

type Props = {
    header?: React.ReactNode;
    body?: React.ReactNode;
    footer?: React.ReactNode;
    isOpen?: boolean;
    onClose: () => void;
    initialFocusRef?: React.RefObject<FocusableElement>;
};

const Modal = ({
    header,
    body,
    footer,
    isOpen,
    onClose,
    initialFocusRef,
}: Props): JSX.Element | null => {
    if (!isOpen) return null;

    const [show, setShow] = useState(false);

    const overlayRef = useRef(null);

    const closeModal = (e: React.MouseEvent<HTMLDivElement>) => {
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
        // FIXME:
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, []);

    return ReactDOM.createPortal(
        <>
            <div
                className={`${cls.overlay} ${show ? cls.visible : ''}`}
                ref={overlayRef}
                onClick={closeModal}
            />
            <FocusLock returnFocus>
                <section
                    id="modal"
                    className={`${cls.modal} ${show ? cls.visible : ''}`}
                    role="dialog"
                    tabIndex={-1}
                    aria-modal="true"
                    aria-labelledby={`modal-header`}
                    aria-describedby={`modal-body`}
                >
                    <header id="modal-header" className={cls.header}>
                        {header}
                    </header>
                    <div id="modal-body" className={cls.body}>
                        {body}
                    </div>
                    <footer className={cls.footer}>{footer}</footer>
                </section>
            </FocusLock>
        </>,
        document.getElementById('modal-portal') as HTMLDivElement,
    );
};

export default Modal;
