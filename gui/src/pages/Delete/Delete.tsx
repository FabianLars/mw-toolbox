import { Box, Button, Flex, useToast } from '@chakra-ui/react';
import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Input, Label, Textarea } from '@/components';
import { errorToast, successToast } from '@/helpers/toast';
import classes from './Delete.module.css';

type Props = {
    isOnline: boolean;
    setNavDisabled: React.Dispatch<React.SetStateAction<boolean>>;
};

const Delete = ({ isOnline, setNavDisabled }: Props) => {
    const [areaValue, setAreaValue] = useState('');
    const [isLoading, setIsLoading] = useState(false);
    const [reason, setReason] = useState('');
    const toast = useToast();

    const deletePages = () => {
        setIsLoading(true);
        invoke('delete', {
            pages: areaValue.split(/\r?\n/),
            reason,
        })
            .then(() => toast(successToast('Delete successful')))
            .catch((err) => toast(errorToast(err)))
            .finally(() => setIsLoading(false));
    };

    useEffect(() => setNavDisabled(isLoading), [isLoading]);

    useEffect(() => {
        (invoke('cache_get', { key: 'delete-reason' }) as Promise<string | null>).then((cache) => {
            if (cache) setReason(cache);
        });
        (invoke('cache_get', { key: 'delete-pages' }) as Promise<string | null>).then((cache) => {
            if (cache) setAreaValue(cache);
        });
    }, []);

    return (
        <Flex direction="column" align="center" w="100%" h="100%">
            <div className={classes.input}>
                <Label htmlFor="delete-reason">Delete reason</Label>
                <Input
                    id="delete-reason"
                    value={reason}
                    onChange={(event) => setReason(event.target.value)}
                    onBlur={() => invoke('cache_set', { key: 'delete-reaseon', value: reason })}
                />
            </div>
            <Textarea
                className={classes.area}
                label="pages to delete"
                value={areaValue}
                onChange={(event) => setAreaValue(event.target.value)}
                onBlur={() => invoke('cache_set', { key: 'delete-pages', value: areaValue })}
                placeholder="Write exact page names here. Separated by newline."
            ></Textarea>
            <Box>
                <Button
                    isLoading={isLoading}
                    isDisabled={!isOnline || areaValue.trim() === ''}
                    onClick={deletePages}
                    loadingText="Deleting..."
                    title={!isOnline ? 'Please login first!' : 'This might take a while!'}
                >
                    Delete all
                </Button>
            </Box>
        </Flex>
    );
};

export default Delete;
