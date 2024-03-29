import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { useState } from 'react';
import { describe, expect, test } from 'vitest';
import Select from './Select';

describe('<Select>', () => {
    test('uncontrolled', async () => {
        const user = userEvent.setup();
        render(
            <Select label="uncontrolled">
                <option value="t1">Profile 1</option>
                <option value="t2">Profile 2</option>
            </Select>,
        );
        expect(screen.getByRole<HTMLOptionElement>('option', { name: /profile 1/i }).selected).toBe(
            true,
        );
        expect(screen.getByRole<HTMLOptionElement>('option', { name: /profile 2/i }).selected).toBe(
            false,
        );
        await user.selectOptions(screen.getByLabelText('uncontrolled'), 't2');
        expect(screen.getByRole<HTMLOptionElement>('option', { name: /profile 1/i }).selected).toBe(
            false,
        );
        expect(screen.getByRole<HTMLOptionElement>('option', { name: /profile 2/i }).selected).toBe(
            true,
        );
    });
    test('controlled', async () => {
        const user = userEvent.setup();
        const Wrapper = () => {
            const [value, setValue] = useState('t1');
            return (
                <Select
                    label="controlled"
                    value={value}
                    onChange={(e) => {
                        setValue(e.target.value);
                    }}
                >
                    <option value="t1">Profile 1</option>
                    <option value="t2">Profile 2</option>
                </Select>
            );
        };
        render(<Wrapper />);
        expect(screen.getByRole<HTMLOptionElement>('option', { name: /profile 1/i }).selected).toBe(
            true,
        );
        expect(screen.getByRole<HTMLOptionElement>('option', { name: /profile 2/i }).selected).toBe(
            false,
        );
        await user.selectOptions(screen.getByLabelText('controlled'), 't2');
        expect(screen.getByRole<HTMLOptionElement>('option', { name: /profile 1/i }).selected).toBe(
            false,
        );
        expect(screen.getByRole<HTMLOptionElement>('option', { name: /profile 2/i }).selected).toBe(
            true,
        );
    });
});
