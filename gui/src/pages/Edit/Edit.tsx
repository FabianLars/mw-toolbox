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
import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import FindReplaceModal from './FindReplaceModal';

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
    const [pageList, setPageList] = useState('');
    const [pageContent, setPageContent] = useState('');
    const [currentPage, setCurrentPage] = useState('');
    const [editSummary, setEditSummary] = useState('');
    const [patterns, setPatterns] = useState<Pattern[]>([
        { find: '', replace: '', isRegex: false },
    ]);
    const { isOpen, onOpen, onClose } = useDisclosure();
    const toast = useToast();

    const startStop = () => {
        if (isRunning) {
            setPageList((state) => [currentPage, state].filter(Boolean).join('\n'));
            setPageContent('');
        } else {
            getNextPage();
        }
        setIsRunning((state) => !state);
    };

    const getNextPage = () => {
        setPageContent('');
        setIsLoading(true);
        const pages = pageList
            .trim()
            .split(/\r?\n/)
            .map((el) => el.trim())
            .filter((el) => el);
        const curr = pages.shift();
        setCurrentPage(curr ?? '');
        setPageList(pages.join('\n'));
        if (!curr) {
            setIsRunning(false);
            setIsLoading(false);
        } else {
            (
                invoke('get_page', {
                    page: curr,
                    patterns: patterns,
                }) as Promise<string>
            )
                .then(setPageContent)
                .catch((err) => {
                    startStop();
                    toast({
                        title: `Something went wrong! ${err.code}-Error`,
                        description: err.description,
                        status: 'error',
                        duration: 10000,
                        isClosable: true,
                    });
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
                toast({
                    title: 'Edit successful',
                    description: res,
                    status: 'success',
                    duration: 1000,
                    isClosable: true,
                });
                getNextPage();
            })
            .catch((err) => {
                setIsLoading(false);
                toast({
                    title: `Something went wrong! ${err.code}-Error`,
                    description: err.description,
                    status: 'error',
                    duration: 10000,
                    isClosable: true,
                });
            });
    };

    useEffect(() => {
        if (isAuto && pageContent !== '') {
            save();
        }
    }, [pageContent]);

    useEffect(() => setNavDisabled(isLoading), [isLoading]);

    return (
        <>
            <Flex w="100%" h="100%">
                <Textarea
                    w="20%"
                    isDisabled={isRunning}
                    resize="none"
                    h="100%"
                    placeholder="List of pages to operate on. Separated by newline."
                    value={pageList}
                    onChange={(event) => setPageList(event.target.value)}
                />
                <Flex direction="column" flex="1" ml={4}>
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
                        <GridItem colSpan={4} mt={2}>
                            Current page: {isRunning ? currentPage : 'Not running!'}
                        </GridItem>
                        <GridItem rowStart={4} colSpan={2}>
                            <Button
                                mt={2}
                                title="This will be processed before contents get displayed!"
                                onClick={onOpen}
                                isDisabled={isLoading}
                            >
                                Setup Find & Replace
                            </Button>
                        </GridItem>
                        <GridItem colSpan={2}>
                            <Input
                                placeholder="Edit summary"
                                value={editSummary}
                                onChange={(event) => setEditSummary(event.target.value)}
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
                                    onClick={startStop}
                                    isDisabled={!isOnline || isLoading || pageList.trim() === ''}
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
                                    onChange={(event) => setIsAuto(event.target.checked)}
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
