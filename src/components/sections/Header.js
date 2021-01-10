import React from 'react';
import { Link as ReactLink, useLocation } from 'react-router-dom';
import { Flex, Link } from '@chakra-ui/react';

const MenuItems = (props) => {
    const { children, isLast, to = '/', ...rest } = props;
    const location = useLocation().pathname;
    return (
        <Link
            as={ReactLink}
            to={to}
            mr={isLast ? 0 : 4}
            borderTop="1px solid transparent"
            borderTopColor={location === to ? 'gray.500' : 'transparent'}
            display="block"
            p={6}
            borderRadius={5}
            {...rest}
            _hover={{ bg: 'gray.700' }}
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
            p={2}
            mb={8}
            borderBottom="1px solid #deb992;"
        >
            <MenuItems to="/">Account</MenuItems>
            <MenuItems to="/Delete">Delete</MenuItems>
            {/* <MenuItems to="/Edit">Edit</MenuItems> */}
            <MenuItems to="/List">List</MenuItems>
            <MenuItems to="/Move">Move</MenuItems>
            <MenuItems to="/Other" isLast>
                Other
            </MenuItems>
        </Flex>
    );
};

export default Header;
