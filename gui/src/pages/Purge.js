import React, { useState } from 'react';
import { Button, Flex, Textarea, useToast } from '@chakra-ui/react';
import { promisified } from 'tauri/api/tauri';
import Header from '../components/Header';

const Purge = ({ isOnline }) => {
    const [isPurging, setIsPurging] = useState(false);
    const [isNulling, setIsNulling] = useState(false);
    const [areaValue, setAreaValue] = useState('');
    const toast = useToast();

    const purgePages = (isNulledit) => {
        if (isNulledit) {
            setIsNulling(true);
        } else {
            setIsPurging(true);
        }
        promisified({
            cmd: 'purge',
            pages: areaValue.split(/\r?\n/),
            is_nulledit: isNulledit,
        })
            .then(() => {
                setIsPurging(false);
                setIsNulling(false);
                toast({
                    title: (isNulledit ? 'Nulledit' : 'Purge') + ' successful',
                    description: (isNulledit ? 'Nulledit' : 'Purge') + ' successful',
                    status: 'success',
                    isClosable: true,
                });
            })
            .catch((err) => {
                setIsPurging(false);
                setIsNulling(false);
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
            <Flex w="100%" h="100%" direction="column">
                <Textarea
                    resize="none"
                    value={areaValue}
                    onChange={(e) => setAreaValue(e.target.value)}
                    placeholder="Write exact page names here. Separated by newline."
                    h="100%"
                    mb={4}
                />
                <Flex direction="row" align="center" justify="center">
                    <Button
                        isLoading={isPurging}
                        isDisabled={!isOnline}
                        onClick={() => purgePages(false)}
                        loadingText="Purging"
                        title={!isOnline ? 'Please login first!' : 'Clear server caches. This might take a while!'}
                        mx={2}
                    >
                        Purge all
                    </Button>
                    <Button
                        isLoading={isNulling}
                        isDisabled={!isOnline}
                        onClick={() => purgePages(true)}
                        loadingText="Saving nulledits"
                        title={
                            !isOnline ? 'Please login first!' : 'Do a nulledit on every page. This might take a while!'
                        }
                        mx={2}
                    >
                        Nulledit all
                    </Button>
                </Flex>
            </Flex>
        </Flex>
    );
};

export default Purge;
