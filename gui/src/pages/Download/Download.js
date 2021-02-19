import { Box, Button, Flex, Textarea, useToast } from '@chakra-ui/react';
import React, { useState } from 'react';
import { promisified } from 'tauri/api/tauri';
import { Header } from '../../components';

const Download = ({ isOnline }) => {
    const [areaValue, setAreaValue] = useState('');
    const [isLoading, setIsLoading] = useState(false);
    const toast = useToast();

    const downloadFiles = () => {
        setIsLoading(true);
        promisified({
            cmd: 'download',
            files: areaValue.split(/\r?\n/),
        })
            .then(() =>
                toast({
                    title: 'Download successful',
                    description: 'Download successful! Check your download folder.',
                    status: 'success',
                    isClosable: true,
                })
            )
            .catch(err =>
                toast({
                    title: 'Something went wrong!',
                    description: err,
                    status: 'error',
                    duration: 10000,
                    isClosable: true,
                })
            )
            .finally(() => setIsLoading(false));
    };

    return (
        <Flex direction="column" align="center" p="0 1rem 1rem" h="100vh">
            <Header isOnline={isOnline} isDisabled={isLoading} />
            <Textarea
                resize="none"
                value={areaValue}
                onChange={event => setAreaValue(event.target.value)}
                placeholder="Write exact page names here. Separated by newline. Inclusive 'File:' Prefix. Saved in your download folder."
                h="100%"
                mb={4}
            />
            <Box>
                <Button
                    isLoading={isLoading}
                    isDisabled={!isOnline}
                    onClick={downloadFiles}
                    loadingText="Downloading..."
                    title={!isOnline ? 'Please login first!' : 'This might take a while!'}
                >
                    Download all
                </Button>
            </Box>
        </Flex>
    );
};

export default Download;
