import React from 'react';
import { Flex } from '@chakra-ui/react';
import { Header, Purge } from '../components';

// Page for misc stuff
const Other = ({ isOnline }) => {
    return (
        <Flex direction="column" align="center" m="0 1rem" h="100vh">
            <Header isOnline={isOnline} />
            <Purge isOnline={isOnline} />
        </Flex>
    );
};

export default Other;
