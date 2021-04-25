import { render } from '@testing-library/preact';
import { expect } from 'chai';
import App from './App';

describe('<App>', () => {
    it('renders "Not logged in!"', () => {
        const { getByText } = render(<App />);
        const linkElement = getByText(/Not logged in/i);
        expect(document.body.contains(linkElement));
    });
});
