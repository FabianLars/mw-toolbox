import { Button, Flex, FormControl, FormLabel, Grid, GridItem, Textarea, Switch, Checkbox } from '@chakra-ui/react';
import { useState } from 'react';
import Header from '../components/Header';

const emptyLinePat = /^\s*[\r\n]/gm;

const Edit = ({ isOnline }) => {
    const [isRunning, setIsRunning] = useState(false);
    const [isAuto, setIsAuto] = useState(false);
    const [pageList, setPageList] = useState('');
    const [pageContent, setPageContent] = useState('');
    const [currentPage, setCurrentPage] = useState('Not running!');

    const startStop = () => {
        if (isRunning) {
        } else {
            getNextPage();
        }
        setIsRunning((state) => !state);
    };

    const getNextPage = () => {
        const pages = pageList
            .trim()
            .split(/\r?\n/)
            .map((el) => el.trim())
            .filter((el) => el);
        setCurrentPage(pages.shift());
        setPageList(pages.join('\n'));
    };

    const save = () => {};

    return (
        <Flex direction="column" align="center" p="0 1rem 1rem" h="100vh">
            <Header isOnline={isOnline} /* isDisabled={isLoading} */ />
            <Grid h="100%" w="100%" templateRows="repeat(3, 1fr)" templateColumns="repeat(4, 1fr)" gap={4}>
                <GridItem rowSpan={3} whiteSpace="nowrap">
                    <Textarea
                        isDisabled={isRunning}
                        resize="none"
                        h="100%"
                        placeholder="List of pages to operate on. Seperated by newline."
                        value={pageList}
                        onChange={(e) => setPageList(e.target.value)}
                    />
                </GridItem>
                <GridItem colSpan={3} rowSpan={2}>
                    <Textarea
                        isDisabled={isAuto}
                        resize="none"
                        h="100%"
                        placeholder="Page contents will be displayed here."
                        value={pageContent}
                        onChange={(e) => setPageContent(e.target.value)}
                    />
                </GridItem>
                <GridItem colSpan={3} display="flex" direction="row" justifyContent="space-between">
                    Current page: {currentPage}
                    <Flex direction="column" align="center" justify="space-between" h="100%">
                        <Button
                            w="100%"
                            onClick={startStop}
                            isDisabled={!isOnline}
                            title={!isOnline ? 'Please login first!' : 'This might take a while!'}
                        >
                            {isRunning ? 'Stop' : 'Start'}
                        </Button>
                        <Checkbox
                            isChecked={isAuto}
                            onChange={(event) => setIsAuto(event.target.checked)}
                            isDisabled={true /* TODO */}
                        >
                            Auto-Save
                        </Checkbox>
                        <Button w="100%" isDisabled={!isRunning || !currentPage} onClick={getNextPage}>
                            Skip
                        </Button>
                        <Button w="100%" isDisabled={!isRunning || !currentPage} onClick={save}>
                            Save
                        </Button>
                    </Flex>
                </GridItem>
            </Grid>
        </Flex>
    );
};

export default Edit;
