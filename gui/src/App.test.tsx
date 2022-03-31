import { render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import App from './App';

// Placeholder test

vi.mock('react-dom/client', () => {
    return {
        createRoot: vi.fn(),
    };
});

describe('<App>', () => {
    afterEach(() => {
        vi.clearAllMocks();
    });

    it('renders "Profile 1"', async () => {
        render(<App />);
        expect(screen.getByText(/Profile Name/i)).toBeDefined();
    });
});
