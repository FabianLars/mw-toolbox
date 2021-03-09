import { promisified } from 'tauri/api/tauri';

import { useEffect, useState } from 'react';
import { Button, Checkbox, Divider, Flex, FormControl, FormLabel, Input, Text, useToast } from '@chakra-ui/react';

import { Header } from '../../components';
import { User } from '../../App';

type Props = {
    user: User;
    setUser: React.Dispatch<React.SetStateAction<User>>;
};

const Account = ({ user, setUser }: Props) => {
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
        (promisified({
            cmd: 'login',
            loginname: lgname,
            password: lgpasswd,
            wikiurl: apiUrl,
            is_persistent: persistent,
        }) as Promise<{ username: string; url: string }>)
            .then(res => {
                setUser({
                    isOnline: true,
                    isPersistent: persistent,
                    username: res.username,
                    password: lgpasswd,
                    url: res.url,
                });
            })
            .catch(err => {
                setUser(u => ({
                    ...u,
                    isOnline: false,
                }));
                toast({
                    title: "Couldn't log in!",
                    description: err,
                    status: 'error',
                    duration: 10000,
                    isClosable: true,
                });
            })
            .finally(() => setLoggingin(false));
    };

    const logout = () => {
        setLoggingin(true);
        (promisified({
            cmd: 'logout',
        }) as Promise<any>).finally(() => {
            setLoggingin(false);
            setUser(u => ({
                ...u,
                isOnline: false,
            }));
        });
    };

    useEffect(() => {
        // @ts-ignore
        if (!!window.__TAURI__) {
            if (user.isOnline) {
                setLgname(user.username);
                setPersistent(user.isPersistent);
                setLgpasswd(user.password);
                setApiUrl(user.url);
            } else {
                (promisified({
                    cmd: 'init',
                }) as Promise<{ wikiurl: string; loginname: string; password: string; is_persistent: boolean }>)
                    .then(({ wikiurl, loginname, password, is_persistent }) => {
                        if (wikiurl !== '') setApiUrl(wikiurl);
                        setLgname(loginname);
                        setLgpasswd(password);
                        setPersistent(is_persistent);
                    })
                    .catch(console.error);
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
        <Flex direction="column" align="center" p="0 1rem 1rem" h="100vh">
            <Header isDisabled={logginin} isOnline={user.isOnline} />

            <Flex as="main" direction="column" align="center" m="0 auto" w="50%" h="100%" justify="center">
                <Text fontSize="xl" align="center">
                    {user.isOnline ? user.username : ''}
                </Text>
                <Text fontSize="xl" align="center">
                    {user.isOnline ? user.url : 'Not logged in!'}
                </Text>
                <Divider my={2} />
                <FormControl id="api-url" isRequired isInvalid={apiUrlInvalid}>
                    <FormLabel>API URL</FormLabel>
                    <Input
                        value={apiUrl}
                        onChange={event => setApiUrl(event.target.value)}
                        isDisabled={user.isOnline}
                        placeholder="Full URL pointing to api.php"
                    />
                </FormControl>
                <Divider my={2} />
                <FormControl id="loginname" isRequired isInvalid={lgnameInvalid}>
                    <FormLabel>Bot Loginname</FormLabel>
                    <Input
                        value={lgname}
                        onChange={event => setLgname(event.target.value)}
                        isDisabled={user.isOnline}
                        placeholder="Generated via Special:BotPasswords"
                    />
                </FormControl>
                <Divider my={2} />
                <FormControl id="password" isRequired isInvalid={lgpasswdInvalid}>
                    <FormLabel>Bot Password</FormLabel>
                    <Input
                        value={lgpasswd}
                        onChange={event => setLgpasswd(event.target.value)}
                        isDisabled={user.isOnline}
                        type="password"
                        placeholder="Generated via Special:BotPasswords"
                    />
                </FormControl>
                <Flex direction="row" w="100%" justify="flex-end" mt={2}>
                    <Checkbox isChecked={persistent} onChange={event => setPersistent(event.target.checked)}>
                        Remember me
                    </Checkbox>
                    <Divider orientation="vertical" mx={2} />
                    <Button
                        isDisabled={apiUrlInvalid || lgnameInvalid || lgpasswdInvalid}
                        isLoading={logginin}
                        onClick={user.isOnline ? logout : login}
                    >
                        {user.isOnline ? 'Log out' : 'Log in'}
                    </Button>
                </Flex>
            </Flex>
        </Flex>
    );
};

export default Account;
