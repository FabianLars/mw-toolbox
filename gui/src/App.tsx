import React, { useEffect, useRef, useState } from 'react';
import { HashRouter as Router, Route, Routes } from 'react-router-dom';
import { invoke } from '@tauri-apps/api/tauri';
import { Account, Delete, Download, Edit, List, Move, Purge, Upload } from './pages';
import { Center, Flex } from '@chakra-ui/react';
import { Header } from './components';

export type User = {
    username: string;
    password: string;
    url: string;
    isPersistent: boolean;
    isOnline: boolean;
};

const App = () => {
    // useRef to make useEffect skip the change from useState
    const mounted = useRef(false);
    // Init dummy object to prevent errors on startup
    const [user, setUser] = useState<User>({
        username: '',
        password: '',
        url: '',
        isPersistent: false,
        isOnline: false,
    });
    const [navDisabled, setNavDisabled] = useState(false);

    // Init user state from cache or default
    // This exists to handle reloads
    useEffect(() => {
        if (!!window.__TAURI__) {
            (
                invoke('cache_get', {
                    key: 'userObj',
                }) as Promise<User | null>
            ).then((res) => {
                const {
                    isOnline = false,
                    isPersistent = false,
                    username = '',
                    password = '',
                    url = '',
                } = res || {};
                setUser({
                    isOnline,
                    isPersistent,
                    username,
                    password,
                    url,
                });
            });
        }
    }, []);

    // Update cache on every user object change
    // This exists to handle reloads
    useEffect(() => {
        if (mounted.current) {
            invoke('cache_set', {
                key: 'userObj',
                value: user,
            });
        } else {
            mounted.current = true;
        }
    }, [user]);

    return (
        <Router>
            <Flex direction="column" h="100vh" w="100vw" userSelect="none">
                <Header isDisabled={navDisabled} isOnline={user.isOnline} />
                <Center flex="1 1 auto" overflow="hidden" p={4}>
                    <Routes>
                        <Route
                            path="/"
                            element={
                                <Account
                                    user={user}
                                    setUser={setUser}
                                    setNavDisabled={setNavDisabled}
                                />
                            }
                        />
                        <Route
                            path="/Delete"
                            element={
                                <Delete isOnline={user.isOnline} setNavDisabled={setNavDisabled} />
                            }
                        />
                        <Route
                            path="/Download"
                            element={
                                <Download
                                    isOnline={user.isOnline}
                                    setNavDisabled={setNavDisabled}
                                />
                            }
                        />
                        <Route
                            path="/Edit"
                            element={
                                <Edit isOnline={user.isOnline} setNavDisabled={setNavDisabled} />
                            }
                        />
                        <Route
                            path="/List"
                            element={
                                <List isOnline={user.isOnline} setNavDisabled={setNavDisabled} />
                            }
                        />
                        <Route
                            path="/Move"
                            element={
                                <Move isOnline={user.isOnline} setNavDisabled={setNavDisabled} />
                            }
                        />
                        <Route
                            path="/Purge"
                            element={
                                <Purge isOnline={user.isOnline} setNavDisabled={setNavDisabled} />
                            }
                        />
                        <Route
                            path="/Upload"
                            element={
                                <Upload isOnline={user.isOnline} setNavDisabled={setNavDisabled} />
                            }
                        />
                    </Routes>
                </Center>
            </Flex>
        </Router>
    );
};

export default App;
