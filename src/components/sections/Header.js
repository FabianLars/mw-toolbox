import React from 'react';
import { Link as ReactLink, useLocation } from 'react-router-dom';
import { Box, Flex, Link, Spacer, Spinner } from '@chakra-ui/react';

const MenuItem = (props) => {
    const { children, isDisabled, isLast, to = '/', ...rest } = props;
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
            color={isDisabled ? 'red.700' : ''}
            style={{ pointerEvents: isDisabled ? 'none' : '' }}
            {...rest}
            _hover={{ bg: 'gray.700' }}
        >
            {children}
        </Link>
    );
};

const Header = ({ isDisabled }) => {
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
            <MenuItem isDisabled={isDisabled} to="/">
                Account
            </MenuItem>
            <MenuItem isDisabled={isDisabled} to="/Delete">
                Delete
            </MenuItem>
            {/* <MenuItems to="/Edit">Edit</MenuItems> */}
            <MenuItem isDisabled={isDisabled} to="/List">
                List
            </MenuItem>
            <MenuItem isDisabled={isDisabled} to="/Move">
                Move
            </MenuItem>
            <MenuItem isDisabled={isDisabled} to="/Other" isLast>
                Other
            </MenuItem>
            <Spacer />
            <Box pr={4} pt={2} display={isDisabled ? 'show' : 'none'}>
                <Spinner color="red.700" />
            </Box>
        </Flex>
    );
};

export default Header;
