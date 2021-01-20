import React, { useState } from 'react';
import { Button, Flex, Textarea, useToast } from '@chakra-ui/react';
import { promisified } from 'tauri/api/tauri';

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
            .then((res) => {
                setIsPurging(false);
                setIsNulling(false);
                toast({
                    title: (isNulledit ? 'Nulledit' : 'Purge') + ' successful',
                    description: res.message,
                    status: 'success',
                    isClosable: true,
                });
            })
            .catch((err) => {
                setIsPurging(false);
                setIsNulling(false);
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
        <Flex w="100%" h="100%" direction="column">
            <Textarea
                value={areaValue}
                onChange={(e) => setAreaValue(e.target.value)}
                placeholder="Write exact page names here. Separated by newline."
                h="100%"
                mb={4}
            />
            <Flex direction="row" align="center" justify="center" mb={4}>
                <Button
                    isLoading={isPurging}
                    isDisabled={!isOnline}
                    onClick={() => purgePages(false)}
                    loadingText="Purging"
                    title="Clear server caches"
                    mx={2}
                >
                    Purge all
                </Button>
                <Button
                    isLoading={isNulling}
                    isDisabled={!isOnline}
                    onClick={() => purgePages(true)}
                    loadingText="Saving nulledits"
                    title="Do a nulledit on every page. This might take a while!"
                    mx={2}
                >
                    Nulledit all
                </Button>
            </Flex>
        </Flex>
    );
};

export default Purge;
