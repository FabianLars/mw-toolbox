import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import FileList from './FileList';
import { emit, listen } from '@tauri-apps/api/event';
import { getCache, setCache } from '@/helpers/invoke';
import { errorToast, successToast } from '@/helpers/toast';
import { Button, Input, Label } from '@/components';
import classes from './Upload.module.css';

type Props = {
    isOnline: boolean;
    setNavDisabled: React.Dispatch<React.SetStateAction<boolean>>;
};

const Upload = ({ isOnline, setNavDisabled }: Props): JSX.Element => {
    const [isUploading, setIsUploading] = useState(false);
    const [isWaiting, setIsWaiting] = useState(false);
    const [uploadtext, setUploadtext] = useState('');
    const [files, setFiles] = useState<string[]>([]);

    const clearList = () => {
        setCache('files-cache', '');
        setFiles([]);
    };

    const openDialog = () => {
        setIsWaiting(true);
        open({ multiple: true, directory: false })
            .then((res) => {
                if (res) {
                    setFiles((oldFiles) => [...new Set([...oldFiles, ...res])]);
                }
            })
            .catch(errorToast)
            .finally(() => setIsWaiting(false));
    };

    const startUpload = () => {
        setIsUploading(true);
        invoke('upload', {
            text: uploadtext,
            files,
        })
            .then(() => successToast('Upload complete'))
            .catch(errorToast)
            .finally(() => {
                setIsWaiting(false);
                setIsUploading(false);
            });
    };

    useEffect(() => {
        const unlistenUploaded = listen('file-uploaded', ({ payload }) => {
            setFiles((oldFiles) => oldFiles.filter((f) => f !== payload));
        });
        const unlistenFileDrop = listen('tauri://file-drop', (res: { payload: string[] }) => {
            if (res.payload[0]) {
                setFiles((oldFiles) => [...new Set([...oldFiles, ...res.payload])]);
            }
        });
        getCache<string[]>('files-cache').then((res) => setFiles(res ?? []));
        getCache<string>('uploadtext-cache').then((res) => setUploadtext(res ?? ''));

        return () => {
            unlistenUploaded.then((f) => f());
            unlistenFileDrop.then((f) => f());
        };
    }, []);

    // componentWillUnmount with files state
    useEffect(() => {
        return () => {
            setCache('files-cache', files);
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
                        onBlur={() => setCache('uploadtext-cache', uploadtext)}
                    />
                </div>
                <div className={classes.buttons}>
                    <Button
                        className={classes.mx}
                        isLoading={isWaiting}
                        isDisabled={isUploading}
                        onClick={openDialog}
                    >
                        Select File(s)
                    </Button>
                    <Button
                        className={classes.mx}
                        isLoading={isWaiting}
                        isDisabled={isUploading}
                        onClick={clearList}
                    >
                        Clear Filelist
                    </Button>
                    <Button
                        className={classes.mx}
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
            <FileList
                placeholder="Selected files will be displayed here.
            You can also drop files on here to add them to the list.
            Paths resolving to folders will be skipped without an error."
            >
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
