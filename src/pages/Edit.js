import { Flex, Grid, GridItem, Textarea } from '@chakra-ui/react';
import Header from '../components/Header';

const Edit = ({ isOnline }) => {
    return (
        <Flex direction="column" align="center" p="0 1rem 1rem" h="100vh">
            <Header isOnline={isOnline} /* isDisabled={isLoading} */ />
            <Grid h="100%" w="100%" templateRows="repeat(2, 1fr)" templateColumns="repeat(4, 1fr)" gap={4} mb={4}>
                <GridItem rowSpan={2}>
                    <Textarea resize="none" h="100%" placeholder="List of pages to operate on. Seperated by newline." />
                </GridItem>
                <GridItem colSpan={3}>
                    <Textarea resize="none" h="100%" placeholder="Page contents will be displayed here." />
                </GridItem>
                <GridItem colSpan={3}>Controls</GridItem>
            </Grid>
        </Flex>
    );
};

export default Edit;
