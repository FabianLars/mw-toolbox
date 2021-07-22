import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import classes from './Header.module.css';
import Menu from './Menu';
import { Badge, Divider, Spinner } from '@/components';

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
                <Menu />
                <div className={classes.spacer}></div>
                {isDisabled ? (
                    <Spinner />
                ) : (
                    <Badge type={isOnline ? 'success' : 'error'}>
                        {isOnline ? 'Online' : 'Offline'}
                    </Badge>
                )}
            </nav>
        </>
    );
};

export default Header;
