import { routes } from '@/helpers/consts';
import { useRef, useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import cls from './Menu.module.css';

const Menu = () => {
    const [isOpen, setIsOpen] = useState(false);
    const firstRun = useRef(true);
    const clickedOpen = useRef(false);
    const currentIndex = useRef<number | null>(null);
    const buttonRef = useRef<HTMLButtonElement>(null);
    const itemRefs = useRef<(HTMLAnchorElement | null)[]>([]);

    const moveFocus = (index: number) => {
        currentIndex.current = index;
        itemRefs.current[index]?.focus();
    };

    // focus first element everytime the menu opens
    useEffect(() => {
        if (isOpen) {
            // skip if the hook fires for the first time
            if (firstRun.current) {
                firstRun.current = false;
                return;
            }

            // If the menu is currently open, set focus on the first item in the menu
            if (isOpen && !clickedOpen.current) {
                moveFocus(0);
            } else if (!isOpen) {
                clickedOpen.current = false;
            }
        }
    }, [isOpen]);

    // close menu on click
    useEffect(() => {
        if (!isOpen) return;

        const handleClicks = () => {
            setTimeout(() => setIsOpen(false), 10);
        };

        setTimeout(() => {
            document.addEventListener('click', handleClicks);
        }, 1);

        return () => document.removeEventListener('click', handleClicks);
    }, [isOpen]);

    const handleButtonClick = () => {
        clickedOpen.current = !isOpen;
        setIsOpen(!isOpen);
    };

    const handleButtonKeyDown = (event: React.KeyboardEvent) => {
        const { key } = event;

        if (!['Enter', ' ', 'Tab', 'ArrowDown'].includes(key)) {
            return;
        }

        if ((key === 'Tab' || key === 'ArrowDown') && clickedOpen.current && isOpen) {
            event.preventDefault();
            moveFocus(0);
        } else if (key !== 'Tab') {
            event.preventDefault();
            setIsOpen(true);
            moveFocus(0);
        }
    };

    const handleItemKeyDown = (event: React.KeyboardEvent<HTMLAnchorElement>) => {
        const { key } = event;
        // Handle keyboard controls
        if (['Tab', 'Shift', 'Enter', 'Escape', 'ArrowUp', 'ArrowDown', ' '].includes(key)) {
            // Create mutable value that initializes as the currentIndex value
            let newIndex = currentIndex.current;

            // Controls whether the menu is open or closed, if the button should regain focus on close, and if a handler function should be called
            // Close menu on Esc key. Set focus on button
            if (key === 'Escape') {
                setIsOpen(false);
                buttonRef.current?.focus();
                return;
                // Close menu on Tab key. move the focus to the next focusable dom element
            } else if (key === 'Tab') {
                setIsOpen(false);
                return;
                // activate item on Spacebar (and Enter key (standard behavior on links))
            } else if (key === ' ') {
                event.currentTarget.click();
                setIsOpen(false);
                return;
            }

            // Controls the current index to focus
            if (newIndex !== null) {
                if (key === 'ArrowUp') {
                    newIndex -= 1;
                } else if (key === 'ArrowDown') {
                    newIndex += 1;
                }

                if (newIndex > 7) {
                    newIndex = 0;
                } else if (newIndex < 0) {
                    newIndex = 7;
                }

                moveFocus(newIndex);
            }
        }
    };

    return (
        <>
            <button
                aria-haspopup="menu"
                aria-expanded={isOpen}
                aria-controls="menu-list"
                className={cls.button}
                onClick={handleButtonClick}
                onKeyDown={handleButtonKeyDown}
                ref={buttonRef}
                role="button"
                tabIndex={0}
            >
                Show Navigation Menu
            </button>
            <div
                id="menu-list"
                role="menu"
                aria-orientation="vertical"
                tabIndex={-1}
                className={`${cls.menu} ${isOpen ? cls.visible : ''}`}
            >
                {routes.map((v, i) => (
                    <Link
                        to={v}
                        key={'menu' + i}
                        ref={(el) => {
                            itemRefs.current[i] = el;
                        }}
                        onKeyDown={handleItemKeyDown}
                        tabIndex={-1}
                        role="menuitem"
                        onAuxClick={(e) => e.preventDefault()}
                    >
                        {v.substring(1) || 'Account'}
                    </Link>
                ))}
            </div>
        </>
    );
};

export default Menu;
