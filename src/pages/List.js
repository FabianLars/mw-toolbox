import React from 'react';

import { promisified } from 'tauri/api/tauri';
import { Flex } from '@chakra-ui/react';

import Header from '../components/sections/Header';

const getList = () => {
    promisified({
        cmd: 'list',
    })
        .then((res) => console.log(res))
        .catch((err) => console.error(err));
};

const List = () => {
    return (
        <Flex direction="column" align="center" maxW={{ xl: '1240px' }} m="0 auto" h="100vh">
            <Header />
        </Flex>
    );
};

export default List;
