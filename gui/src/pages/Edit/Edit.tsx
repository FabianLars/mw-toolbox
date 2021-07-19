import { Button, Flex, useDisclosure, useToast } from '@chakra-ui/react';
import React, { useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import FindReplaceModal from './FindReplaceModal';
import { listen, emit } from '@tauri-apps/api/event';
import { Checkbox, Input, Label, Textarea } from '@/components';
import { errorToast, successToast } from '@/helpers/toast';
import { removeFirst } from '@/helpers/array';
import classes from './Edit.module.css';

type Pattern = {
    find: string;
    replace: string;
    isRegex: boolean;
};

type Props = {
    isOnline: boolean;
    setNavDisabled: React.Dispatch<React.SetStateAction<boolean>>;
};

const Edit = ({ isOnline, setNavDisabled }: Props) => {
    const [isRunning, setIsRunning] = useState(false);
    const [isAuto, setIsAuto] = useState(false);
    const [isLoading, setIsLoading] = useState(false);
    const [pageList, setPageList] = useState('');
    const [pageContent, setPageContent] = useState('');
    const [currentPage, setCurrentPage] = useState('');
    const [editSummary, setEditSummary] = useState('');
    const [patterns, setPatterns] = useState<Pattern[]>([
        { find: '', replace: '', isRegex: false },
    ]);
    const { isOpen, onOpen, onClose } = useDisclosure();
    const toast = useToast();
    const initialRef = useRef<HTMLButtonElement>();

    const start = () => {
        setIsRunning(true);
        if (isAuto) {
            setPageContent('');
            invoke('auto_edit', {
                titles: pageList,
                patterns,
                summary: editSummary,
            })
                .catch((err) => {
                    toast(errorToast(err));
                })
                .finally(stop);
        } else {
            getNextPage();
        }
    };

    const stop = () => {
        if (isAuto) {
            emit('cancel-autoedit');
        }
        setPageList((state) => currentPage + '\n' + state);
        setPageContent('');
        setIsRunning(false);
    };

    const getNextPage = () => {
        setPageContent('');
        setIsLoading(true);
        const pages = pageList.split(/\r?\n/);
        const curr = pages.shift();
        setCurrentPage(curr ?? '');
        setPageList(pages.join('\n'));
        if (!curr) {
            setIsRunning(false);
            setIsLoading(false);
        } else {
            (
                invoke('get_page', {
                    page: curr,
                    patterns: patterns,
                }) as Promise<{ content: string; edited: boolean }>
            )
                .then(({ content }) => {
                    setPageContent(content);
                })
                .catch((err) => {
                    stop();
                    toast(errorToast(err));
                })
                .finally(() => setIsLoading(false));
        }
    };

    const save = () => {
        setIsLoading(true);
        (
            invoke('edit', {
                title: currentPage,
                content: pageContent
                    .replace(/[\u007F-\u009F\u200B]/g, '')
                    .replace(/…/g, '...')
                    .trim(),
                summary: editSummary || null,
            }) as Promise<string>
        )
            .then((res) => {
                toast(successToast('Edit successful', res));
                getNextPage();
            })
            .catch((err) => {
                setIsLoading(false);
                toast(errorToast(err));
            });
    };

    useEffect(
        () => setNavDisabled(isLoading || isAuto ? isRunning : false),
        [isLoading, isRunning],
    );

    useEffect(() => {
        const unlistenEdited = listen('page-edited', ({ payload }: { payload: string }) => {
            setPageList((old) => removeFirst(old.split(/\r?\n/), payload).join('\n'));
        });
        const unlistenSkipped = listen('page-skipped', ({ payload }: { payload: string }) => {
            setPageList((old) => removeFirst(old.split(/\r?\n/), payload).join('\n'));
        });

        const getCache = async () => {
            const list: string | null = await invoke('cache_get', { key: 'edit-pagelist' });
            const patts: Pattern[] | null = await invoke('cache_get', { key: 'edit-patterns' });
            const summary: string | null = await invoke('cache_get', { key: 'edit-summary' });
            const auto: boolean | null = await invoke('cache_get', { key: 'edit-isauto' });

            if (list) setPageList(list);
            if (patts) setPatterns(patts);
            if (summary) setEditSummary(summary);
            if (auto) setIsAuto(auto);
        };

        getCache();

        return () => {
            unlistenEdited.then((f) => f());
            unlistenSkipped.then((f) => f());
        };
    }, []);

    return (
        <>
            <Flex w="100%" h="100%" direction={['column', null, 'row']}>
                <Textarea
                    className={classes.list}
                    label="list of pages"
                    isDisabled={isRunning}
                    placeholder="List of pages to operate on. Separated by newline."
                    value={pageList}
                    onChange={(event) => setPageList(event.target.value)}
                    onBlur={() => {
                        setPageList((old) => {
                            return old
                                .split(/\r?\n/)
                                .map((el: string) => el.trim())
                                .filter(Boolean)
                                .join('\n');
                        });
                        invoke('cache_set', { key: 'edit-pagelist', value: pageList });
                    }}
                />
                <Flex direction="column" flex="1" ml={[null, null, 4]}>
                    <Textarea
                        className={classes.content}
                        label="page content container"
                        isDisabled={isAuto || isLoading || !isRunning}
                        placeholder="Page contents will be displayed here."
                        value={pageContent}
                        onChange={(event) => setPageContent(event.target.value)}
                    />
                    <div className={classes.grid}>
                        <div className={classes.giCurrent}>
                            Current page:{' '}
                            {isRunning
                                ? isAuto
                                    ? 'Automated saving mode...'
                                    : currentPage
                                : 'Not running!'}
                        </div>
                        <div className={classes.giSetup}>
                            <Button
                                mt={2}
                                title="This will be processed before contents get displayed!"
                                onClick={onOpen}
                                isDisabled={isLoading}
                            >
                                Setup Find & Replace
                            </Button>
                        </div>
                        <div className={classes.giSummary}>
                            <Label htmlFor="edit-summary" className={classes.label}>
                                Edit summary:
                            </Label>
                            <Input
                                id="edit-summary"
                                value={editSummary}
                                onChange={(event) => setEditSummary(event.target.value)}
                                onBlur={() =>
                                    invoke('cache_set', { key: 'edit-summary', value: editSummary })
                                }
                            />
                        </div>
                        <div className={classes.giControls}>
                            <Flex
                                direction="column"
                                align="center"
                                justify="space-between"
                                h="100%"
                            >
                                <Button
                                    w="100%"
                                    onClick={() => (isRunning ? stop() : start())}
                                    isDisabled={!isOnline || isLoading || pageList.trim() === ''}
                                    title={
                                        !isOnline
                                            ? 'Please login first!'
                                            : isRunning
                                            ? ''
                                            : 'This might take a while!'
                                    }
                                >
                                    {isRunning ? 'Stop' : 'Start'}
                                </Button>
                                <Checkbox
                                    id="auto-save"
                                    isChecked={isAuto}
                                    onChange={(event) => {
                                        setIsAuto(event.target.checked);
                                    }}
                                    onBlur={() => {
                                        invoke('cache_set', {
                                            key: 'edit-isauto',
                                            value: isAuto,
                                        });
                                    }}
                                    isDisabled={isRunning}
                                >
                                    Auto-Save
                                </Checkbox>
                                <Button
                                    w="100%"
                                    isDisabled={!isRunning || !currentPage}
                                    isLoading={isLoading}
                                    onClick={getNextPage}
                                >
                                    Skip
                                </Button>
                                <Button
                                    w="100%"
                                    isDisabled={!isRunning || !currentPage}
                                    isLoading={isLoading}
                                    onClick={save}
                                >
                                    Save
                                </Button>
                            </Flex>
                        </div>
                    </div>
                </Flex>
            </Flex>

            <FindReplaceModal
                isOpen={isOpen}
                onClose={onClose}
                patterns={patterns}
                setPatterns={setPatterns}
                initialRef={initialRef as React.RefObject<HTMLButtonElement>}
            />
        </>
    );
};

export default Edit;
