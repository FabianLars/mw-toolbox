import { useEffect, useRef, useState } from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { invoke } from '@tauri-apps/api/tauri';
import { Account, Delete, Download, Edit, List, Move, Purge, Upload } from './pages';

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

    // Init user state from cache or default
    // This exists to handle reloads
    useEffect(() => {
        // @ts-ignore
        if (!!window.__TAURI__) {
            (invoke('cache_get', {
                key: 'userObj',
            }) as Promise<User>).then(
                ({ isOnline = false, isPersistent = false, username = '', password = '', url = '' }) => {
                    setUser({
                        isOnline,
                        isPersistent,
                        username,
                        password,
                        url,
                    });
                }
            );
        }
        // eslint-disable-next-line
    }, []);

    // Update cache on every user object change
    // This exists to handle reloads
    useEffect(() => {
        if (mounted.current) {
            invoke('cache_set', {
                key: 'userObj',
                value: user,
            }).catch(console.error);
        } else {
            mounted.current = true;
        }
    }, [user]);

    return (
        <Router>
            <Routes>
                <Route path="/" element={<Account user={user} setUser={setUser} />} />
                <Route path="/Delete" element={<Delete isOnline={user.isOnline} />} />
                <Route path="/Download" element={<Download isOnline={user.isOnline} />} />
                <Route path="/Edit" element={<Edit isOnline={user.isOnline} />} />
                <Route path="/List" element={<List isOnline={user.isOnline} />} />
                <Route path="/Move" element={<Move isOnline={user.isOnline} />} />
                <Route path="/Purge" element={<Purge isOnline={user.isOnline} />} />
                <Route path="/Upload" element={<Upload isOnline={user.isOnline} />} />
            </Routes>
        </Router>
    );
};

export default App;
