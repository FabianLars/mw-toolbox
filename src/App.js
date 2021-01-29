import React, { useEffect, useRef, useState } from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { Account, Delete, Download, List, Move, Purge, Upload } from './pages';

const App = () => {
    // useRef to make useEffect skip the change from useState
    const mounted = useRef(false);
    // Init user state with sessionstorage or default
    const [user, setUser] = useState(
        () =>
            JSON.parse(window.sessionStorage.getItem('userObj')) ?? {
                isOnline: false,
                isPersistent: false,
                username: '',
                password: '',
                url: '',
            }
    );

    // Update sessionStorage on every user object change
    useEffect(() => {
        if (mounted.current) {
            window.sessionStorage.setItem('userObj', JSON.stringify(user));
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
                <Route path="/List" element={<List isOnline={user.isOnline} />} />
                <Route path="/Move" element={<Move isOnline={user.isOnline} />} />
                <Route path="/Purge" element={<Purge isOnline={user.isOnline} />} />
                <Route path="/Upload" element={<Upload isOnline={user.isOnline} />} />
            </Routes>
        </Router>
    );
};

export default App;
