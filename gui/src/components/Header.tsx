import React from 'react';
import { Link as ReactLink, useLocation } from 'react-router-dom';
import {
    Badge,
    Divider,
    Flex,
    Button,
    Link,
    Menu,
    MenuButton,
    MenuItem,
    MenuList,
    Spacer,
    Spinner,
} from '@chakra-ui/react';
import { readTextFile } from '@tauri-apps/api/fs';

type MenuProps = {
    children: string;
    isDisabled: boolean;
    to: string;
};

type HeaderProps = {
    isDisabled: boolean;
    isOnline: boolean;
};

const HeaderItem = ({ children, isDisabled, to = '/' }: MenuProps): JSX.Element => {
    const location = useLocation().pathname;
    return (
        <Link
            as={ReactLink}
            to={to}
            borderTop="1px solid transparent"
            borderTopColor={location === to ? 'gray.500' : 'transparent'}
            px={4}
            py={3}
            borderRadius={5}
            color={isDisabled ? 'red.700' : undefined}
            pointerEvents={isDisabled ? 'none' : undefined}
            _hover={{ bg: 'gray.700' }}
            onClick={() => window.getSelection()?.removeAllRanges()}
        >
            {children}
        </Link>
    );
};

const Header = ({ isDisabled, isOnline }: HeaderProps): JSX.Element => {
    return (
        <>
            <Flex
                as="nav"
                align="center"
                justify="left"
                w="100%"
                p={2}
                borderBottom="1px solid #deb992"
            >
                <Flex display={['none', null, 'flex']} h={'50px'} pr={2}>
                    <HeaderItem isDisabled={isDisabled} to="/">
                        Account
                    </HeaderItem>
                    <Divider orientation="vertical" mx={1} />
                    <HeaderItem isDisabled={isDisabled} to="/Delete">
                        Delete
                    </HeaderItem>
                    <Divider orientation="vertical" mx={1} />
                    <HeaderItem isDisabled={isDisabled} to="/Download">
                        Download
                    </HeaderItem>
                    <Divider orientation="vertical" mx={1} />
                    <HeaderItem isDisabled={isDisabled} to="/Edit">
                        Edit
                    </HeaderItem>
                    <Divider orientation="vertical" mx={1} />
                    <HeaderItem isDisabled={isDisabled} to="/List">
                        List
                    </HeaderItem>
                    <Divider orientation="vertical" mx={1} />
                    <HeaderItem isDisabled={isDisabled} to="/Move">
                        Move
                    </HeaderItem>
                    <Divider orientation="vertical" mx={1} />
                    <HeaderItem isDisabled={isDisabled} to="/Purge">
                        Purge
                    </HeaderItem>
                    <Divider orientation="vertical" mx={1} />
                    <HeaderItem isDisabled={isDisabled} to="/Upload">
                        Upload
                    </HeaderItem>
                </Flex>
                <Menu isLazy>
                    <MenuButton as={Button} h="50px" display={[null, null, 'none']}>
                        Show Navigation Menu
                    </MenuButton>
                    <MenuList>
                        <MenuItem as={ReactLink} to="/">
                            Account
                        </MenuItem>
                        <MenuItem as={ReactLink} to="/Delete">
                            Delete
                        </MenuItem>
                        <MenuItem as={ReactLink} to="/Download">
                            Download
                        </MenuItem>
                        <MenuItem as={ReactLink} to="/Edit">
                            Edit
                        </MenuItem>
                        <MenuItem as={ReactLink} to="/List">
                            List
                        </MenuItem>
                        <MenuItem as={ReactLink} to="/Move">
                            Move
                        </MenuItem>
                        <MenuItem as={ReactLink} to="/Purge">
                            Purge
                        </MenuItem>
                        <MenuItem as={ReactLink} to="/Upload">
                            Upload
                        </MenuItem>
                    </MenuList>
                </Menu>
                <Spacer />
                {isDisabled ? (
                    <Spinner mr={2} color="red.700" />
                ) : (
                    <Badge mr={2} colorScheme={isOnline ? 'green' : 'red'}>
                        {isOnline ? 'Online' : 'Offline'}
                    </Badge>
                )}
            </Flex>
        </>
    );
};

export default Header;
