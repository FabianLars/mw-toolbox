import { Link, useLocation } from 'react-router-dom';
import cls from './Header.module.css';
import Menu from './Menu';
import { Badge, Divider, Spinner } from '@/components';
import { routes } from '@/helpers/consts';
import { Fragment } from 'react';

type MenuProps = {
    children: string;
    isDisabled: boolean;
    to: string;
};

type HeaderProps = {
    isDisabled: boolean;
    isOnline: boolean;
};

const HeaderItem = ({ children, isDisabled = false, to = '/' }: MenuProps) => {
    const location = useLocation().pathname;
    return (
        <Link
            to={to}
            className={`${cls.link} ${isDisabled ? cls.disabled : ''} ${
                location == to ? cls.current : ''
            }`}
            onAuxClick={(e) => e.preventDefault()}
        >
            {children}
        </Link>
    );
};

const Header = ({ isDisabled, isOnline }: HeaderProps) => {
    return (
        <>
            <nav className={cls.nav}>
                <div className={cls.wide}>
                    {routes.map((v, i) => (
                        <Fragment key={'head' + i}>
                            {i !== 0 ? <Divider orientation="vertical" /> : undefined}
                            <HeaderItem to={v} isDisabled={isDisabled}>
                                {v.substring(1) || 'Account'}
                            </HeaderItem>
                        </Fragment>
                    ))}
                </div>
                <Menu />
                <div className={cls.spacer}></div>
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
