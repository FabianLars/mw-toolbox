import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import Select from './Select';

// Placeholder test

describe('<Select>', () => {
    it('controlled', async () => {
        await render(
            <Select>
                <option value="t1">Profile 1</option>
                <option value="t2">Profile 2</option>
            </Select>,
        );
        expect(screen.getByText(/Profile 1/i)).toBeTruthy();
    });
    it('uncontrolled', async () => {
        let value = 't1';
        await render(
            <Select
                value={value}
                onChange={(e) => {
                    value = e.target.value;
                }}
            >
                <option value="t1">Profile 1</option>
                <option value="t2">Profile 2</option>
            </Select>,
        );
        expect(screen.getByText(/Profile 1/i)).toBeTruthy();
    });
});
