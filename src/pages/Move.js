import { Flex } from '@chakra-ui/react';
import React from 'react';
import Header from '../components/sections/Header';

const Move = ({ isOnline }) => {
    return (
        <Flex direction="column" align="center" m="0 1rem" h="100vh">
            <Header isOnline={isOnline} />
        </Flex>
    );
};

export default Move;
