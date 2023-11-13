import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import FindReplaceModal from './FindReplaceModal';
import { listen, emit } from '@tauri-apps/api/event';
import { Button, Checkbox, Input, Label, Textarea } from '@/components';
import { getCache, setCache } from '@/helpers/invoke';
import { errorToast, successToast } from '@/helpers/toast';
import { removeFirst } from '@/helpers/array';
import cls from './Edit.module.css';

type Pattern = {
    find: string;
    replace: string;
    isRegex: boolean;
};

type Props = {
    isOnline: boolean;
    setNavDisabled: React.Dispatch<React.SetStateAction<boolean>>;
};

const Edit = ({ isOnline, setNavDisabled }: Props): JSX.Element => {
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

    const [isOpen, setIsOpen] = useState(false);

    const onOpen = () => setIsOpen(true);
    const onClose = () => setIsOpen(false);

    const start = () => {
        setIsRunning(true);
        if (isAuto) {
            setPageContent('');
            invoke('auto_edit', {
                titles: pageList,
                patterns,
                summary: editSummary,
            })
                .catch(errorToast)
                .finally(stop);
        } else {
            getNextPage();
        }
    };

    const stop = () => {
        if (isAuto) {
            emit('cancel-action');
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
            invoke<{ content: string; edited: boolean }>('get_page', {
                page: curr,
                patterns: patterns,
            })
                .then(({ content }) => {
                    setPageContent(content);
                })
                .catch((err) => {
                    stop();
                    errorToast(err);
                })
                .finally(() => setIsLoading(false));
        }
    };

    const save = () => {
        setIsLoading(true);
        invoke<string>('edit', {
            title: currentPage,
            content: pageContent
                .replace(/[\u007F-\u009F\u200B]/g, '')
                .replace(/…/g, '...')
                .trim(),
            summary: editSummary || null,
        })
            .then((res) => {
                successToast('Edit successful', res);
                getNextPage();
            })
            .catch((err) => {
                setIsLoading(false);
                errorToast(err);
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

        const init = async () => {
            const list = await getCache<string>('edit-pagelist');
            const patts = await getCache<Pattern[]>('edit-patterns');
            const summary = await getCache<string>('edit-summary');
            const auto = await getCache<boolean>('edit-isauto');

            if (list) setPageList(list);
            if (patts) setPatterns(patts);
            if (summary) setEditSummary(summary);
            if (auto) setIsAuto(auto);
        };

        init();

        return () => {
            unlistenEdited.then((f) => f());
            unlistenSkipped.then((f) => f());
        };
    }, []);

    return (
        <>
            <div className={cls.container}>
                <Textarea
                    className={cls.list}
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
                        setCache('edit-pagelist', pageList);
                    }}
                />
                <div className={cls.right}>
                    <Textarea
                        className={cls.content}
                        label="page content container"
                        isDisabled={isAuto || isLoading || !isRunning}
                        placeholder="Page contents will be displayed here."
                        value={pageContent}
                        onChange={(event) => setPageContent(event.target.value)}
                    />
                    <div className={cls.grid}>
                        <div className={cls.giCurrent}>
                            Current page:{' '}
                            {isRunning
                                ? isAuto
                                    ? 'Automated saving mode...'
                                    : currentPage
                                : 'Not running!'}
                        </div>
                        <div className={cls.giSetup}>
                            <Button
                                title="This will be processed before contents get displayed!"
                                onClick={onOpen}
                                isDisabled={isLoading}
                            >
                                Setup Find & Replace
                            </Button>
                        </div>
                        <div className={cls.giSummary}>
                            <Label htmlFor="edit-summary" className={cls.label}>
                                Edit summary:
                            </Label>
                            <Input
                                id="edit-summary"
                                value={editSummary}
                                onChange={(event) => setEditSummary(event.target.value)}
                                onBlur={() => setCache('edit-summary', editSummary)}
                            />
                        </div>
                        <div className={cls.giControls}>
                            <div className={cls.controls}>
                                <Button
                                    onClick={() => (isRunning ? stop() : start())}
                                    isDisabled={
                                        !isOnline ||
                                        isLoading ||
                                        (!isRunning && pageList.trim() === '')
                                    }
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
                                    onBlur={() => setCache('edit-isauto', isAuto)}
                                    isDisabled={isRunning}
                                >
                                    Auto-Save
                                </Checkbox>
                                <Button
                                    isDisabled={!isRunning || !currentPage}
                                    isLoading={isLoading}
                                    onClick={getNextPage}
                                >
                                    Skip
                                </Button>
                                <Button
                                    isDisabled={!isRunning || !currentPage}
                                    isLoading={isLoading}
                                    onClick={save}
                                >
                                    Save
                                </Button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <FindReplaceModal
                isOpen={isOpen}
                onClose={onClose}
                patterns={patterns}
                setPatterns={setPatterns}
            />
        </>
    );
};

export default Edit;
