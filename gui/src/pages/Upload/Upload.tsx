import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import FileList from './FileList';
import { emit, listen } from '@tauri-apps/api/event';
import { getCache, setCache } from '@/helpers/invoke';
import { errorToast, successToast } from '@/helpers/toast';
import { Button, Input, Label } from '@/components';
import cls from './Upload.module.css';

enum Action {
    None,
    Wait,
    Upload,
}

type Props = {
    isOnline: boolean;
    setNavDisabled: React.Dispatch<React.SetStateAction<boolean>>;
};

const Upload = ({ isOnline, setNavDisabled }: Props): JSX.Element => {
    const [status, setStatus] = useState(Action.None);
    const [uploadtext, setUploadtext] = useState('');
    const [files, setFiles] = useState<string[]>([]);

    const clearList = () => {
        setCache('files-cache', '');
        setFiles([]);
    };

    const openDialog = () => {
        setStatus(Action.Wait);
        open({ multiple: true, directory: false })
            .then((res) => {
                if (res) {
                    setFiles((oldFiles) => [...new Set([...oldFiles, ...res])]);
                }
            })
            .catch(errorToast)
            .finally(() => setStatus(Action.None));
    };

    const startUpload = () => {
        setStatus(Action.Upload);
        invoke('upload', {
            text: uploadtext,
            files,
        })
            .then(() => successToast('Upload complete'))
            .catch(errorToast)
            .finally(() => {
                setStatus(Action.None);
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

    useEffect(() => setNavDisabled(status !== Action.None), [status]);

    return (
        <div className={cls.container}>
            <div className={cls.controls}>
                <div className={cls.count}>
                    <div className={cls.label}>Number of files</div>
                    {files.length}
                </div>
                <div
                    title="No effect on existing pages"
                    id="uploadtext-input"
                    className={cls.uploadtext}
                >
                    <Label htmlFor="uploadtext-input">Text for new file pages</Label>
                    <Input
                        id="uploadtext-input"
                        value={uploadtext}
                        isDisabled={status !== Action.None}
                        onChange={(event) => setUploadtext(event.target.value)}
                        onBlur={() => setCache('uploadtext-cache', uploadtext)}
                    />
                </div>
                <div className={cls.buttons}>
                    <Button
                        className={cls.mx}
                        isLoading={status === Action.Wait}
                        isDisabled={status === Action.Upload}
                        onClick={openDialog}
                    >
                        Select File(s)
                    </Button>
                    <Button
                        className={cls.mx}
                        isLoading={status === Action.Wait}
                        isDisabled={status === Action.Upload}
                        onClick={clearList}
                    >
                        Clear Filelist
                    </Button>
                    <Button
                        className={cls.mx}
                        isDisabled={status === Action.Wait || !isOnline || !files[0]}
                        onClick={() => {
                            if (status === Action.Upload) {
                                emit('cancel-upload').finally(() => setStatus(Action.Wait));
                            } else {
                                startUpload();
                            }
                        }}
                        title={!isOnline ? 'Please login first!' : 'This might take a while!'}
                    >
                        {status === Action.Upload ? 'Cancel' : 'Upload'}
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
                        className={cls.entry}
                        aria-label="click to remove item"
                        title="click to remove item"
                        onClick={() => {
                            if (status === Action.None) {
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
