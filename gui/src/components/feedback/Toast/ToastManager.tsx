import { createRoot, Root } from 'react-dom/client';
import Toast from './Toast';

export class ToastManager {
    private currentToast: React.ReactNode = null;
    private root: Root;

    constructor() {
        const toastContainer = document.getElementById('toast-portal') as HTMLDivElement;
        this.root = createRoot(toastContainer);
    }

    public show(message: React.ReactNode): void {
        if (this.currentToast) {
            this.destroy();
        }

        this.currentToast = message;
        this.render();
    }

    public destroy(): void {
        this.currentToast = null;
        this.root.unmount();
    }

    private render() {
        this.root.render(
            this.currentToast ? (
                <Toast destroy={() => this.destroy()}>{this.currentToast}</Toast>
            ) : (
                []
            ),
        );
    }
}

export const toast = new ToastManager();
