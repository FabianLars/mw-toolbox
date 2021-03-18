import { Box, Button, Flex, Textarea, useToast } from '@chakra-ui/react';
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Header } from '../../components';

const Delete = ({ isOnline }: { isOnline: boolean }) => {
    const [areaValue, setAreaValue] = useState('');
    const [isLoading, setIsLoading] = useState(false);
    const toast = useToast();

    const deletePages = () => {
        setIsLoading(true);
        invoke('delete', {
            pages: areaValue.split(/\r?\n/),
        })
            .then(() =>
                toast({
                    title: 'Delete successful',
                    description: 'Delete successful',
                    status: 'success',
                    isClosable: true,
                })
            )
            .catch(err =>
                toast({
                    title: 'Something went wrong!',
                    description: err,
                    status: 'error',
                    duration: 10000,
                    isClosable: true,
                })
            )
            .finally(() => setIsLoading(false));
    };

    return (
        <Flex direction="column" align="center" p="0 1rem 1rem" h="100vh">
            <Header isOnline={isOnline} isDisabled={isLoading} />
            <Textarea
                resize="none"
                value={areaValue}
                onChange={event => setAreaValue(event.target.value)}
                placeholder="Write exact page names here. Separated by newline."
                h="100%"
                mb={4}
            />
            <Box>
                <Button
                    isLoading={isLoading}
                    isDisabled={!isOnline}
                    onClick={deletePages}
                    loadingText="Deleting..."
                    title={!isOnline ? 'Please login first!' : 'This might take a while!'}
                >
                    Delete all
                </Button>
            </Box>
        </Flex>
    );
};

export default Delete;
