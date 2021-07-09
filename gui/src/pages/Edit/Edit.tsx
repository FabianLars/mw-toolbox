import {
    Button,
    Checkbox,
    Flex,
    Grid,
    GridItem,
    Input,
    Textarea,
    useDisclosure,
    useToast,
} from '@chakra-ui/react';
import React, { useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import FindReplaceModal from './FindReplaceModal';
import { listen, emit } from '@tauri-apps/api/event';
import { errorToast, successToast } from '../../helpers/toast';
import { removeFirst } from '../../helpers/array';

type Pattern = {
    find: string;
    replace: string;
    isRegex: boolean;
};

type Props = {
    isOnline: boolean;
    setNavDisabled: React.Dispatch<React.SetStateAction<boolean>>;
};

const Edit = ({ isOnline, setNavDisabled }: Props) => {
    const [isRunning, setIsRunning] = useState(false);
    const [isAuto, setIsAuto] = useState(false);
    const [isLoading, setIsLoading] = useState(false);
    const [pageList, setPageList] = useState(['']);
    const [pageContent, setPageContent] = useState('');
    const [currentPage, setCurrentPage] = useState('');
    const [editSummary, setEditSummary] = useState('');
    const [patterns, setPatterns] = useState<Pattern[]>([
        { find: '', replace: '', isRegex: false },
    ]);
    const { isOpen, onOpen, onClose } = useDisclosure();
    const toast = useToast();

    const start = () => {
        setIsRunning(true);
        if (isAuto) {
            setPageContent('');
            console.log('start', pageList);
            invoke('auto_edit', {
                titles: pageList,
                patterns,
                summary: editSummary,
            })
                .catch((err) => {
                    toast(errorToast(err));
                })
                .finally(stop);
        } else {
            getNextPage();
        }
    };

    const stop = () => {
        if (isAuto) {
            emit('cancel-autoedit');
        }
        setPageList((state) => [currentPage, ...state].filter(Boolean));
        setPageContent('');
        setIsRunning(false);
    };

    const getNextPage = () => {
        setPageContent('');
        setIsLoading(true);
        const pages = pageList;
        const curr = pages.shift();
        console.log(!curr, pages);
        setCurrentPage(curr ?? '');
        setPageList(pages);
        if (!curr) {
            setIsRunning(false);
            setIsLoading(false);
        } else {
            (
                invoke('get_page', {
                    page: curr,
                    patterns: patterns,
                }) as Promise<{ content: string; edited: boolean }>
            )
                .then(({ content }) => {
                    setPageContent(content);
                })
                .catch((err) => {
                    stop();
                    toast(errorToast(err));
                })
                .finally(() => setIsLoading(false));
        }
    };

    const save = () => {
        setIsLoading(true);
        (
            invoke('edit', {
                title: currentPage,
                content: pageContent
                    .replace(/[\u007F-\u009F\u200B]/g, '')
                    .replace(/…/g, '...')
                    .trim(),
                summary: editSummary || null,
            }) as Promise<string>
        )
            .then((res) => {
                toast(successToast('Edit successful', res));
                getNextPage();
            })
            .catch((err) => {
                setIsLoading(false);
                toast(errorToast(err));
            });
    };

    useEffect(
        () => setNavDisabled(isLoading || isAuto ? isRunning : false),
        [isLoading, isRunning],
    );

    useEffect(() => {
        const unlistenEdited = listen('page-edited', ({ payload }: { payload: string }) => {
            setPageList((old) => removeFirst(old, payload));
        });
        const unlistenSkipped = listen('page-skipped', ({ payload }: { payload: string }) => {
            setPageList((old) => removeFirst(old, payload));
        });

        const getCache = async () => {
            const list: string[] | null = await invoke('cache_get', { key: 'edit-pagelist' });
            const patts: Pattern[] | null = await invoke('cache_get', { key: 'edit-patterns' });
            const summary: string | null = await invoke('cache_get', { key: 'edit-summary' });
            const auto: boolean | null = await invoke('cache_get', { key: 'edit-isauto' });

            if (list) setPageList(list);
            if (patts) setPatterns(patts);
            if (summary) setEditSummary(summary);
            if (auto) setIsAuto(auto);
        };

        getCache();

        return () => {
            unlistenEdited.then((f) => f());
            unlistenSkipped.then((f) => f());
        };
    }, []);

    return (
        <>
            <Flex w="100%" h="100%" direction={['column', null, 'row']}>
                <Textarea
                    w={[null, null, '30%', '25%', '20%']}
                    isDisabled={isRunning}
                    resize="none"
                    mb={[4, null, 0]}
                    h={['20%', null, '100%']}
                    placeholder="List of pages to operate on. Separated by newline."
                    value={pageList.join('\n')}
                    onChange={(event) => setPageList(event.target.value.split(/\r?\n/))}
                    onBlur={() => {
                        setPageList((old) => {
                            return old.map((el: string) => el.trim()).filter(Boolean);
                        });
                        invoke('cache_set', { key: 'edit-pagelist', value: pageList });
                    }}
                />
                <Flex direction="column" flex="1" ml={[null, null, 4]}>
                    <Textarea
                        flex="2"
                        isDisabled={isAuto || isLoading || !isRunning}
                        resize="none"
                        h="100%"
                        placeholder="Page contents will be displayed here."
                        value={pageContent}
                        onChange={(event) => setPageContent(event.target.value)}
                    />
                    <Grid
                        pt={4}
                        flex="1"
                        templateColumns="repeat(7, 1fr)"
                        templateRows="repeat(4, 1fr)"
                        columnGap={4}
                        maxH="250px"
                    >
                        <GridItem colSpan={[4, null, 2]} mt={2} overflow="hidden">
                            Current page:{' '}
                            {isRunning
                                ? isAuto
                                    ? 'Automated saving mode...'
                                    : currentPage
                                : 'Not running!'}
                        </GridItem>
                        <GridItem rowStart={4}>
                            <Button
                                mt={2}
                                title="This will be processed before contents get displayed!"
                                onClick={onOpen}
                                isDisabled={isLoading}
                            >
                                Setup Find & Replace
                            </Button>
                        </GridItem>
                        <GridItem
                            colSpan={[6, null, 4]}
                            colStart={[1, null, 3]}
                            rowStart={[3, null, 1]}
                        >
                            <Input
                                placeholder="Edit summary"
                                value={editSummary}
                                onChange={(event) => setEditSummary(event.target.value)}
                                onBlur={() =>
                                    invoke('cache_set', { key: 'edit-summary', value: editSummary })
                                }
                            />
                        </GridItem>
                        <GridItem rowSpan={4} colStart={7} colSpan={1}>
                            <Flex
                                direction="column"
                                align="center"
                                justify="space-between"
                                h="100%"
                            >
                                <Button
                                    w="100%"
                                    onClick={() => (isRunning ? stop() : start())}
                                    isDisabled={!isOnline || isLoading || pageList.join('') === ''}
                                    title={
                                        !isOnline
                                            ? 'Please login first!'
                                            : isRunning
                                            ? ''
                                            : 'This might take a while!'
                                    }
                                >
                                    {isRunning ? 'Stop' : 'Start'}
                                </Button>
                                <Checkbox
                                    isChecked={isAuto}
                                    onChange={(event) => {
                                        setIsAuto(event.target.checked);
                                    }}
                                    onBlur={() => {
                                        invoke('cache_set', {
                                            key: 'edit-isauto',
                                            value: isAuto,
                                        });
                                    }}
                                    isDisabled={isRunning}
                                    whiteSpace="nowrap"
                                >
                                    Auto-Save
                                </Checkbox>
                                <Button
                                    w="100%"
                                    isDisabled={!isRunning || !currentPage}
                                    isLoading={isLoading}
                                    onClick={getNextPage}
                                >
                                    Skip
                                </Button>
                                <Button
                                    w="100%"
                                    isDisabled={!isRunning || !currentPage}
                                    isLoading={isLoading}
                                    onClick={save}
                                >
                                    Save
                                </Button>
                            </Flex>
                        </GridItem>
                    </Grid>
                </Flex>
            </Flex>

            <FindReplaceModal
                isOpen={isOpen}
                onClose={onClose}
                patterns={patterns}
                setPatterns={setPatterns}
            />
        </>
    );
};

export default Edit;
