import { Button, useToast } from '@chakra-ui/react';
import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import FileList from './FileList';
import { emit, listen } from '@tauri-apps/api/event';
import { errorToast, successToast } from '@/helpers/toast';
import { Input, Label } from '@/components';
import classes from './Upload.module.css';

type Props = {
    isOnline: boolean;
    setNavDisabled: React.Dispatch<React.SetStateAction<boolean>>;
};

const Upload = ({ isOnline, setNavDisabled }: Props) => {
    const [isUploading, setIsUploading] = useState(false);
    const [isWaiting, setIsWaiting] = useState(false);
    const [uploadtext, setUploadtext] = useState('');
    const [files, setFiles] = useState<string[]>([]);
    const toast = useToast();

    const clearList = () => {
        invoke('cache_set', { key: 'files-cache', value: '' });
        setFiles([]);
    };

    const openDialog = () => {
        setIsWaiting(true);
        (open({ multiple: true, directory: false }) as Promise<string[]>)
            .then((res) => {
                if (res) {
                    setFiles((oldFiles) => [...new Set([...oldFiles, ...res])]);
                }
            })
            .catch((err) => {
                toast(errorToast(err));
            })
            .finally(() => setIsWaiting(false));
    };

    const startUpload = () => {
        setIsUploading(true);
        (
            invoke('upload', {
                text: uploadtext,
                files,
            }) as Promise<null>
        )
            .then(() => toast(successToast('Upload complete')))
            .catch((err) => toast(errorToast(err)))
            .finally(() => {
                setIsWaiting(false);
                setIsUploading(false);
            });
    };

    useEffect(() => {
        const unlistenUploaded = listen('file-uploaded', ({ payload }) => {
            setFiles((oldFiles) => oldFiles.filter((f) => f !== payload));
        });
        (invoke('cache_get', { key: 'files-cache' }) as Promise<string[] | null>).then((res) =>
            setFiles(res ?? []),
        );
        (invoke('cache_get', { key: 'uploadtext-cache' }) as Promise<string | null>).then((res) =>
            setUploadtext(res ?? ''),
        );

        return () => {
            unlistenUploaded.then((f) => f());
        };
    }, []);

    // componentWillUnmount with files state
    useEffect(() => {
        return () => {
            invoke('cache_set', { key: 'files-cache', value: files });
        };
    }, [files]);

    useEffect(() => setNavDisabled(isUploading || isWaiting), [isUploading, isWaiting]);

    return (
        <div className={classes.container}>
            <div className={classes.controls}>
                <div className={classes.count}>
                    <div className={classes.label}>Number of files</div>
                    {files.length}
                </div>
                <div
                    title="No effect on existing pages"
                    id="uploadtext-input"
                    className={classes.uploadtext}
                >
                    <Label htmlFor="uploadtext-input">Text for new file pages</Label>
                    <Input
                        id="uploadtext-input"
                        value={uploadtext}
                        isDisabled={isUploading || isWaiting}
                        onChange={(event) => setUploadtext(event.target.value)}
                        onBlur={() =>
                            invoke('cache_set', {
                                key: 'uploadtext-cache',
                                value: uploadtext,
                            })
                        }
                    />
                </div>
                <div className={classes.buttons}>
                    <Button
                        mx={2}
                        isLoading={isWaiting}
                        isDisabled={isUploading}
                        onClick={openDialog}
                    >
                        Select File(s)
                    </Button>
                    <Button
                        mx={2}
                        isLoading={isWaiting}
                        isDisabled={isUploading}
                        onClick={clearList}
                    >
                        Clear Filelist
                    </Button>
                    <Button
                        ml={2}
                        isDisabled={isWaiting || !isOnline || !files[0]}
                        onClick={() => {
                            if (isUploading) {
                                emit('cancel-upload').finally(() => setIsWaiting(true));
                            } else {
                                startUpload();
                            }
                        }}
                        title={!isOnline ? 'Please login first!' : 'This might take a while!'}
                    >
                        {isUploading ? 'Cancel' : 'Upload'}
                    </Button>
                </div>
            </div>
            <FileList placeholder="Selected files will be displayed here.">
                {files.map((f) => (
                    <div
                        key={f}
                        className={classes.entry}
                        aria-label="click to remove item"
                        title="click to remove item"
                        onClick={() => {
                            if (!isUploading) {
                                setFiles((oldFiles) => oldFiles.filter((ff) => ff !== f));
                            }
                        }}
                    >
                        {f}
                    </div>
                ))}
            </FileList>
        </div>
    );
};

export default Upload;
