import { useState } from 'react';
import { Button, Flex, Textarea, useToast } from '@chakra-ui/react';
import { invoke } from '@tauri-apps/api/dist/tauri';
import { Header } from '../../components';

const Purge = ({ isOnline }: { isOnline: boolean }) => {
    const [isPurging, setIsPurging] = useState(false);
    const [isNulling, setIsNulling] = useState(false);
    const [areaValue, setAreaValue] = useState('');
    const toast = useToast();

    const purgePages = (isNulledit: boolean) => {
        if (isNulledit) {
            setIsNulling(true);
        } else {
            setIsPurging(true);
        }
        invoke('purge', {
            pages: areaValue.split(/\r?\n/),
            is_nulledit: isNulledit,
        })
            .then(() =>
                toast({
                    title: (isNulledit ? 'Nulledit' : 'Purge') + ' successful',
                    description: (isNulledit ? 'Nulledit' : 'Purge') + ' successful',
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
            .finally(() => {
                setIsPurging(false);
                setIsNulling(false);
            });
    };

    return (
        <Flex direction="column" align="center" p="0 1rem 1rem" h="100vh">
            <Header isOnline={isOnline} isDisabled={isNulling || isPurging} />
            <Flex w="100%" h="100%" direction="column">
                <Textarea
                    resize="none"
                    value={areaValue}
                    onChange={event => setAreaValue(event.target.value)}
                    placeholder="Write exact page names here. Separated by newline."
                    h="100%"
                    mb={4}
                />
                <Flex direction="row" align="center" justify="center">
                    <Button
                        isLoading={isPurging}
                        isDisabled={!isOnline || isNulling}
                        onClick={() => purgePages(false)}
                        loadingText="Purging"
                        title={!isOnline ? 'Please login first!' : 'Clear server caches. This might take a while!'}
                        mx={2}
                    >
                        Purge all
                    </Button>
                    <Button
                        isLoading={isNulling}
                        isDisabled={!isOnline || isPurging}
                        onClick={() => purgePages(true)}
                        loadingText="Saving nulledits"
                        title={
                            !isOnline ? 'Please login first!' : 'Do a nulledit on every page. This might take a while!'
                        }
                        mx={2}
                    >
                        Nulledit all
                    </Button>
                </Flex>
            </Flex>
        </Flex>
    );
};

export default Purge;
