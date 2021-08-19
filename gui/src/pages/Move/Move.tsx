import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { errorToast, successToast } from '@/helpers/toast';
import { Button, Textarea } from '@/components';
import classes from './Move.module.css';

type Props = {
    isOnline: boolean;
    setNavDisabled: React.Dispatch<React.SetStateAction<boolean>>;
};

const Move = ({ isOnline, setNavDisabled }: Props) => {
    const [isLoading, setIsLoading] = useState(false);
    const [areaFrom, setAreaFrom] = useState('');
    const [areaTo, setAreaTo] = useState('');

    const movePages = () => {
        setIsLoading(true);
        invoke('rename', {
            from: areaFrom.split(/\r?\n/),
            to: areaTo.split(/\r?\n/),
        })
            .then(() => successToast('Successfully moved pages'))
            .catch((err) => errorToast(err))
            .finally(() => setIsLoading(false));
    };

    useEffect(() => setNavDisabled(isLoading), [isLoading]);

    useEffect(() => {
        invoke<string | null>('cache_get', { key: 'move-cache-from' }).then((cache) => {
            if (cache) setAreaFrom(cache);
        });
        invoke<string | null>('cache_get', { key: 'move-cache-to' }).then((cache) => {
            if (cache) setAreaTo(cache);
        });
    }, []);

    return (
        <div className={classes.container}>
            <div className={classes.fields}>
                <Textarea
                    className={classes.from}
                    label="pages to move"
                    value={areaFrom}
                    onChange={(event) => setAreaFrom(event.target.value)}
                    onBlur={() => invoke('cache_set', { key: 'move-cache-from', value: areaFrom })}
                    placeholder="Write exact names of pages to move. Separated by newline."
                />
                <Textarea
                    className={classes.to}
                    label="new names for pages"
                    value={areaTo}
                    onChange={(event) => setAreaTo(event.target.value)}
                    onBlur={() => invoke('cache_set', { key: 'move-cache-to', value: areaTo })}
                    placeholder="Write exact names of destinations. Separated by newline."
                />
            </div>
            <div>
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
            </div>
        </div>
    );
};

export default Move;
