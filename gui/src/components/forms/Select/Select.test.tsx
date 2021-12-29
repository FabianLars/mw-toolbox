import '@testing-library/jest-dom';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { useState } from 'react';
import { describe, expect, it } from 'vitest';
import Select from './Select';

describe('<Select>', () => {
    it('uncontrolled', async () => {
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
        userEvent.selectOptions(screen.getByLabelText('uncontrolled'), 't2');
        expect(screen.getByRole<HTMLOptionElement>('option', { name: /profile 1/i }).selected).toBe(
            false,
        );
        expect(screen.getByRole<HTMLOptionElement>('option', { name: /profile 2/i }).selected).toBe(
            true,
        );
    });
    it('controlled', async () => {
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
        userEvent.selectOptions(screen.getByLabelText('controlled'), 't2');
        expect(screen.getByRole<HTMLOptionElement>('option', { name: /profile 1/i }).selected).toBe(
            false,
        );
        expect(screen.getByRole<HTMLOptionElement>('option', { name: /profile 2/i }).selected).toBe(
            true,
        );
    });
});
