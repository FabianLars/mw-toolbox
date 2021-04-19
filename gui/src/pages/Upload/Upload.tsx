import { Box, Button, Flex, FormControl, FormLabel, Input, Textarea, useToast } from '@chakra-ui/react';
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { emit } from '@tauri-apps/api/event';
import { Header } from '../../components';

const Upload = ({ isOnline }: { isOnline: boolean }) => {
    const [isUploading, setIsUploading] = useState(false);
    const [isWaiting, setIsWaiting] = useState(false);
    const [uploadtext, setUploadtext] = useState('');
    const [files, setFiles] = useState('');
    const toast = useToast();

    const clearList = () => {
        emit('clear-files').catch(console.error);
        invoke('cacheSet', { key: 'files-cache', value: '' }).catch(console.error);
        setFiles('');
    };

    const openDialog = () => {
        setIsWaiting(true);
        (invoke('uploadDialog') as Promise<string[]>)
            .then(res => {
                const files = res.join('\n');
                setFiles(files);
            })
            .catch(err =>
                toast({
                    title: 'Something went wrong!',
                    description: err.Err,
                    status: 'error',
                    duration: 5000,
                    isClosable: true,
                })
            )
            .finally(() => setIsWaiting(false));
    };

    const startUpload = () => {
        setIsUploading(true);
        (invoke('upload', {
            text: uploadtext,
        }) as Promise<any>)
            .then(() =>
                toast({
                    title: 'Upload complete!',
                    description: 'Upload complete!',
                    status: 'success',
                    isClosable: true,
                })
            )
            .catch(err =>
                toast({
                    title: 'Something went wrong!',
                    description: err.Err,
                    status: 'error',
                    duration: 10000,
                    isClosable: true,
                })
            )
            .finally(() => setIsUploading(false));
    };

    useEffect(() => {
        (invoke('cacheGet', { key: 'files-cache' }) as Promise<string>).then(setFiles);
        (invoke('cacheGet', { key: 'uploadtext-cache' }) as Promise<string>).then(setUploadtext);
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
                        onChange={event => setUploadtext(event.target.value)}
                        onBlur={() => invoke('cacheSet', { key: 'uploadtext-cache', value: uploadtext })}
                    />
                </FormControl>
                <Box>
                    <Button mx={2} isLoading={isWaiting} isDisabled={isUploading} onClick={openDialog}>
                        Select File(s)
                    </Button>
                </Box>
                <Box>
                    <Button mx={2} isLoading={isWaiting} isDisabled={isUploading} onClick={clearList}>
                        Clear Filelist
                    </Button>
                </Box>
                <Box>
                    <Button
                        mx={2}
                        isDisabled={isWaiting || !isOnline}
                        isLoading={isUploading}
                        onClick={startUpload}
                        title={!isOnline ? 'Please login first!' : 'This might take a while!'}
                    >
                        Upload
                    </Button>
                </Box>
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
