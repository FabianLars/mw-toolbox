import { render } from '@testing-library/react';
import { expect } from 'chai';
import App from './App';

// Placeholder test

describe('<App>', () => {
    it('renders "Profile 1"', () => {
        const { getByText } = render(<App />);
        const el = getByText(/Profile 1/i);
        expect(document.body.contains(el));
    });
});
