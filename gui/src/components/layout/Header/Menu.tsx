import React, { useRef, useState, useEffect, useMemo } from 'react';
import { Link } from 'react-router-dom';
import classes from './Menu.module.css';

const Menu = () => {
    const [isOpen, setIsOpen] = useState(false);
    const firstRun = useRef(true);
    const clickedOpen = useRef(false);
    const currentIndex = useRef<number | null>(null);
    const buttonRef = useRef<HTMLButtonElement>(null);
    const itemRefs = Array.from({ length: 8 }, () => useRef<HTMLAnchorElement>(null));

    const moveFocus = (index: number) => {
        currentIndex.current = index;
        itemRefs[index].current?.focus();
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
    useEffect!(() => {
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
                className={classes.button}
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
                className={`${classes.menu} ${isOpen ? classes.visible : ''}`}
            >
                <Link
                    to="/"
                    ref={itemRefs[0]}
                    onKeyDown={handleItemKeyDown}
                    tabIndex={-1}
                    role="menuitem"
                >
                    Account
                </Link>
                <Link
                    to="/Delete"
                    ref={itemRefs[1]}
                    onKeyDown={handleItemKeyDown}
                    tabIndex={-1}
                    role="menuitem"
                >
                    Delete
                </Link>
                <Link
                    to="/Download"
                    ref={itemRefs[2]}
                    onKeyDown={handleItemKeyDown}
                    tabIndex={-1}
                    role="menuitem"
                >
                    Download
                </Link>
                <Link
                    to="/Edit"
                    ref={itemRefs[3]}
                    onKeyDown={handleItemKeyDown}
                    tabIndex={-1}
                    role="menuitem"
                >
                    Edit
                </Link>
                <Link
                    to="/List"
                    ref={itemRefs[4]}
                    onKeyDown={handleItemKeyDown}
                    tabIndex={-1}
                    role="menuitem"
                >
                    List
                </Link>
                <Link
                    to="/Move"
                    ref={itemRefs[5]}
                    onKeyDown={handleItemKeyDown}
                    tabIndex={-1}
                    role="menuitem"
                >
                    Move
                </Link>
                <Link
                    to="/Purge"
                    ref={itemRefs[6]}
                    onKeyDown={handleItemKeyDown}
                    tabIndex={-1}
                    role="menuitem"
                >
                    Purge
                </Link>
                <Link
                    to="/Upload"
                    ref={itemRefs[7]}
                    onKeyDown={handleItemKeyDown}
                    tabIndex={-1}
                    role="menuitem"
                >
                    Upload
                </Link>
            </div>
        </>
    );
};

export default Menu;
