import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import App from './App';

// Placeholder test

describe('<App>', () => {
    it('renders "Profile 1"', async () => {
        render(<App />);
        expect(screen.getByText(/Profile Name/i)).toBeDefined();
    });
});
