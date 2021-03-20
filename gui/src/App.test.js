import { render, screen } from '@testing-library/react';
import App from './App';

test('renders "Not logged in!"', () => {
    render(<App />);
    const linkElement = screen.getByText(/Not logged in!/i);
    expect(linkElement).toBeInTheDocument();
});
