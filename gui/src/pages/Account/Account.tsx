import { invoke } from '@tauri-apps/api/tauri';

import React, { useEffect, useState } from 'react';
import {
    Button,
    Checkbox,
    Divider,
    Flex,
    FormControl,
    FormLabel,
    IconButton,
    Input,
    Select,
    useToast,
} from '@chakra-ui/react';

import type { Profile } from '../../App';
import { AddIcon, CloseIcon } from '@chakra-ui/icons';

type Props = {
    profiles: Profile[];
    setProfiles: React.Dispatch<React.SetStateAction<Profile[]>>;
    currentProfile: number;
    setCurrentProfile: React.Dispatch<React.SetStateAction<number>>;
    setNavDisabled: React.Dispatch<React.SetStateAction<boolean>>;
};

const Account = ({
    profiles,
    setProfiles,
    currentProfile,
    setCurrentProfile,
    setNavDisabled,
}: Props) => {
    const [logginin, setLoggingin] = useState(false);
    const [urlInvalid, seturlInvalid] = useState(false);
    const [usernameInvalid, setUsnameInvalid] = useState(false);
    const [passwordInvalid, setPasswordInvalid] = useState(false);
    const toast = useToast();

    const login = () => {
        setLoggingin(true);
        (invoke('login', { profiles, current: currentProfile }) as Promise<number>)
            .then((res) => {
                setProfiles((old) => {
                    const curr = [...old];
                    curr[res].isOnline = true;
                    return curr;
                });
            })
            .catch((err) => {
                setProfiles((old) => {
                    const curr = [...old];
                    curr.map((p) => (p.isOnline = false));
                    return curr;
                });
                toast({
                    title: `Couldn't log in! - ${err.code}-Error`,
                    description: <span style={{ wordBreak: 'break-word' }}>{err.description}</span>,
                    status: 'error',
                    duration: 10000,
                    isClosable: true,
                });
            })
            .finally(() => setLoggingin(false));
    };

    const logout = () => {
        setLoggingin(true);
        (invoke('logout') as Promise<any>).finally(() => {
            setLoggingin(false);
            setProfiles((old) => {
                const curr = [...old];
                curr.map((p) => (p.isOnline = false));
                return curr;
            });
        });
    };

    const addProfile = () => {
        const oldLen = profiles.length;
        if (oldLen < 10) {
            setProfiles((old) =>
                old.concat({
                    profile: 'Profile ' + (old.length + 1),
                    username: '',
                    password: '',
                    url: '',
                    savePassword: old[currentProfile].savePassword || false,
                    isOnline: false,
                }),
            );
            setCurrentProfile(oldLen);
        }
    };

    const removeProfile = () => {
        setProfiles((old) => {
            const curr = [...old];
            curr.splice(currentProfile);
            return curr;
        });
        setCurrentProfile((old) => old - 1);
    };

    useEffect(() => {
        const curr = profiles[currentProfile];
        if (
            !curr.url.endsWith('api.php') ||
            curr.url.startsWith('http://') === curr.url.startsWith('https://')
        ) {
            seturlInvalid(true);
        } else {
            seturlInvalid(false);
        }
        if (!curr.username.includes('@')) {
            setUsnameInvalid(true);
        } else {
            setUsnameInvalid(false);
        }
        if (/\W/.test(curr.password) || curr.password.length <= 16) {
            setPasswordInvalid(true);
        } else {
            setPasswordInvalid(false);
        }
    }, [profiles, currentProfile]);

    useEffect(() => setNavDisabled(logginin), [logginin]);

    return (
        <Flex as="main" direction="column" align="center" w="50%" justify="center">
            <Flex w="100%" alignItems="flex-end">
                <FormControl
                    flex="2"
                    mr={3}
                    id="profile-name"
                    isRequired
                    isInvalid={profiles[currentProfile].profile.trim() === ''}
                >
                    <FormLabel>Profile Name</FormLabel>
                    <Input
                        value={profiles[currentProfile].profile}
                        onChange={(event) =>
                            setProfiles((old) => {
                                const curr = [...old];
                                curr[currentProfile].profile = event.target.value;
                                return curr;
                            })
                        }
                        isDisabled={profiles[currentProfile].isOnline}
                        placeholder="Profile name"
                    />
                </FormControl>
                <FormControl flex="1" id="profile-select" isRequired>
                    <FormLabel>Select Profile</FormLabel>
                    <Select
                        value={currentProfile}
                        onChange={(event) => setCurrentProfile(parseInt(event.target.value))}
                        isDisabled={logginin || profiles[currentProfile].isOnline}
                    >
                        {profiles.map((v, i) => (
                            <option key={i + '-profile-' + v.profile} value={i}>
                                {v.profile || 'Profile ' + (i + 1)}
                            </option>
                        ))}
                    </Select>
                </FormControl>
                <IconButton
                    isDisabled={
                        logginin || profiles.length >= 10 || profiles[currentProfile].isOnline
                    }
                    w={10}
                    mx={3}
                    title="Add additional profile"
                    aria-label="Add additional profile"
                    icon={<AddIcon />}
                    onClick={addProfile}
                />
                <IconButton
                    colorScheme="red"
                    isDisabled={
                        logginin || profiles.length <= 1 || profiles[currentProfile].isOnline
                    }
                    w={10}
                    title="Remove current profile"
                    aria-label="Remove current profile"
                    icon={<CloseIcon />}
                    onClick={removeProfile}
                />
            </Flex>
            <Divider my={2} />
            <FormControl id="api-url" isRequired isInvalid={urlInvalid}>
                <FormLabel>API URL</FormLabel>
                <Input
                    value={profiles[currentProfile].url}
                    onChange={(event) =>
                        setProfiles((old) => {
                            const curr = [...old];
                            curr[currentProfile].url = event.target.value;
                            return curr;
                        })
                    }
                    isDisabled={logginin || profiles[currentProfile].isOnline}
                    placeholder="Full api.php URL (https://wikiname.fandom.com/en/api.php)"
                />
            </FormControl>
            <Divider my={2} />
            <FormControl id="loginname" isRequired isInvalid={usernameInvalid}>
                <FormLabel>Bot Loginname</FormLabel>
                <Input
                    value={profiles[currentProfile].username}
                    onChange={(event) =>
                        setProfiles((old) => {
                            const curr = [...old];
                            curr[currentProfile].username = event.target.value;
                            return curr;
                        })
                    }
                    isDisabled={logginin || profiles[currentProfile].isOnline}
                    placeholder="Generated via Special:BotPasswords"
                />
            </FormControl>
            <Divider my={2} />
            <FormControl id="password" isRequired isInvalid={passwordInvalid}>
                <FormLabel>Bot Password</FormLabel>
                <Input
                    value={profiles[currentProfile].password}
                    onChange={(event) =>
                        setProfiles((old) => {
                            const curr = [...old];
                            curr[currentProfile].password = event.target.value;
                            return curr;
                        })
                    }
                    isDisabled={logginin || profiles[currentProfile].isOnline}
                    type="password"
                    placeholder="Generated via Special:BotPasswords"
                />
            </FormControl>
            <Flex direction="row" w="100%" justify="flex-end" mt={2}>
                <Checkbox
                    isDisabled={logginin || profiles[currentProfile].isOnline}
                    isChecked={profiles[currentProfile].savePassword}
                    onChange={(event) =>
                        setProfiles((old) => {
                            const curr = [...old];
                            curr[currentProfile].savePassword = event.target.checked;
                            return curr;
                        })
                    }
                >
                    Remember password
                </Checkbox>
                <Divider orientation="vertical" mx={2} />
                <Button
                    isDisabled={urlInvalid || usernameInvalid || passwordInvalid}
                    isLoading={logginin}
                    onClick={profiles[currentProfile].isOnline ? logout : login}
                >
                    {profiles[currentProfile].isOnline ? 'Log out' : 'Log in'}
                </Button>
            </Flex>
        </Flex>
    );
};

export default Account;
