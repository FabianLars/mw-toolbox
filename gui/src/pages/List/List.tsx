import React, { useEffect, useState } from 'react';

import { invoke } from '@tauri-apps/api/tauri';
import {
    Box,
    Button,
    Flex,
    FormControl,
    FormLabel,
    Input,
    Select,
    useToast,
} from '@chakra-ui/react';

import { Output } from '../../components';
import { errorToast } from '../../helpers/toast';

type Props = {
    isOnline: boolean;
    setNavDisabled: React.Dispatch<React.SetStateAction<boolean>>;
};

const List = ({ isOnline, setNavDisabled }: Props) => {
    const [loading, setLoading] = useState(false);
    const [listOutput, setListOutput] = useState('');
    const [listType, setListType] = useState('');
    const [paramInfo, setParamInfo] = useState('');
    const [paramInput, setParamInput] = useState('');
    const [paramRequired, setParamRequired] = useState(true);
    const toast = useToast();

    const getList = () => {
        if (listType !== '') {
            setLoading(true);
            (
                invoke('list', {
                    listtype: listType,
                    param: paramInput || null,
                }) as Promise<string[]>
            )
                .then((res) => {
                    const output = res.join('\n');
                    setListOutput(output);
                    invoke('cache_set', { key: 'list-cache', value: output });
                })
                .catch((err) => toast(errorToast(err)))
                .finally(() => setLoading(false));
        }
    };

    const clearOutput = () => {
        invoke('cache_set', { key: 'list-cache', value: '' });
        setListOutput('');
    };

    useEffect(() => {
        (invoke('cache_get', { key: 'list-cache' }) as Promise<string | null>).then((res) =>
            setListOutput(res ?? ''),
        );
    }, []);

    useEffect(() => {
        setParamInput('');
        let paramReq = true;
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
                paramReq = false;
                setParamInfo('');
        }
        setParamRequired(paramReq);
    }, [listType]);

    useEffect(() => setNavDisabled(loading), [loading]);

    return (
        <Flex direction="column" align="center" h="100%" w="100%">
            <Flex w="100%" mb={4} direction="row" align="center">
                {paramInfo === '' ? (
                    <Box mx={2} w="100%" />
                ) : (
                    <FormControl
                        id="parameter-input"
                        mx={2}
                        isRequired
                        visibility={paramInfo === '' ? 'hidden' : undefined}
                    >
                        <FormLabel>Required Parameter</FormLabel>
                        <Input
                            placeholder={paramInfo}
                            title={paramInfo}
                            value={paramInput}
                            onChange={(event) => setParamInput(event.target.value)}
                        />
                    </FormControl>
                )}
                <FormControl id="listtype-dropdown" mx={2} isRequired>
                    <FormLabel>API Endpoint</FormLabel>
                    <Select
                        placeholder="Select type of list"
                        onChange={(event) => setListType(event.target.value)}
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
                <Box>
                    <Button
                        mx={2}
                        onClick={getList}
                        isLoading={loading}
                        isDisabled={!isOnline || !listType || (paramRequired && !paramInput.trim())}
                        title={!isOnline ? 'Please login first!' : 'This might take a while!'}
                    >
                        Get List
                    </Button>
                </Box>
                <Box>
                    <Button mx={2} onClick={clearOutput}>
                        Clear Output
                    </Button>
                </Box>
            </Flex>
            <Output placeholder="Output will be displayed here.">{listOutput}</Output>
        </Flex>
    );
};

export default List;
