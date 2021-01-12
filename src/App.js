import React, { useState } from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { Account, Delete, List, Move, Other } from './pages';
import './App.css';

const App = () => {
    const [user, setUser] = useState({ loggedin: false, isPersistent: false, username: '', password: '', url: '' });

    return (
        <Router>
            <Routes>
                <Route path="/" element={<Account user={user} setUser={setUser} />} />
                <Route path="/Delete" element={<Delete />} />
                <Route path="/List" element={<List />} />
                <Route path="/Move" element={<Move />} />
                <Route path="/Other" element={<Other />} />
            </Routes>
        </Router>
    );
};

export default App;
