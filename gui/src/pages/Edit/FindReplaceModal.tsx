import { useEffect, useState, useRef } from 'react';

import { Button, Checkbox, Input, Modal } from '@/components';
import { setCache } from '@/helpers/invoke';
import cls from './FindReplaceModal.module.css';

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

const FindReplaceModal = ({ isOpen, onClose, patterns, setPatterns }: Props): JSX.Element => {
    const [localPatterns, setLocalPatterns] = useState<Pattern[]>([]);
    const initialRef = useRef<HTMLButtonElement>(null);

    const onModalClose = () => {
        const arr = patterns.map((obj) => Object.assign({}, obj));
        setLocalPatterns(arr);
        onClose();
    };

    const onModalSave = () => {
        const arr = localPatterns.map((obj) => Object.assign({}, obj));
        setPatterns(arr);
        setCache('edit-patterns', arr).finally(onClose);
    };

    useEffect(() => {
        setLocalPatterns(patterns);
    }, [patterns]);

    return (
        <Modal
            onClose={onModalClose}
            isOpen={isOpen}
            initialFocusRef={initialRef}
            header={
                <>
                    Find & Replace
                    <div className={cls.regexinfo}>
                        <a
                            target="_blank"
                            rel="noopener noreferrer"
                            className={cls.link}
                            href="https://docs.rs/regex/"
                            title="Open Regex Documentation"
                        >
                            Click here for the regex docs
                        </a>
                    </div>
                </>
            }
            body={
                <div className={cls.container}>
                    {localPatterns.map((_, index) => (
                        <div className={cls.entry} key={index}>
                            <Input
                                className={cls.input}
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
                                className={cls.input}
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
                                className={cls.input}
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
            }
            footer={
                <>
                    <Button
                        className={cls.mr}
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
                    <div className={cls.spacer}></div>
                    <Button className={cls.mr} colorScheme="blue" onClick={onModalSave}>
                        Save
                    </Button>
                    <Button onClick={onModalClose} ref={initialRef}>
                        Cancel
                    </Button>
                </>
            }
        />
    );
};

export default FindReplaceModal;
