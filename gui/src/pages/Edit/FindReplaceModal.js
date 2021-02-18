import {
    Button,
    Flex,
    Checkbox,
    Link,
    Input,
    Modal,
    ModalBody,
    ModalOverlay,
    ModalContent,
    ModalHeader,
    ModalFooter,
    Spacer,
    IconButton,
} from '@chakra-ui/react';
import { InfoOutlineIcon } from '@chakra-ui/icons';
import { useState } from 'react';

const FindReplaceModal = ({ isOpen, onClose, patterns, setPatterns }) => {
    const [localPatterns, setLocalPatterns] = useState([{}]);

    const onModalClose = () => {
        const arr = patterns.map(obj => Object.assign({}, obj));
        setLocalPatterns(arr);
        onClose();
    };

    const onModalSave = () => {
        const arr = localPatterns.map(obj => Object.assign({}, obj));
        setPatterns(arr);
        onClose();
    };

    return (
        <Modal onClose={onModalClose} isOpen={isOpen} isCentered size="xl">
            <ModalOverlay />
            <ModalContent>
                <ModalHeader>Find & Replace</ModalHeader>
                <ModalBody>
                    <Flex direction="column" h="100%" w="100%">
                        {patterns.map((_, index) => (
                            <Flex key={index} align="center">
                                <Input
                                    m={1}
                                    placeholder="Find"
                                    value={localPatterns[index]['find'] || ''}
                                    onKeyDown={e => {
                                        if (e.key === 'Enter') onModalSave();
                                    }}
                                    onChange={event =>
                                        setLocalPatterns(oldArr => {
                                            const values = [...oldArr];
                                            values[index]['find'] = event.target.value;
                                            return values;
                                        })
                                    }
                                />
                                <Input
                                    m={1}
                                    placeholder="Replace"
                                    value={localPatterns[index]['replace'] || ''}
                                    onKeyDown={e => {
                                        if (e.key === 'Enter') onModalSave();
                                    }}
                                    onChange={event =>
                                        setLocalPatterns(oldArr => {
                                            const values = [...oldArr];
                                            values[index]['replace'] = event.target.value;
                                            return values;
                                        })
                                    }
                                />
                                <Checkbox
                                    verticalAlign="center"
                                    m={1}
                                    isChecked={localPatterns[index]['isRegex']}
                                    onChange={event =>
                                        setLocalPatterns(oldArr => {
                                            const values = [...oldArr];
                                            values[index]['isRegex'] = event.target.checked;
                                            return values;
                                        })
                                    }
                                >
                                    Regex
                                </Checkbox>
                                <Link href="https://docs.rs/regex/" isExternal title="Open Regex Documentation">
                                    <IconButton
                                        mt={2}
                                        arial-label="Infos about Regular Expressions"
                                        icon={<InfoOutlineIcon />}
                                        variant="link"
                                    />
                                </Link>
                            </Flex>
                        ))}
                    </Flex>
                </ModalBody>
                <ModalFooter>
                    <Button
                        mr={2}
                        onClick={() => {
                            if (localPatterns.length < 10) setLocalPatterns(old => old.concat({}));
                        }}
                        isDisabled={localPatterns.length >= 10}
                    >
                        Add Row
                    </Button>
                    <Button colorScheme="red" title="Press 'Save' to apply." onClick={() => setLocalPatterns([{}])}>
                        Clear all
                    </Button>
                    <Spacer />
                    <Button colorScheme="blue" mr={2} onClick={onModalSave}>
                        Save
                    </Button>
                    <Button onClick={onModalClose}>Cancel</Button>
                </ModalFooter>
            </ModalContent>
        </Modal>
    );
};

export default FindReplaceModal;
