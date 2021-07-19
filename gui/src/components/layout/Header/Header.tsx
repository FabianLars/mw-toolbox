import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import { Badge, Button, Menu, MenuButton, MenuItem, MenuList, Spinner } from '@chakra-ui/react';
import classes from './Header.module.css';
import { Divider } from '@/components';

type MenuProps = {
    children: string;
    isDisabled: boolean;
    to: string;
};

type HeaderProps = {
    isDisabled: boolean;
    isOnline: boolean;
};

const HeaderItem = ({ children, isDisabled = false, to = '/' }: MenuProps): JSX.Element => {
    const location = useLocation().pathname;
    return (
        <Link
            to={to}
            className={`${classes.link} ${isDisabled ? classes.disabled : ''} ${
                location == to ? classes.current : ''
            }`}
        >
            {children}
        </Link>
    );
};

const Header = ({ isDisabled, isOnline }: HeaderProps): JSX.Element => {
    return (
        <>
            <nav className={classes.nav}>
                <div className={classes.wide}>
                    <HeaderItem isDisabled={isDisabled} to="/">
                        Account
                    </HeaderItem>
                    <Divider orientation="vertical" />
                    <HeaderItem isDisabled={isDisabled} to="/Delete">
                        Delete
                    </HeaderItem>
                    <Divider orientation="vertical" />
                    <HeaderItem isDisabled={isDisabled} to="/Download">
                        Download
                    </HeaderItem>
                    <Divider orientation="vertical" />
                    <HeaderItem isDisabled={isDisabled} to="/Edit">
                        Edit
                    </HeaderItem>
                    <Divider orientation="vertical" />
                    <HeaderItem isDisabled={isDisabled} to="/List">
                        List
                    </HeaderItem>
                    <Divider orientation="vertical" />
                    <HeaderItem isDisabled={isDisabled} to="/Move">
                        Move
                    </HeaderItem>
                    <Divider orientation="vertical" />
                    <HeaderItem isDisabled={isDisabled} to="/Purge">
                        Purge
                    </HeaderItem>
                    <Divider orientation="vertical" />
                    <HeaderItem isDisabled={isDisabled} to="/Upload">
                        Upload
                    </HeaderItem>
                </div>
                <Menu isLazy>
                    <MenuButton as={Button} h="50px" display={[null, null, 'none']}>
                        Show Navigation Menu
                    </MenuButton>
                    <MenuList>
                        <MenuItem as={Link} to="/">
                            Account
                        </MenuItem>
                        <MenuItem as={Link} to="/Delete">
                            Delete
                        </MenuItem>
                        <MenuItem as={Link} to="/Download">
                            Download
                        </MenuItem>
                        <MenuItem as={Link} to="/Edit">
                            Edit
                        </MenuItem>
                        <MenuItem as={Link} to="/List">
                            List
                        </MenuItem>
                        <MenuItem as={Link} to="/Move">
                            Move
                        </MenuItem>
                        <MenuItem as={Link} to="/Purge">
                            Purge
                        </MenuItem>
                        <MenuItem as={Link} to="/Upload">
                            Upload
                        </MenuItem>
                    </MenuList>
                </Menu>
                <div className={classes.spacer}></div>
                {isDisabled ? (
                    <Spinner mr={2} color="red.700" />
                ) : (
                    <Badge mr={2} colorScheme={isOnline ? 'green' : 'red'}>
                        {isOnline ? 'Online' : 'Offline'}
                    </Badge>
                )}
            </nav>
        </>
    );
};

export default Header;
