import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Button, Input, Label, Textarea } from '@/components';
import { getCache, setCache } from '@/helpers/invoke';
import { errorToast, successToast } from '@/helpers/toast';
import cls from './Delete.module.css';

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
            .catch(errorToast)
            .finally(() => setIsLoading(false));
    };

    useEffect(() => setNavDisabled(isLoading), [isLoading, setNavDisabled]);

    useEffect(() => {
        getCache<string>('delete-reason').then((cache) => {
            if (cache) setReason(cache);
        });
        getCache<string>('delete-pages').then((cache) => {
            if (cache) setAreaValue(cache);
        });
    }, []);

    return (
        <div className={cls.container}>
            <div className={cls.input}>
                <Label htmlFor="delete-reason">Delete reason</Label>
                <Input
                    id="delete-reason"
                    value={reason}
                    onChange={(event) => setReason(event.target.value)}
                    onBlur={() => setCache('delete-reaseon', reason)}
                />
            </div>
            <Textarea
                className={cls.area}
                label="pages to delete"
                value={areaValue}
                onChange={(event) => setAreaValue(event.target.value)}
                onBlur={() => setCache('delete-pages', areaValue)}
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
