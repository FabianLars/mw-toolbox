import { Box, Button, Flex, FormControl, FormLabel, Input, useToast } from '@chakra-ui/react';
import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import FileList from './FileList';
import { emit, listen } from '@tauri-apps/api/event';
import { errorToast, successToast } from '../../helpers/toast';

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
        <Flex direction="column" align="center" h="100%" w="100%">
            <Flex direction={['column', null, 'row']} align="center" w="100%" mb={4}>
                <Flex
                    mx={2}
                    pb={[null, null, 2]}
                    flex="1 0 auto"
                    direction="column"
                    justify="space-between"
                    align="center"
                    h={[null, null, '100%']}
                    pr={[null, null, 4]}
                    borderRight={[null, null, '1px solid rgba(255, 255, 255, 0.16)']}
                >
                    <Box fontWeight={500}>Number of files</Box>
                    {files.length}
                </Flex>
                <FormControl
                    title="No effect on existing pages"
                    id="uploadtext-input"
                    mx={2}
                    flex="1 1 auto"
                >
                    <FormLabel>Text for new file pages</FormLabel>
                    <Input
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
                </FormControl>
                <Box flex="1 0 auto" alignSelf="flex-end" mt={4}>
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
                        mx={2}
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
                </Box>
            </Flex>
            <FileList placeholder="Selected files will be displayed here.">
                {files.map((f) => (
                    <Box
                        key={f}
                        aria-label="click to remove item"
                        title="click to remove item"
                        cursor="pointer"
                        _hover={{
                            color: 'red',
                            backgroundColor: 'rgba(0, 0, 0, 0.1)',
                        }}
                        onClick={() => {
                            if (!isUploading) {
                                setFiles((oldFiles) => oldFiles.filter((ff) => ff !== f));
                            }
                        }}
                    >
                        {f}
                    </Box>
                ))}
            </FileList>
        </Flex>
    );
};

export default Upload;
