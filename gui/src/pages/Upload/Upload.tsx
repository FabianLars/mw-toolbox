import {
    Box,
    Button,
    Flex,
    FormControl,
    FormLabel,
    Input,
    Spacer,
    useToast,
} from '@chakra-ui/react';
import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import { Header, Output } from '../../components';
import { emit, listen } from '@tauri-apps/api/event';

const Upload = ({ isOnline }: { isOnline: boolean }) => {
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
                toast({
                    title: 'Something went wrong!',
                    description: err,
                    status: 'error',
                    duration: 5000,
                    isClosable: true,
                });
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
            .then(() =>
                toast({
                    title: 'Upload complete!',
                    description: 'Upload complete!',
                    status: 'success',
                    isClosable: true,
                }),
            )
            .catch((err) =>
                toast({
                    title: 'Something went wrong!',
                    description: err,
                    status: 'error',
                    duration: 10000,
                    isClosable: true,
                }),
            )
            .finally(() => {
                setIsWaiting(false);
                setIsUploading(false);
            });
    };

    useEffect(() => {
        listen('file-uploaded', ({ payload }) => {
            setFiles((oldFiles) => oldFiles.filter((f) => f !== payload));
        });
        (invoke('cache_get', { key: 'files-cache' }) as Promise<string[] | null>).then((res) =>
            setFiles(res ?? []),
        );
        (invoke('cache_get', { key: 'uploadtext-cache' }) as Promise<string | null>).then((res) =>
            setUploadtext(res ?? ''),
        );
    }, []);

    // componentWillUnmount with files state
    useEffect(() => {
        return () => {
            invoke('cache_set', { key: 'files-cache', value: files });
        };
    }, [files]);

    return (
        <Flex direction="column" align="center" p="0 1rem 1rem" h="100vh" userSelect="none">
            <Header isDisabled={isWaiting || isUploading} isOnline={isOnline} />
            <Flex direction="row" align="center" w="100%" mb={4}>
                <Box>Number of files: {files.length}</Box>
                <Spacer />
                <FormControl id="uploadtext-input" mx={2} maxW="50%">
                    <FormLabel>Text for newly created file pages</FormLabel>
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
                <Box>
                    <Button
                        mx={2}
                        isLoading={isWaiting}
                        isDisabled={isUploading}
                        onClick={openDialog}
                    >
                        Select File(s)
                    </Button>
                </Box>
                <Box>
                    <Button
                        mx={2}
                        isLoading={isWaiting}
                        isDisabled={isUploading}
                        onClick={clearList}
                    >
                        Clear Filelist
                    </Button>
                </Box>
                <Box>
                    <Button
                        mx={2}
                        isDisabled={isWaiting || !isOnline || !files}
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
            <Output placeholder="Selected files will be displayed here.">
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
            </Output>
        </Flex>
    );
};

export default Upload;
