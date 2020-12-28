import { promisified } from 'tauri/api/tauri';
import { listen } from 'tauri/api/event';

import { React, useEffect, useState } from 'react';
import { Button, Flex, Input } from '@chakra-ui/react';

import Header from '../components/sections/Header';

function Home() {
    const [wurl, setWurl] = useState('');
    const [lgname, setLgname] = useState('');
    const [lgpasswd, setLgpasswd] = useState('');

    function login() {
        promisified({
            cmd: 'login',
            loginname: lgname,
            password: lgpasswd,
            url: wurl,
        })
            .then((res) => console.log(res))
            .catch((err) => console.err(err));
    }

    function list() {
        promisified({
            cmd: 'list',
        })
            .then((res) => console.log(res))
            .catch((err) => console.err(err));
    }

    useEffect(() => {
        listen('loggedin', (payload) => {
            console.log(payload);
        });
    }, []);

    return (
        <Flex direction="column" align="center" maxW={{ xl: '1240px' }} m="0 auto" h="100vh">
            <Header />

            <Flex
                as="main"
                direction="column"
                align="center"
                maxW={{ xl: '1200px' }}
                m="0 auto"
                w="50%"
                h="100%"
                justify="center"
            >
                <Input
                    mb={2}
                    value={wurl}
                    onChange={(event) => setWurl(event.target.value)}
                    placeholder="Wiki URL (pointing to api.php => 'https://leagueoflegends.fandom.com/api.php')"
                    isRequired
                />
                <Input
                    mb={2}
                    value={lgname}
                    onChange={(event) => setLgname(event.target.value)}
                    placeholder="Loginname via S:BotPasswords"
                    isRequired
                />
                <Input
                    mb={2}
                    value={lgpasswd}
                    onChange={(event) => setLgpasswd(event.target.value)}
                    type="password"
                    placeholder="Password via S:BotPasswords"
                    isRequired
                />
                <Button colorScheme="blue" onClick={login}>
                    Login
                </Button>
                {/* <button onClick={list}>list</button> */}
            </Flex>
        </Flex>
    );
}

export default Home;
