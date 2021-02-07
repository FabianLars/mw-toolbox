import { Box, Button, Flex, Textarea, useToast } from '@chakra-ui/react';
import React, { useState } from 'react';
import { promisified } from 'tauri/api/tauri';
import Header from '../components/Header';

const Move = ({ isOnline }) => {
    const [isLoading, setIsLoading] = useState(false);
    const [areaFrom, setAreaFrom] = useState('');
    const [areaTo, setAreaTo] = useState('');
    const toast = useToast();

    const movePages = () => {
        setIsLoading(true);
        promisified({
            cmd: 'move',
            from: areaFrom.split(/\r?\n/),
            to: areaTo.split(/\r?\n/),
        })
            .then(() => {
                setIsLoading(false);
                toast({
                    title: 'Successfully moved pages',
                    description: 'Successfully moved pages.',
                    status: 'success',
                    isClosable: true,
                });
            })
            .catch((err) => {
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

    return (
        <Flex direction="column" align="center" p="0 1rem 1rem" h="100vh">
            <Header isOnline={isOnline} />
            <Flex direction="row" align="center" justify="center" h="100%" w="100%" mb={4}>
                <Textarea
                    resize="none"
                    value={areaFrom}
                    onChange={(e) => setAreaFrom(e.target.value)}
                    placeholder="Write exact names of pages to move. Seperated by newline."
                    h="100%"
                    mr={2}
                />
                <Textarea
                    resize="none"
                    value={areaTo}
                    onChange={(e) => setAreaTo(e.target.value)}
                    placeholder="Write exact names of destinations. Seperated by newline."
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
