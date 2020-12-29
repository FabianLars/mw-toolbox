import { promisified } from 'tauri/api/tauri';
import { listen } from 'tauri/api/event';

import React, { useEffect, useState } from 'react';
import { Button, Checkbox, Flex, Input, Spacer } from '@chakra-ui/react';

import Header from '../components/sections/Header';

function Home() {
    const [wurl, setWurl] = useState('https://leagueoflegends.fandom.com/de/api.php');
    const [lgname, setLgname] = useState('');
    const [lgpasswd, setLgpasswd] = useState('');
    const [logginin, setLoggingin] = useState(false);
    const [persistent, setPersistent] = useState(false);

    function login() {
        setLoggingin(true);
        promisified({
            cmd: 'login',
            loginname: lgname,
            password: lgpasswd,
            url: wurl,
            persistent: persistent,
        })
            .then((res) => console.log(res))
            .catch((err) => console.error(err));
    }

    function list() {
        promisified({
            cmd: 'list',
        })
            .then((res) => console.log(res))
            .catch((err) => console.error(err));
    }

    useEffect(() => {
        listen('loggedin', (payload) => {
            setLoggingin(false);
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
                <Flex direction="row" w="100%" justify="flex-end">
                    <Checkbox onChange={(event) => setPersistent(event.target.checked)}>Stay logged in</Checkbox>
                    <Button ml={2} isLoading={logginin} colorScheme="blue" onClick={login}>
                    Login
                </Button>
                </Flex>

                {/* <button onClick={list}>list</button> */}
            </Flex>
        </Flex>
    );
}

export default Home;
