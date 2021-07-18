import { Box, Button, Flex, useToast } from '@chakra-ui/react';
import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { errorToast, successToast } from '@/helpers/toast';
import { Textarea } from '@/components';
import classes from './Download.module.css';

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
            .then(() => toast(successToast('Download successful', 'Check your download folder.')))
            .catch((err) => toast(errorToast(err)))
            .finally(() => setIsLoading(false));
    };

    useEffect(() => setNavDisabled(isLoading), [isLoading]);

    useEffect(() => {
        (invoke('cache_get', { key: 'download-cache' }) as Promise<string | null>).then((cache) => {
            if (cache) setAreaValue(cache);
        });
    }, []);

    return (
        <Flex direction="column" align="center" h="100%" w="100%">
            <Textarea
                className={classes.area}
                label="file names to download, including the File: prefix"
                value={areaValue}
                onChange={(event) => setAreaValue(event.target.value)}
                onBlur={() => invoke('cache_set', { key: 'download-cache', value: areaValue })}
                placeholder="Write exact page names here. Separated by newline. Inclusive 'File:' Prefix. Saved in your download folder."
            />
            <Box>
                <Button
                    isLoading={isLoading}
                    isDisabled={!isOnline || areaValue.trim() === ''}
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
