import React, { useEffect, useState } from 'react';

import { promisified } from 'tauri/api/tauri';
import { Button, Flex, Select, Textarea, useToast } from '@chakra-ui/react';

import Header from '../components/sections/Header';

const List = ({ isOnline }) => {
    const [loading, setLoading] = useState(false);
    const [listOutput, setListOutput] = useState('');
    const [listType, setListType] = useState('');
    const toast = useToast();

    const getList = () => {
        if (listType !== '') {
            setLoading(true);
            promisified({
                cmd: 'list',
                listtype: listType,
            })
                .then((res) => {
                    const output = res.list.join('\n') ?? '';
                    setListOutput(output);
                    window.sessionStorage.setItem('list-cache', output);
                    setLoading(false);
                })
                .catch((err) => {
                    console.error(err);
                    setLoading(false);
                    toast({
                        title: 'Request failed!',
                        description: err,
                        status: 'error',
                        duration: 10000,
                        isClosable: true,
                    });
                });
        }
    };

    useEffect(() => {
        setListOutput(window.sessionStorage.getItem('list-cache') ?? '');
    }, []);

    return (
        <Flex direction="column" align="center" maxW={{ xl: '1240px' }} m="0 auto" h="100vh">
            <Header isDisabled={loading} isOnline={isOnline} />
            <Flex mb={4} direction="row">
                <Select
                    ml={2}
                    mr={2}
                    placeholder="Select type of list"
                    onChange={({ target: { value } }) => setListType(value)}
                >
                    <option value="allimages">allimages</option>
                    <option value="allpages">allpages</option>
                    <option value="alllinks">alllinks</option>
                    <option value="allcategories">allcategories</option>
                    <option value="backlinks">backlinks</option>
                    <option value="categorymembers">categorymembers</option>
                    <option value="embeddedin">embeddedin</option>
                    <option value="imageusage">imageusage</option>
                    <option value="iwbacklinks">iwbacklinks</option>
                    <option value="langbacklinks">langbacklinks</option>
                    <option value="search">search</option>
                    <option value="exturlusage">exturlusage</option>
                    <option value="protectedtitles">protectedtitles</option>
                    <option value="querypage">querypage</option>
                    <option value="wkpoppages">wkpoppages</option>
                    <option value="allinfoboxes">allinfoboxes</option>
                </Select>
                <Button ml={2} mr={2} onClick={getList} isLoading={loading}>
                    Get List
                </Button>
            </Flex>
            <Textarea value={listOutput} isReadOnly placeholder="Output will be displayed here" h="100%" mb={4} />
        </Flex>
    );
};

export default List;
