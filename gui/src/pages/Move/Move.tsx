import { Box, Button, Flex, Textarea, useToast } from '@chakra-ui/react';
import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { errorToast, successToast } from '../../helpers/toast';

type Props = {
    isOnline: boolean;
    setNavDisabled: React.Dispatch<React.SetStateAction<boolean>>;
};

const Move = ({ isOnline, setNavDisabled }: Props) => {
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
            .then(() => toast(successToast('Successfully moved pages')))
            .catch((err) => toast(errorToast(err)))
            .finally(() => setIsLoading(false));
    };

    useEffect(() => setNavDisabled(isLoading), [isLoading]);

    return (
        <Flex direction="column" align="center" h="100%" w="100%">
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
                    isDisabled={
                        !isOnline ||
                        areaFrom.trim() === '' ||
                        areaTo.trim() === '' ||
                        areaFrom.split(/\r\n|\r|\n/).length !== areaTo.split(/\r\n|\r|\n/).length
                    }
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
