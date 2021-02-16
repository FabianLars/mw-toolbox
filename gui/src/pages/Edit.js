import {
    Button,
    Flex,
    Grid,
    GridItem,
    Textarea,
    Checkbox,
    useToast,
    Input,
    useDisclosure,
    Modal,
    ModalBody,
    ModalOverlay,
    ModalContent,
    ModalHeader,
    ModalFooter,
    Spacer,
} from '@chakra-ui/react';
import { useState } from 'react';
import { promisified } from 'tauri/api/tauri';
import Header from '../components/Header';

const Edit = ({ isOnline }) => {
    const [isRunning, setIsRunning] = useState(false);
    const [isAuto, setIsAuto] = useState(false);
    const [isLoading, setIsLoading] = useState(false);
    const [pageList, setPageList] = useState('');
    const [pageContent, setPageContent] = useState('');
    const [currentPage, setCurrentPage] = useState('Not running!');
    const [editSummary, setEditSummary] = useState('');
    const [searches, setSearches] = useState([{}]);
    const [oldSearches, setOldSearches] = useState([{}]);
    const { isOpen, onOpen, onClose } = useDisclosure();
    const toast = useToast();

    const startStop = () => {
        if (isRunning) {
            setPageList(state => currentPage + '\n' + state);
            setPageContent('');
        } else {
            getNextPage();
        }
        setIsRunning(state => !state);
    };

    const getNextPage = () => {
        setPageContent('');
        setIsLoading(true);
        const pages = pageList
            .trim()
            .split(/\r?\n/)
            .map(el => el.trim())
            .filter(el => el);
        const curr = pages.shift();
        setCurrentPage(curr);
        setPageList(pages.join('\n'));
        if (!curr) {
            setIsRunning(false);
            setIsLoading(false);
        } else {
            promisified({
                cmd: 'getPage',
                page: curr,
            })
                .then(setPageContent)
                .catch(err => {
                    startStop();
                    toast({
                        title: 'Something went wrong!',
                        description: err,
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
        promisified({
            cmd: 'edit',
            title: currentPage,
            content: pageContent
                .replace(/[\u007F-\u009F\u200B]/g, '')
                .replaceAll('…', '...')
                .trim(),
            summary: editSummary || null,
        })
            .then(res => {
                toast({
                    title: 'Edit successful',
                    description: res,
                    status: 'success',
                    duration: 1000,
                    isClosable: true,
                });
                getNextPage();
            })
            .catch(err => {
                setIsLoading(false);
                toast({
                    title: 'Something went wrong!',
                    description: err,
                    status: 'error',
                    duration: 10000,
                    isClosable: true,
                });
            });
    };

    const onModalClose = () => {
        const arr = oldSearches.map(obj => Object.assign({}, obj));
        setSearches(arr);
        onClose();
    };

    const onModalSave = () => {
        const arr = searches.map(obj => Object.assign({}, obj));
        setOldSearches(arr);
        onClose();
    };

    return (
        <>
            <Flex direction="column" align="center" p="0 1rem 1rem" h="100vh">
                <Header isOnline={isOnline} isDisabled={isLoading} />
                <Grid h="100%" w="100%" templateRows="repeat(3, 1fr)" templateColumns="repeat(4, 1fr)" gap={4}>
                    <GridItem rowSpan={3} whiteSpace="nowrap">
                        <Textarea
                            isDisabled={isRunning}
                            resize="none"
                            h="100%"
                            placeholder="List of pages to operate on. Seperated by newline."
                            value={pageList}
                            onChange={event => setPageList(event.target.value)}
                        />
                    </GridItem>
                    <GridItem colSpan={3} rowSpan={2}>
                        <Textarea
                            isDisabled={isAuto || isLoading || !isRunning}
                            resize="none"
                            h="100%"
                            placeholder="Page contents will be displayed here."
                            value={pageContent}
                            onChange={event => setPageContent(event.target.value)}
                        />
                    </GridItem>
                    <GridItem colSpan={3}>
                        <Grid templateColumns="repeat(8, 1fr)" templateRows="repeat(4, 1fr)" h="100%">
                            <GridItem colSpan={4} mt={2}>
                                Current page: {currentPage}
                            </GridItem>
                            <GridItem rowStart={4} colSpan={2}>
                                <Button
                                    mt={2}
                                    title="This will be processed before contents get displayed!"
                                    onClick={onOpen}
                                >
                                    Setup Search & Replace
                                </Button>
                            </GridItem>
                            <GridItem colSpan={3} mr={4}>
                                <Input
                                    placeholder="Edit summary"
                                    value={editSummary}
                                    onChange={event => setEditSummary(event.target.value)}
                                />
                            </GridItem>
                            <GridItem rowSpan={4} colStart={8}>
                                <Flex direction="column" align="center" justify="space-between" h="100%">
                                    <Button
                                        w="100%"
                                        onClick={startStop}
                                        isDisabled={!isOnline || isLoading}
                                        title={
                                            !isOnline
                                                ? 'Please login first!'
                                                : isRunning
                                                ? 'Skip all remaining pages'
                                                : 'This might take a while!'
                                        }
                                    >
                                        {isRunning ? 'Stop' : 'Start'}
                                    </Button>
                                    <Checkbox
                                        isChecked={isAuto}
                                        onChange={event => setIsAuto(event.target.checked)}
                                        isDisabled={true /* TODO */}
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
                    </GridItem>
                </Grid>
            </Flex>

            <Modal onClose={onModalClose} isOpen={isOpen} isCentered size="xl">
                <ModalOverlay />
                <ModalContent>
                    <ModalHeader>Search & Replace</ModalHeader>
                    <ModalBody>
                        <Flex direction="column" h="100%" w="100%">
                            {searches.map((_, index) => (
                                <Flex key={index}>
                                    <Input
                                        m={1}
                                        placeholder="Search"
                                        value={searches[index][0] || ''}
                                        onKeyDown={e => {
                                            if (e.key === 'Enter') onModalSave();
                                        }}
                                        onChange={event =>
                                            setSearches(oldArr => {
                                                const values = [...oldArr];
                                                values[index][0] = event.target.value;
                                                return values;
                                            })
                                        }
                                    />
                                    <Input
                                        m={1}
                                        placeholder="Replace"
                                        value={searches[index][1] || ''}
                                        onKeyDown={e => {
                                            if (e.key === 'Enter') onModalSave();
                                        }}
                                        onChange={event =>
                                            setSearches(oldArr => {
                                                const values = [...oldArr];
                                                values[index][1] = event.target.value;
                                                return values;
                                            })
                                        }
                                    />
                                </Flex>
                            ))}
                        </Flex>
                    </ModalBody>
                    <ModalFooter>
                        <Button
                            mr={2}
                            onClick={() => {
                                if (searches.length < 10) setSearches(old => old.concat({}));
                            }}
                            isDisabled={searches.length >= 10}
                        >
                            Add Row
                        </Button>
                        <Button colorScheme="red" onClick={() => setSearches([{}])}>
                            Clear all
                        </Button>
                        <Spacer />
                        <Button colorScheme="blue" mr={2} onClick={onModalSave}>
                            Save
                        </Button>
                        <Button onClick={onModalClose}>Cancel</Button>
                    </ModalFooter>
                </ModalContent>
            </Modal>
        </>
    );
};

export default Edit;
