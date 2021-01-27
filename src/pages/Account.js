import { promisified } from 'tauri/api/tauri';

import React, { useEffect, useState } from 'react';
import { Button, Checkbox, Divider, Flex, FormControl, FormLabel, Input, Text, useToast } from '@chakra-ui/react';

import Header from '../components/Header';

const Account = ({ user, setUser }) => {
    const [apiUrl, setApiUrl] = useState('https://leagueoflegends.fandom.com/de/api.php');
    const [lgname, setLgname] = useState('');
    const [lgpasswd, setLgpasswd] = useState('');
    const [logginin, setLoggingin] = useState(false);
    const [persistent, setPersistent] = useState(false);
    const [apiUrlInvalid, setApiUrlInvalid] = useState(false);
    const [lgnameInvalid, setLgnameInvalid] = useState(false);
    const [lgpasswdInvalid, setLgpasswdInvalid] = useState(false);
    const toast = useToast();

    const login = () => {
        setLoggingin(true);
        promisified({
            cmd: 'login',
            loginname: lgname,
            password: lgpasswd,
            wikiurl: apiUrl,
            is_persistent: persistent,
        })
            .then((res) => {
                setLoggingin(false);
                setUser({
                    isOnline: true,
                    isPersistent: persistent,
                    username: res.username,
                    password: lgpasswd,
                    url: res.url,
                });
            })
            .catch((err) => {
                setLoggingin(false);
                setUser((user) => {
                    return { isOnline: false, ...user };
                });
                toast({
                    title: "Couldn't log in!",
                    description: err.split('Login failed! ')[1],
                    status: 'error',
                    duration: 10000,
                    isClosable: true,
                });
            });
    };

    useEffect(() => {
        if (typeof window.__TAURI_INVOKE_HANDLER__ === 'function') {
            if (user.isOnline) {
                setLgname(user.username);
                setPersistent(user.isPersistent);
                setLgpasswd(user.password);
                setApiUrl(user.url);
            } else {
                promisified({
                    cmd: 'init',
                })
                    .then((res) => {
                        const { wikiurl, loginname, password, is_persistent } = res;
                        if (wikiurl !== '') setApiUrl(res.wikiurl);
                        setLgname(loginname);
                        setLgpasswd(password);
                        setPersistent(is_persistent);
                    })
                    .catch((err) => console.error(err));
            }
        }
        // eslint-disable-next-line
    }, []);

    useEffect(() => {
        if (!apiUrl.endsWith('api.php') || apiUrl.startsWith('http://') === apiUrl.startsWith('https://')) {
            setApiUrlInvalid(true);
        } else {
            setApiUrlInvalid(false);
        }
        if (!lgname.includes('@')) {
            setLgnameInvalid(true);
        } else {
            setLgnameInvalid(false);
        }
        if (/\W/.test(lgpasswd) || lgpasswd.length <= 16) {
            setLgpasswdInvalid(true);
        } else {
            setLgpasswdInvalid(false);
        }
    }, [apiUrl, lgname, lgpasswd]);

    return (
        <Flex direction="column" align="center" m="0 1rem" h="100vh">
            <Header isDisabled={logginin} isOnline={user.isOnline} />

            <Flex as="main" direction="column" align="center" m="0 auto" w="50%" h="100%" justify="center">
                <Text fontSize="xl" align="center">
                    {user.isOnline ? user.username : ''}
                </Text>
                <Text fontSize="xl" align="center">
                    {user.isOnline ? user.url : 'Not logged in!'}
                </Text>
                <Divider my={2} />
                <FormControl isRequired isInvalid={apiUrlInvalid}>
                    <FormLabel htmlFor="api-url">API URL</FormLabel>
                    <Input
                        id="api-url"
                        value={apiUrl}
                        onChange={(event) => setApiUrl(event.target.value)}
                        placeholder="Full URL pointing to api.php)"
                    />
                </FormControl>
                <Divider my={2} />
                <FormControl isRequired isInvalid={lgnameInvalid}>
                    <FormLabel htmlFor="loginname">Bot Loginname</FormLabel>
                    <Input
                        id="loginname"
                        value={lgname}
                        onChange={(event) => setLgname(event.target.value)}
                        placeholder="Generated via Special:BotPasswords"
                    />
                </FormControl>
                <Divider my={2} />
                <FormControl isRequired isInvalid={lgpasswdInvalid}>
                    <FormLabel htmlFor="password">Bot Password</FormLabel>
                    <Input
                        id="password"
                        value={lgpasswd}
                        onChange={(event) => setLgpasswd(event.target.value)}
                        type="password"
                        placeholder="Generated via Special:BotPasswords"
                    />
                </FormControl>
                <Flex direction="row" w="100%" justify="flex-end" mt={2}>
                    <Checkbox isChecked={persistent} onChange={(event) => setPersistent(event.target.checked)}>
                        Remember me
                    </Checkbox>
                    <Divider orientation="vertical" mx={2} />
                    <Button
                        isDisabled={apiUrlInvalid || lgnameInvalid || lgpasswdInvalid}
                        isLoading={logginin}
                        onClick={login}
                    >
                        Log in
                    </Button>
                </Flex>
            </Flex>
        </Flex>
    );
};

export default Account;
