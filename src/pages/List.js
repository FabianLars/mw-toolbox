import React, { useEffect, useState } from 'react';

import { promisified } from 'tauri/api/tauri';
import { Box, Button, Flex, FormControl, FormLabel, Input, Select, Textarea, useToast } from '@chakra-ui/react';

import Header from '../components/Header';

const List = ({ isOnline }) => {
    const [loading, setLoading] = useState(false);
    const [listOutput, setListOutput] = useState('');
    const [listType, setListType] = useState('');
    const [paramInfo, setParamInfo] = useState('');
    const [paramInput, setParamInput] = useState('');
    const toast = useToast();

    const getList = () => {
        if (listType !== '') {
            setLoading(true);
            promisified({
                cmd: 'list',
                listtype: listType,
                param: paramInput !== '' ? paramInput : null,
            })
                .then((res) => {
                    const output = res.list.join('\n') ?? '';
                    setListOutput(output);
                    window.sessionStorage.setItem('list-cache', output);
                    setLoading(false);
                })
                .catch((err) => {
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

    const clearOutput = () => {
        window.sessionStorage.setItem('list-cache', '');
        setListOutput('');
    };

    useEffect(() => {
        setListOutput(window.sessionStorage.getItem('list-cache') ?? '');
    }, []);

    useEffect(() => {
        setParamInput('');
        switch (listType) {
            case 'allpages':
                setParamInfo("Namespace id or 'all'");
                break;
            case 'backlinks':
                setParamInfo('Title to search');
                break;
            case 'categorymembers':
                setParamInfo("Category (incl. 'Category:' prefix)");
                break;
            case 'embeddedin':
                setParamInfo("Template to search (incl. 'Template:' prefix)");
                break;
            case 'imagesearch':
                setParamInfo("Image to search (incl. 'File:' prefix)");
                break;
            case 'querypage':
                setParamInfo('Title to special page');
                break;
            case 'search':
                setParamInfo('Search');
                break;
            default:
                setParamInfo('');
        }
    }, [listType]);

    return (
        <Flex direction="column" align="center" m="0 1rem" h="100vh">
            <Header isDisabled={loading} isOnline={isOnline} />
            <Flex w="100%" mb={4} direction="row" align="center">
                {paramInfo === '' ? (
                    <Box mx={2} w="100%"></Box>
                ) : (
                    <FormControl mx={2} isRequired visibility={paramInfo === '' ? 'hidden' : ''}>
                        <FormLabel htmlFor="parameter-input">Required Parameter</FormLabel>
                        <Input
                            id="parameter-input"
                            placeholder={paramInfo}
                            title={paramInfo}
                            value={paramInput}
                            onChange={(event) => setParamInput(event.target.value)}
                        />
                    </FormControl>
                )}
                <FormControl mx={2} isRequired>
                    <FormLabel htmlFor="listtype-dropdown">API Endpoint</FormLabel>
                    <Select
                        id="listtype-dropdown"
                        placeholder="Select type of list"
                        onChange={({ target: { value } }) => setListType(value)}
                    >
                        <option value="allcategories">allcategories</option>
                        <option value="allimages">allimages</option>
                        <option value="allinfoboxes">allinfoboxes</option>
                        <option value="alllinks">alllinks</option>
                        <option value="allpages">allpages</option>
                        <option value="backlinks">backlinks</option>
                        <option value="categorymembers">categorymembers</option>
                        <option value="embeddedin">embeddedin</option>
                        <option value="exturlusage">exturlusage</option>
                        <option value="imageusage">imageusage</option>
                        <option value="protectedtitles">protectedtitles</option>
                        <option value="querypage">querypage</option>
                        <option value="search">search</option>
                    </Select>
                </FormControl>
                <Button
                    mx={2}
                    onClick={getList}
                    isLoading={loading}
                    isDisabled={!isOnline}
                    title={!isOnline ? 'Please login first!' : 'This might take a while!'}
                >
                    Get List
                </Button>
                <Button mx={2} onClick={clearOutput}>
                    Clear Output
                </Button>
            </Flex>
            <Textarea
                resize="none"
                value={listOutput}
                isReadOnly
                placeholder="Output will be displayed here"
                h="100%"
                mb={4}
            />
        </Flex>
    );
};

export default List;
