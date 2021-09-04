import React, { useEffect, useState } from 'react';

import { invoke } from '@tauri-apps/api/tauri';

import { errorToast } from '@/helpers/toast';
import { Button, Input, Label, Select, Textarea } from '@/components';
import { getCache, setCache } from '@/helpers/invoke';
import classes from './List.module.css';

type Props = {
    isOnline: boolean;
    setNavDisabled: React.Dispatch<React.SetStateAction<boolean>>;
};

const List = ({ isOnline, setNavDisabled }: Props): JSX.Element => {
    const [loading, setLoading] = useState(false);
    const [listOutput, setListOutput] = useState('');
    const [listType, setListType] = useState('');
    const [paramInfo, setParamInfo] = useState('');
    const [paramInput, setParamInput] = useState('');
    const [paramRequired, setParamRequired] = useState(true);

    const getList = () => {
        if (listType !== '') {
            setLoading(true);
            invoke<string[]>('list', {
                listtype: listType,
                param: paramInput || null,
            })
                .then((res) => {
                    const output = res.join('\n');
                    setListOutput(output);
                    setCache('list-cache', output);
                })
                .catch((err) => errorToast(err))
                .finally(() => setLoading(false));
        }
    };

    const clearOutput = () => {
        setCache('list-cache', '');
        setListOutput('');
    };

    useEffect(() => {
        getCache<string>('list-cache').then((res) => setListOutput(res ?? ''));
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
        <div className={classes.container}>
            <div className={classes.fields}>
                <div className={classes.endpoint}>
                    <Label htmlFor="listtype" isRequired>
                        API Endpoint
                    </Label>
                    <Select
                        id="listtype"
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
                </div>
                <div title={paramInfo} className={classes.parameter}>
                    <Label
                        htmlFor="parameter"
                        isRequired={paramRequired}
                        isDisabled={!paramRequired}
                    >
                        Required Parameter
                    </Label>
                    <Input
                        isDisabled={!paramRequired}
                        id="parameter"
                        placeholder={paramInfo}
                        value={paramInput}
                        onChange={(event) => setParamInput(event.target.value)}
                    />
                </div>
                <div className={classes.buttons}>
                    <Button
                        className={classes.mr}
                        onClick={getList}
                        isLoading={loading}
                        isDisabled={!isOnline || !listType || (paramRequired && !paramInput.trim())}
                        title={!isOnline ? 'Please login first!' : 'This might take a while!'}
                    >
                        Get List
                    </Button>
                    <Button onClick={clearOutput}>Clear Output</Button>
                </div>
            </div>
            <Textarea
                className={classes.area}
                label="output container"
                value={listOutput}
                readOnly
                placeholder="Output will be displayed here."
            />
        </div>
    );
};

export default List;
