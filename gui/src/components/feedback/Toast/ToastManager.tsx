import React from 'react';
import ReactDOM from 'react-dom';
import Toast from './Toast';

export class ToastManager {
    private containerRef: HTMLDivElement;
    private currentToast: React.ReactNode = null;

    constructor() {
        const toastContainer = document.getElementById('toast-portal') as HTMLDivElement;
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
