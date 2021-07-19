import {
    Button,
    Modal,
    ModalBody,
    ModalContent,
    ModalFooter,
    ModalHeader,
    ModalOverlay,
} from '@chakra-ui/react';
import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import classes from './FindReplaceModal.module.css';
import { Checkbox, Input } from '@/components';

type Pattern = {
    find: string;
    replace: string;
    isRegex: boolean;
};

type Props = {
    isOpen: boolean;
    onClose: () => void;
    patterns: Pattern[];
    setPatterns: React.Dispatch<React.SetStateAction<Pattern[]>>;
    initialRef?: React.RefObject<HTMLButtonElement>;
};

const FindReplaceModal = ({ isOpen, onClose, patterns, setPatterns, initialRef }: Props) => {
    const [localPatterns, setLocalPatterns] = useState<Pattern[]>([]);

    const onModalClose = () => {
        const arr = patterns.map((obj) => Object.assign({}, obj));
        setLocalPatterns(arr);
        onClose();
    };

    const onModalSave = () => {
        const arr = localPatterns.map((obj) => Object.assign({}, obj));
        setPatterns(arr);
        invoke('cache_set', { key: 'edit-patterns', value: arr }).finally(onClose);
    };

    useEffect(() => {
        setLocalPatterns(patterns);
    }, [patterns]);

    return (
        <Modal
            onClose={onModalClose}
            isOpen={isOpen}
            isCentered
            size="xl"
            initialFocusRef={initialRef}
        >
            <ModalOverlay />
            <ModalContent>
                <ModalHeader>
                    Find & Replace
                    <div className={classes.regexinfo}>
                        <a
                            target="_blank"
                            rel="noopener noreferrer"
                            className={classes.link}
                            href="https://docs.rs/regex/"
                            title="Open Regex Documentation"
                        >
                            Click here for the regex docs
                        </a>
                    </div>
                </ModalHeader>
                <ModalBody>
                    <div className={classes.container}>
                        {localPatterns.map((_, index) => (
                            <div className={classes.entry} key={index}>
                                <Input
                                    className={classes.input}
                                    placeholder="Find"
                                    label="text to find"
                                    value={localPatterns[index]['find'] || ''}
                                    onChange={(event) =>
                                        setLocalPatterns((oldArr) => {
                                            const values = [...oldArr];
                                            values[index]['find'] = event.target.value;
                                            return values;
                                        })
                                    }
                                />
                                <Input
                                    className={classes.input}
                                    placeholder="Replace"
                                    label="replacement text"
                                    value={localPatterns[index]['replace'] || ''}
                                    onChange={(event) =>
                                        setLocalPatterns((oldArr) => {
                                            const values = [...oldArr];
                                            values[index]['replace'] = event.target.value;
                                            return values;
                                        })
                                    }
                                />
                                <Checkbox
                                    id={'rgx' + index}
                                    className={classes.input}
                                    isChecked={localPatterns[index]['isRegex']}
                                    onChange={(event) =>
                                        setLocalPatterns((oldArr) => {
                                            const values = [...oldArr];
                                            values[index]['isRegex'] = event.target.checked;
                                            return values;
                                        })
                                    }
                                >
                                    Regex
                                </Checkbox>
                            </div>
                        ))}
                    </div>
                </ModalBody>
                <ModalFooter>
                    <Button
                        mr={2}
                        onClick={() => {
                            if (localPatterns.length < 10)
                                setLocalPatterns((old) =>
                                    old.concat({ find: '', replace: '', isRegex: false }),
                                );
                        }}
                        isDisabled={localPatterns.length >= 10}
                    >
                        Add Row
                    </Button>
                    <Button
                        colorScheme="red"
                        onClick={() =>
                            setLocalPatterns([{ find: '', replace: '', isRegex: false }])
                        }
                    >
                        Clear all
                    </Button>
                    <div className={classes.spacer}></div>
                    <Button colorScheme="blue" mr={2} onClick={onModalSave}>
                        Save
                    </Button>
                    <Button onClick={onModalClose} ref={initialRef}>
                        Cancel
                    </Button>
                </ModalFooter>
            </ModalContent>
        </Modal>
    );
};

export default FindReplaceModal;
