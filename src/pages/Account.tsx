import { promisified } from 'tauri/api/tauri';
import { listen } from 'tauri/api/event';

import React, { useEffect, useState } from 'react';
import { Button, Checkbox, Flex, Input, Spacer, Text } from '@chakra-ui/react';

import Header from '../components/sections/Header';
//({ wikiurl, loginname, password, is_persistent }: {wikiurl: string, loginname: string, password: string, is_persistant: boolean})

type InitRes = {
    wikiurl: string,
    loginname: string,
    password: string,
    is_persistent: boolean,
}

type LoggedInEvent = {
    payload: {
        username: string,
        url: string,
    },
    type: string,
}

function Account() {
    const [wurl, setWurl] = useState('https://leagueoflegends.fandom.com/de/api.php');
    const [lgname, setLgname] = useState('');
    const [lgpasswd, setLgpasswd] = useState('');
    const [logginin, setLoggingin] = useState(false);
    const [persistent, setPersistent] = useState(false);
    const [loggedin, setLoggedin] = useState(false);
    const [user, setUser] = useState({loggedin: false, username: "", url: ""})

    function init() {
        (promisified({
            cmd: 'init',
        }) as Promise<InitRes>).then(res => {
            const { wikiurl, loginname, password, is_persistent } = res
            console.log(wikiurl, loginname, is_persistent);
            if (wikiurl !== "") setWurl(res.wikiurl);
            setLgname(loginname);
            setLgpasswd(password);
            setPersistent(is_persistent);
        }).catch(err => console.log(err));
    }

    function login() {
        setLoggingin(true);
        promisified({
            cmd: 'login',
            loginname: lgname,
            password: lgpasswd,
            wikiurl: wurl,
            is_persistent: persistent,
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
        listen('loggedin', ({payload}: LoggedInEvent) => {
            setLoggingin(false);
            setLoggedin(true);
            setUser({ loggedin: true, username: payload.username, url: payload.url });
        });
        init();
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
                <Text fontSize="xl" align="center">{user.loggedin ? user.username : ''}</Text>
                <Text fontSize="xl" mb={2} align="center">{user.loggedin ? user.url : 'Not logged in!' }</Text>
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
                    <Checkbox isChecked={persistent} onChange={(event) => setPersistent(event.target.checked)}>Stay logged in</Checkbox>
                    <Button ml={2} isLoading={logginin} colorScheme="blue" onClick={login}>
                    Login
                </Button>
                </Flex>

                {/* <button onClick={list}>list</button> */}
            </Flex>
        </Flex>
    );
}

export default Account;
