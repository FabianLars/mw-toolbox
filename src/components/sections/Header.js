import React from 'react';
import { Link as ReactLink } from 'react-router-dom';
import { Flex, Link } from '@chakra-ui/react';

const MenuItems = (props) => {
    const { children, isLast, to = '/', ...rest } = props;
    return (
        <Link
            as={ReactLink}
            to={to}
            mb={{ base: isLast ? 0 : 8, sm: 0 }}
            mr={{ base: 0, sm: isLast ? 0 : 8 }}
            display="block"
            {...rest}
        >
            {children}
        </Link>
    );
};

const Header = () => {
    return (
        <Flex
            as="nav"
            align="center"
            justify="left"
            wrap="wrap"
            w="100%"
            mb={8}
            p={8}
            borderBottom="1px solid #deb992;"
        >
            <MenuItems to="/">Account</MenuItems>
            <MenuItems to="/Delete">Delete</MenuItems>
            {/* <MenuItems to="/Edit">Edit</MenuItems> */}
            <MenuItems to="/List">List</MenuItems>
            <MenuItems to="/Move">Move</MenuItems>
            <MenuItems to="/Other" isLast>Other</MenuItems>
        </Flex>
    );
};

export default Header;
