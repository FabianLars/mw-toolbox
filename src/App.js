import React, { useState } from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { Account, Delete, List, Move, Other } from './pages';
import './App.css';

const App = () => {
    const [user, setUser] = useState({ isOnline: false, isPersistent: false, username: '', password: '', url: '' });

    return (
        <Router>
            <Routes>
                <Route path="/" element={<Account user={user} setUser={setUser} />} />
                <Route path="/Delete" element={<Delete isOnline={user.isOnline} />} />
                <Route path="/List" element={<List isOnline={user.isOnline} />} />
                <Route path="/Move" element={<Move isOnline={user.isOnline} />} />
                <Route path="/Other" element={<Other isOnline={user.isOnline} />} />
            </Routes>
        </Router>
    );
};

export default App;
