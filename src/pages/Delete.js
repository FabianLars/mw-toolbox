import { Box, Button, Flex, Textarea, useToast } from '@chakra-ui/react';
import React, { useState } from 'react';
import { promisified } from 'tauri/api/tauri';
import Header from '../components/sections/Header';

const Delete = ({ isOnline }) => {
    const [areaValue, setAreaValue] = useState('');
    const [isLoading, setIsLoading] = useState(false);
    const toast = useToast();

    const deletePages = () => {
        setIsLoading(true);
        promisified({
            cmd: 'delete',
            pages: areaValue.split(/\r?\n/),
        })
            .then((res) => {
                setIsLoading(false);
                toast({
                    title: 'Delete successful',
                    description: res.message,
                    status: 'success',
                    isClosable: true,
                });
            })
            .catch((err) => {
                setIsLoading(false);
                console.error(err);
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
                value={areaValue}
                onChange={(e) => setAreaValue(e.target.value)}
                placeholder="Write exact page names here. Separated by newline."
                h="100%"
                mb={4}
            />
            <Box mb={4}>
                <Button isLoading={isLoading} onClick={deletePages} loadingText="Deleting">
                    Delete all
                </Button>
            </Box>
        </Flex>
    );
};

export default Delete;
