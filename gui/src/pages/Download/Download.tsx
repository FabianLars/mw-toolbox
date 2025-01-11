import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { getCache, setCache } from '@/helpers/invoke';
import { errorToast, successToast } from '@/helpers/toast';
import { Button, Textarea } from '@/components';
import cls from './Download.module.css';

type Props = {
    isOnline: boolean;
    setNavDisabled: React.Dispatch<React.SetStateAction<boolean>>;
};

const Download = ({ isOnline, setNavDisabled }: Props) => {
    const [areaValue, setAreaValue] = useState('');
    const [isLoading, setIsLoading] = useState(false);

    const downloadFiles = () => {
        setIsLoading(true);
        invoke('download', {
            files: areaValue.split(/\r?\n/),
        })
            .then(() => successToast('Download successful', 'Check your download folder.'))
            .catch(errorToast)
            .finally(() => setIsLoading(false));
    };

    useEffect(() => setNavDisabled(isLoading), [isLoading, setNavDisabled]);

    useEffect(() => {
        getCache<string>('download-cache').then((cache) => {
            if (cache) setAreaValue(cache);
        });
    }, []);

    return (
        <div className={cls.container}>
            <Textarea
                className={cls.area}
                label="file names to download, including the File: prefix"
                value={areaValue}
                onChange={(event) => setAreaValue(event.target.value)}
                onBlur={() => setCache('download-cache', areaValue)}
                placeholder="Write exact page names here, including the 'File:' prefix. Separated by newline. Saved in your download folder."
            />
            <div>
                <Button
                    isLoading={isLoading}
                    isDisabled={!isOnline || areaValue.trim() === ''}
                    onClick={downloadFiles}
                    loadingText="Downloading..."
                    title={!isOnline ? 'Please login first!' : 'This might take a while!'}
                >
                    Download all
                </Button>
            </div>
        </div>
    );
};

export default Download;
