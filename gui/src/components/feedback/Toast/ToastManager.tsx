import React from 'react';
import ReactDOM from 'react-dom';
import Toast from './Toast';

export class ToastManager {
    private containerRef: HTMLDivElement;
    private currentToast: React.ReactNode = null;

    constructor() {
        document.getElementById('toast-container')?.remove();
        const body = document.getElementsByTagName('body')[0] as HTMLBodyElement;
        const toastContainer = document.createElement('div') as HTMLDivElement;
        toastContainer.id = 'toast-container';
        body.insertAdjacentElement('beforeend', toastContainer);
        this.containerRef = toastContainer;
    }

    public show(message: React.ReactNode) {
        if (this.currentToast) {
            this.destroy();
        }

        this.currentToast = message;
        this.render();
    }

    public destroy() {
        this.currentToast = null;
        ReactDOM.unmountComponentAtNode(this.containerRef);
    }

    private render() {
        ReactDOM.render(
            this.currentToast ? (
                <Toast destroy={() => this.destroy()}>{this.currentToast}</Toast>
            ) : (
                []
            ),
            this.containerRef,
        );
    }
}

export const toast = new ToastManager();
