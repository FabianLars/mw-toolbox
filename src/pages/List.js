import React, { useEffect, useState } from 'react';

import { promisified } from 'tauri/api/tauri';
import { Button, Flex, Textarea } from '@chakra-ui/react';

import Header from '../components/sections/Header';

const List = () => {
    const [loading, setLoading] = useState(false);
    const [listOutput, setListOutput] = useState('');

    const init = () => {
        (async () => setListOutput(window.sessionStorage.getItem('list-cache') ?? ''))();
    };

    const getList = () => {
        setLoading(true);
        promisified({
            cmd: 'list',
            listtype: 'allimages',
        })
            .then((res) => {
                const output = res.list.join('\n') ?? '';
                setListOutput(output);
                window.sessionStorage.setItem('list-cache', output);
                setLoading(false);
            })
            .catch((err) => console.error(err));
    };

    useEffect(() => {
        init();
    }, []);

    return (
        <Flex direction="column" align="center" maxW={{ xl: '1240px' }} m="0 auto" h="100vh">
            <Header />
            <Flex mb={8} direction="row">
                <Button onClick={getList} isLoading={loading}>
                    Get List
                </Button>
            </Flex>
            <Textarea value={listOutput} isReadOnly placeholder="Output will be displayed here" h="100%" mb={4} />
        </Flex>
    );
};

export default List;
