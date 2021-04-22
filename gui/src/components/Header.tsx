import React from 'react';
import { Link as ReactLink, useLocation } from 'react-router-dom';
import { Badge, Flex, Link, Spacer, Spinner } from '@chakra-ui/react';

type MenuProps = {
    children: string;
    isDisabled: boolean;
    isLast?: boolean;
    to: string;
};

type HeaderProps = {
    isDisabled: boolean;
    isOnline: boolean;
};

const MenuItem = ({ children, isDisabled, isLast, to = '/' }: MenuProps): JSX.Element => {
    const location = useLocation().pathname;
    return (
        <Link
            as={ReactLink}
            to={to}
            mr={isLast ? 0 : 4}
            borderTop="1px solid transparent"
            borderTopColor={location === to ? 'gray.500' : 'transparent'}
            p="1rem 1.5rem"
            borderRadius={5}
            color={isDisabled ? 'red.700' : ''}
            pointerEvents={isDisabled ? 'none' : undefined}
            _hover={{ bg: 'gray.700' }}
        >
            {children}
        </Link>
    );
};

const Header = ({ isDisabled, isOnline }: HeaderProps): JSX.Element => {
    return (
        <Flex as="nav" align="center" justify="left" w="100%" p={2} mb={4} borderBottom="1px solid #deb992;">
            <MenuItem isDisabled={isDisabled} to="/">
                Account
            </MenuItem>
            <MenuItem isDisabled={isDisabled} to="/Delete">
                Delete
            </MenuItem>
            <MenuItem isDisabled={isDisabled} to="/Download">
                Download
            </MenuItem>
            <MenuItem isDisabled={isDisabled} to="/Edit">
                Edit
            </MenuItem>
            <MenuItem isDisabled={isDisabled} to="/List">
                List
            </MenuItem>
            <MenuItem isDisabled={isDisabled} to="/Move">
                Move
            </MenuItem>
            <MenuItem isDisabled={isDisabled} to="/Purge">
                Purge
            </MenuItem>
            <MenuItem isDisabled={isDisabled} to="/Upload" isLast>
                Upload
            </MenuItem>
            <Spacer />
            <Flex justify="center" h="100%" align="center">
                <Spinner display={isDisabled ? 'show' : 'none'} color="red.700" />
                <Badge m={2} colorScheme={isOnline ? 'green' : 'red'}>
                    {isOnline ? 'Online' : 'Offline'}
                </Badge>
            </Flex>
        </Flex>
    );
};

export default Header;
