import React, { useEffect, useRef, useState } from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { promisified } from 'tauri/api/tauri';
import { Account, Delete, Download, Edit, List, Move, Purge, Upload } from './pages';

const App = () => {
    // useRef to make useEffect skip the change from useState
    const mounted = useRef(false);
    // Init dummy object to prevent errors on startup
    const [user, setUser] = useState({});

    // Init user state from cache or default
    // This exists to handle reloads
    useEffect(() => {
        if (!!window.__TAURI__) {
            promisified({
                cmd: 'cacheGet',
                key: 'userObj',
            }).then(({ isOnline = false, isPersistent = false, username = '', password = '', url = '' }) => {
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
            promisified({
                cmd: 'cacheSet',
                key: 'userObj',
                value: user,
            });
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
