import { Box, Button, Flex, Textarea, useToast } from '@chakra-ui/react';
import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

type Props = {
    isOnline: boolean;
    setNavDisabled: React.Dispatch<React.SetStateAction<boolean>>;
};

const Download = ({ isOnline, setNavDisabled }: Props) => {
    const [areaValue, setAreaValue] = useState('');
    const [isLoading, setIsLoading] = useState(false);
    const toast = useToast();

    const downloadFiles = () => {
        setIsLoading(true);
        invoke('download', {
            files: areaValue.split(/\r?\n/),
        })
            .then(() =>
                toast({
                    title: 'Download successful',
                    description: 'Download successful! Check your download folder.',
                    status: 'success',
                    isClosable: true,
                }),
            )
            .catch((err) =>
                toast({
                    title: 'Something went wrong!',
                    description: err,
                    status: 'error',
                    duration: 10000,
                    isClosable: true,
                }),
            )
            .finally(() => setIsLoading(false));
    };

    useEffect(() => setNavDisabled(isLoading), [isLoading]);

    return (
        <Flex direction="column" align="center" h="100%" w="100%">
            <Textarea
                resize="none"
                value={areaValue}
                onChange={(event) => setAreaValue(event.target.value)}
                placeholder="Write exact page names here. Separated by newline. Inclusive 'File:' Prefix. Saved in your download folder."
                flex="1"
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
