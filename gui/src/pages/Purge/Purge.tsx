import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { getCache, setCache } from '@/helpers/invoke';
import { errorToast, successToast } from '@/helpers/toast';
import { Button, Textarea } from '@/components';
import cls from './Purge.module.css';

enum Action {
    None,
    Purge,
    Null,
}

type Props = {
    isOnline: boolean;
    setNavDisabled: React.Dispatch<React.SetStateAction<boolean>>;
};

const Purge = ({ isOnline, setNavDisabled }: Props) => {
    const [status, setStatus] = useState(Action.None);
    const [areaValue, setAreaValue] = useState('');

    const purgePages = (isNulledit: boolean) => {
        setNavDisabled(true);
        setStatus(isNulledit ? Action.Null : Action.Purge);

        invoke('purge', {
            pages: areaValue.split(/\r?\n/),
            isNulledit,
        })
            .then(() => successToast((isNulledit ? 'Nulledit' : 'Purge') + ' successful'))
            .catch(errorToast)
            .finally(() => {
                setNavDisabled(false);
                setStatus(Action.None);
            });
    };

    useEffect(() => {
        getCache<string>('purge-cache').then((cache) => {
            if (cache) setAreaValue(cache);
        });
    }, []);

    return (
        <div className={cls.container}>
            <Textarea
                className={cls.area}
                label="pages to purge"
                value={areaValue}
                onChange={(event) => setAreaValue(event.target.value)}
                onBlur={() => setCache('purge-cache', areaValue)}
                placeholder="Write exact page names here. Separated by newline."
            />
            <div className={cls.buttons}>
                <Button
                    isLoading={status === Action.Purge}
                    isDisabled={!isOnline || status === Action.Null || areaValue.trim() === ''}
                    onClick={() => purgePages(false)}
                    loadingText="Purging"
                    title={
                        !isOnline
                            ? 'Please login first!'
                            : 'Clear server caches. This might take a while!'
                    }
                    className={cls.mx}
                >
                    Purge all
                </Button>
                <Button
                    isLoading={status === Action.Null}
                    isDisabled={!isOnline || status === Action.Purge || areaValue.trim() === ''}
                    onClick={() => purgePages(true)}
                    loadingText="Saving nulledits"
                    title={
                        !isOnline
                            ? 'Please login first!'
                            : 'Do a nulledit on every page. This might take a while!'
                    }
                    className={cls.mx}
                >
                    Nulledit all
                </Button>
            </div>
        </div>
    );
};

export default Purge;
