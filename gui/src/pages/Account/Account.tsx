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
import { errorToast } from '../../helpers/toast';

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
    const [usernameInvalid, setUsernameInvalid] = useState(false);
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
                toast(errorToast(err));
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

    const handleInput = ({
        target: { value, id, checked, name },
    }: React.ChangeEvent<HTMLInputElement>) => {
        setProfiles((old) => {
            const curr = [...old];
            //@ts-ignore omg dynamic object indexing is sooo annoying in typescript
            curr[currentProfile][name || id] = id === 'save-password' ? checked : value;
            return curr;
        });
    };

    // using useEffect for this to not rely on input events for validity-checks
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
            setUsernameInvalid(true);
        } else {
            setUsernameInvalid(false);
        }
        if (/\W/.test(curr.password) || curr.password.length <= 16) {
            setPasswordInvalid(true);
        } else {
            setPasswordInvalid(false);
        }
    }, [profiles, currentProfile]);

    useEffect(() => setNavDisabled(logginin), [logginin]);

    return (
        <Flex
            as="main"
            direction="column"
            align="center"
            /* second null: break menu */
            w={['100%', null, '75%', null, '50%']}
            justify="center"
        >
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
                        name="profile"
                        value={profiles[currentProfile].profile}
                        onChange={handleInput}
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
            <FormControl id="url" isRequired isInvalid={urlInvalid}>
                <FormLabel>API URL</FormLabel>
                <Input
                    value={profiles[currentProfile].url}
                    onChange={handleInput}
                    isDisabled={logginin || profiles[currentProfile].isOnline}
                    placeholder="Full api.php URL (https://wikiname.fandom.com/en/api.php)"
                />
            </FormControl>
            <Divider my={2} />
            <FormControl id="username" isRequired isInvalid={usernameInvalid}>
                <FormLabel>Bot Loginname</FormLabel>
                <Input
                    value={profiles[currentProfile].username}
                    onChange={handleInput}
                    isDisabled={logginin || profiles[currentProfile].isOnline}
                    placeholder="Generated via Special:BotPasswords"
                />
            </FormControl>
            <Divider my={2} />
            <FormControl id="password" isRequired isInvalid={passwordInvalid}>
                <FormLabel>Bot Password</FormLabel>
                <Input
                    value={profiles[currentProfile].password}
                    onChange={handleInput}
                    isDisabled={logginin || profiles[currentProfile].isOnline}
                    type="password"
                    placeholder="Generated via Special:BotPasswords"
                />
            </FormControl>
            <Flex direction="row" w="100%" h="40px" justify="flex-end" mt={2}>
                <Checkbox
                    id="save-password"
                    name="savePassword"
                    isDisabled={logginin || profiles[currentProfile].isOnline}
                    isChecked={profiles[currentProfile].savePassword}
                    onChange={handleInput}
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
