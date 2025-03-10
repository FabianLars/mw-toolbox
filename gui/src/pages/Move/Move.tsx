import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { getCache, setCache } from '@/helpers/invoke';
import { errorToast, successToast } from '@/helpers/toast';
import { Button, Textarea } from '@/components';
import cls from './Move.module.css';

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
            .catch(errorToast)
            .finally(() => setIsLoading(false));
    };

    useEffect(() => setNavDisabled(isLoading), [isLoading, setNavDisabled]);

    useEffect(() => {
        getCache<string>('move-cache-from').then((cache) => {
            if (cache) setAreaFrom(cache);
        });
        getCache<string>('move-cache-to').then((cache) => {
            if (cache) setAreaTo(cache);
        });
    }, []);

    return (
        <div className={cls.container}>
            <div className={cls.fields}>
                <Textarea
                    className={cls.from}
                    label="pages to move"
                    value={areaFrom}
                    onChange={(event) => setAreaFrom(event.target.value)}
                    onBlur={() => setCache('move-cache-from', areaFrom)}
                    placeholder="Write exact names of pages to move. Separated by newline."
                />
                <Textarea
                    className={cls.to}
                    label="new names for pages"
                    value={areaTo}
                    onChange={(event) => setAreaTo(event.target.value)}
                    onBlur={() => setCache('move-cache-to', areaTo)}
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
