import React, { useState } from 'react';
import { Link as ReactLink } from 'react-router-dom';
import { Box, Flex, Link, Button } from '@chakra-ui/react';
import { CloseIcon, HamburgerIcon as MenuIcon } from '@chakra-ui/icons';

type Props = {
    children: React.ReactNode,
    to: string,
    [x: string]: any,
};

const MenuItems = (props: Props) => {
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
            justify="space-between"
            wrap="wrap"
            w="100%"
            mb={8}
            p={8}
            bg={['primary.500', 'primary.500', 'transparent', 'transparent']}
            color={['white', 'white', 'primary.700', 'primary.700']}
            borderBottom="1px solid #deb992;"
        >
            <Box flexBasis={{ base: '100%', md: 'auto' }}>
                <Flex
                    align={['center', 'center', 'center', 'center']}
                    justify={['center', 'space-between', 'flex-end', 'flex-end']}
                    direction={['column', 'row', 'row', 'row']}
                    pt={[4, 4, 0, 0]}
                >
                    <MenuItems to="/">Account</MenuItems>
                    <MenuItems to="/Delete">Delete</MenuItems>
                    {/* <MenuItems to="/Edit">Edit</MenuItems> */}
                    <MenuItems to="/List">List</MenuItems>
                    <MenuItems to="/Move">Move</MenuItems>
                    <MenuItems to="/Other" isLast>Other</MenuItems>
                </Flex>
            </Box>
        </Flex>
    );
};

export default Header;
