import { Button, Flex, FormControl, FormLabel, Input, Textarea, useToast } from '@chakra-ui/react';
import React, { useEffect, useState } from 'react';
import { promisified } from 'tauri/api/tauri';
import { emit } from 'tauri/api/event';
import Header from '../components/Header';

const Upload = ({ isOnline }) => {
    const [isUploading, setIsUploading] = useState(false);
    const [isWaiting, setIsWaiting] = useState(false);
    const [uploadtext, setUploadtext] = useState('');
    const [files, setFiles] = useState('');
    const toast = useToast();

    const clearList = () => {
        emit('clear-files');
        window.sessionStorage.setItem('files-cache', '');
        setFiles('');
    };

    const openDialog = () => {
        setIsWaiting(true);
        promisified({
            cmd: 'uploadDialog',
        })
            .then((res) => {
                const files = res.join('\n');
                setFiles(files);
                window.sessionStorage.setItem('uploadtext-cache', uploadtext);
                window.sessionStorage.setItem('files-cache', files);
                setIsWaiting(false);
            })
            .catch((err) => {
                setIsWaiting(false);
                toast({
                    title: 'Something went wrong!',
                    description: err,
                    status: 'error',
                    duration: 5000,
                    isClosable: true,
                });
            });
    };

    const startUpload = () => {
        setIsUploading(true);
        promisified({
            cmd: 'upload',
            text: uploadtext,
        })
            .then(() => {
                setIsUploading(false);
                toast({
                    title: 'Upload complete!',
                    description: 'Upload complete!',
                    status: 'success',
                    isClosable: true,
                });
            })
            .catch((err) => {
                setIsWaiting(false);
                toast({
                    title: 'Something went wrong!',
                    description: err,
                    status: 'error',
                    duration: 10000,
                    isClosable: true,
                });
            });
    };

    useEffect(() => {
        setFiles(window.sessionStorage.getItem('files-cache') ?? '');
        setUploadtext(window.sessionStorage.getItem('uploadtext-cache') ?? '');
    }, []);

    return (
        <Flex direction="column" align="center" p="0 1rem 1rem" h="100vh">
            <Header isDisabled={isWaiting || isUploading} isOnline={isOnline} />
            <Flex direction="row" justify="center" align="center" w="75%" mb={4}>
                <FormControl id="uploadtext-input" mx={2}>
                    <FormLabel>Text for newly created file pages</FormLabel>
                    <Input
                        value={uploadtext}
                        isDisabled={isUploading || isWaiting}
                        onChange={(event) => setUploadtext(event.target.value)}
                    />
                </FormControl>
                <Button mx={2} isLoading={isWaiting} isDisabled={isUploading} onClick={openDialog}>
                    Select File(s)
                </Button>
                <Button mx={2} isLoading={isWaiting} isDisabled={isUploading} onClick={clearList}>
                    Clear Filelist
                </Button>
                <Button
                    mx={2}
                    isDisabled={isWaiting || !isOnline}
                    isLoading={isUploading}
                    onClick={startUpload}
                    title={!isOnline ? 'Please login first!' : 'This might take a while!'}
                >
                    Upload
                </Button>
            </Flex>
            <Textarea
                resize="none"
                value={files}
                isReadOnly
                placeholder="Selected files will be displayed here."
                h="100%"
            />
        </Flex>
    );
};

export default Upload;
