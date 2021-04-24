import { Box, Button, Flex, Textarea, useToast } from '@chakra-ui/react';
import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Header } from '../../components';

const Move = ({ isOnline }: { isOnline: boolean }) => {
    const [isLoading, setIsLoading] = useState(false);
    const [areaFrom, setAreaFrom] = useState('');
    const [areaTo, setAreaTo] = useState('');
    const toast = useToast();

    const movePages = () => {
        setIsLoading(true);
        invoke('move', {
            from: areaFrom.split(/\r?\n/),
            to: areaTo.split(/\r?\n/),
        })
            .then(() =>
                toast({
                    title: 'Successfully moved pages',
                    description: 'Successfully moved pages.',
                    status: 'success',
                    isClosable: true,
                }),
            )
            .catch((err) =>
                toast({
                    title: 'Something went wrong!',
                    description: err.Err,
                    status: 'error',
                    duration: 10000,
                    isClosable: true,
                }),
            )
            .finally(() => setIsLoading(false));
    };

    return (
        <Flex direction="column" align="center" p="0 1rem 1rem" h="100vh">
            <Header isOnline={isOnline} isDisabled={isLoading} />
            <Flex direction="row" align="center" justify="center" flex="1" w="100%" mb={4}>
                <Textarea
                    resize="none"
                    value={areaFrom}
                    onChange={(event) => setAreaFrom(event.target.value)}
                    placeholder="Write exact names of pages to move. Separated by newline."
                    h="100%"
                    mr={2}
                />
                <Textarea
                    resize="none"
                    value={areaTo}
                    onChange={(event) => setAreaTo(event.target.value)}
                    placeholder="Write exact names of destinations. Separated by newline."
                    h="100%"
                    ml={2}
                />
            </Flex>
            <Box>
                <Button
                    isLoading={isLoading}
                    isDisabled={!isOnline}
                    onClick={movePages}
                    loadingText="Moving..."
                    title={!isOnline ? 'Please login first!' : 'This might take a while!'}
                >
                    Start moving
                </Button>
            </Box>
        </Flex>
    );
};

export default Move;
