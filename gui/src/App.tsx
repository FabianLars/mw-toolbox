import { useEffect, useRef, useState } from 'preact/hooks';
import { Router, Route } from 'preact-router';
import { invoke } from '@tauri-apps/api/tauri';
import { Account, Delete, Download, Edit, List, Move, Purge, Upload } from './pages';
import { AnyComponent } from 'preact';

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
            }) as Promise<User>).then((res) => {
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
        //@ts-ignore
        <Router>
            <Account path="/" user={user} setUser={setUser} />
            <Delete path="/Delete" isOnline={user.isOnline} />
            <Download path="/Download" isOnline={user.isOnline} />
            <Edit path="/Edit" isOnline={user.isOnline} />
            <List path="/List" isOnline={user.isOnline} />
            <Move path="/Move" isOnline={user.isOnline} />
            <Purge path="/Purge" isOnline={user.isOnline} />
            <Upload path="/Upload" isOnline={user.isOnline} />
        </Router>
    );
};

export default App;
