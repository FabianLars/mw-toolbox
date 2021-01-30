import { Box, Button, Flex, Textarea, useToast } from '@chakra-ui/react';
import React, { useState } from 'react';
import { promisified } from 'tauri/api/tauri';
import Header from '../components/Header';

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
            .then((res) => {
                setIsLoading(false);
                toast({
                    title: 'Download successful',
                    description: res.message,
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
        <Flex direction="column" align="center" m="0 1rem" h="100vh">
            <Header isOnline={isOnline} isDisabled={isLoading} />
            <Textarea
                resize="none"
                value={areaValue}
                onChange={(e) => setAreaValue(e.target.value)}
                placeholder="Write exact page names here. Separated by newline. Inclusive 'File:' Prefix. Saved in your download folder."
                h="100%"
                mb={4}
            />
            <Box mb={4}>
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
