import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { getCache, setCache } from '@/helpers/invoke';
import { errorToast, successToast } from '@/helpers/toast';
import { Button, Textarea } from '@/components';
import classes from './Purge.module.css';

type Props = {
    isOnline: boolean;
    setNavDisabled: React.Dispatch<React.SetStateAction<boolean>>;
};

const Purge = ({ isOnline, setNavDisabled }: Props) => {
    const [isPurging, setIsPurging] = useState(false);
    const [isNulling, setIsNulling] = useState(false);
    const [areaValue, setAreaValue] = useState('');

    const purgePages = (isNulledit: boolean) => {
        if (isNulledit) {
            setIsNulling(true);
        } else {
            setIsPurging(true);
        }
        invoke('purge', {
            pages: areaValue.split(/\r?\n/),
            isNulledit,
        })
            .then(() => successToast((isNulledit ? 'Nulledit' : 'Purge') + ' successful'))
            .catch((err) => errorToast(err))
            .finally(() => {
                setIsPurging(false);
                setIsNulling(false);
            });
    };

    useEffect(() => setNavDisabled(isNulling || isPurging), [isNulling, isPurging]);

    useEffect(() => {
        getCache<string>('purge-cache').then((cache) => {
            if (cache) setAreaValue(cache);
        });
    }, []);

    return (
        <div className={classes.container}>
            <Textarea
                className={classes.area}
                label="pages to purge"
                value={areaValue}
                onChange={(event) => setAreaValue(event.target.value)}
                onBlur={() => setCache('purge-cache', areaValue)}
                placeholder="Write exact page names here. Separated by newline."
            />
            <div className={classes.buttons}>
                <Button
                    isLoading={isPurging}
                    isDisabled={!isOnline || isNulling || areaValue.trim() === ''}
                    onClick={() => purgePages(false)}
                    loadingText="Purging"
                    title={
                        !isOnline
                            ? 'Please login first!'
                            : 'Clear server caches. This might take a while!'
                    }
                    className={classes.mx}
                >
                    Purge all
                </Button>
                <Button
                    isLoading={isNulling}
                    isDisabled={!isOnline || isPurging || areaValue.trim() === ''}
                    onClick={() => purgePages(true)}
                    loadingText="Saving nulledits"
                    title={
                        !isOnline
                            ? 'Please login first!'
                            : 'Do a nulledit on every page. This might take a while!'
                    }
                    className={classes.mx}
                >
                    Nulledit all
                </Button>
            </div>
        </div>
    );
};

export default Purge;
