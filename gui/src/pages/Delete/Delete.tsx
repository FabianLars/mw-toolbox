import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Button, Input, Label, Textarea } from '@/components';
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

    const deletePages = () => {
        setIsLoading(true);
        invoke('delete', {
            pages: areaValue.split(/\r?\n/),
            reason,
        })
            .then(() => successToast('Delete successful'))
            .catch((err) => errorToast(err))
            .finally(() => setIsLoading(false));
    };

    useEffect(() => setNavDisabled(isLoading), [isLoading]);

    useEffect(() => {
        invoke<string | null>('cache_get', { key: 'delete-reason' }).then((cache) => {
            if (cache) setReason(cache);
        });
        invoke<string | null>('cache_get', { key: 'delete-pages' }).then((cache) => {
            if (cache) setAreaValue(cache);
        });
    }, []);

    return (
        <div className={classes.container}>
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
            <div>
                <Button
                    isLoading={isLoading}
                    isDisabled={!isOnline || areaValue.trim() === ''}
                    onClick={deletePages}
                    loadingText="Deleting..."
                    title={!isOnline ? 'Please login first!' : 'This might take a while!'}
                >
                    Delete all
                </Button>
            </div>
        </div>
    );
};

export default Delete;
