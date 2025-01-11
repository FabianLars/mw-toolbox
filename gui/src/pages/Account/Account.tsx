import { invoke } from '@tauri-apps/api/core';

import { useEffect, useState } from 'react';
import { Button, Checkbox, Divider, Input, Label, Select } from '@/components';

import { errorToast } from '@/helpers/toast';
import cls from './Account.module.css';
import { CloseIcon, PlusIcon } from '@/components/icons';
import type { Profile } from '@/helpers/types';

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

    const login = () => {
        setLoggingin(true);
        invoke<number>('login', { profiles, current: currentProfile })
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
                errorToast(err);
            })
            .finally(() => setLoggingin(false));
    };

    const logout = () => {
        setLoggingin(true);
        invoke<never>('logout').finally(() => {
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
            curr[currentProfile][name || id] = id === 'save-password' ? checked : value;
            return curr;
        });
    };

    // using useEffect for this to not rely on input events for validity-checks
    // useful when switching profiles via dropdown and loading profiles from cache
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

    useEffect(() => setNavDisabled(logginin), [logginin, setNavDisabled]);

    return (
        <main className={cls.container}>
            <div className={cls.profile}>
                <div>
                    <Label htmlFor="profile-name" isRequired>
                        Profile Name
                    </Label>
                    <Input
                        id="profile-name"
                        name="profile"
                        value={profiles[currentProfile].profile}
                        onChange={handleInput}
                        isDisabled={profiles[currentProfile].isOnline}
                        isRequired
                        isInvalid={profiles[currentProfile].profile.trim() === ''}
                        placeholder="Profile name"
                    />
                </div>
                <div className="w100">
                    <Label htmlFor="profile-select" isRequired>
                        Select Profile
                    </Label>
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
                </div>
                <Button
                    isDisabled={
                        logginin || profiles.length >= 10 || profiles[currentProfile].isOnline
                    }
                    className={cls.add}
                    title="Add additional profile"
                    aria-label="Add additional profile"
                    onClick={addProfile}
                >
                    <PlusIcon />
                </Button>
                <Button
                    colorScheme="red"
                    isDisabled={
                        logginin || profiles.length <= 1 || profiles[currentProfile].isOnline
                    }
                    className={cls.remove}
                    title="Remove current profile"
                    aria-label="Remove current profile"
                    onClick={removeProfile}
                >
                    <CloseIcon />
                </Button>
            </div>
            <Divider />
            <div className="w100">
                <Label htmlFor="url" isRequired>
                    API URL
                </Label>
                <Input
                    id="url"
                    value={profiles[currentProfile].url}
                    onChange={handleInput}
                    isDisabled={logginin || profiles[currentProfile].isOnline}
                    isRequired
                    isInvalid={urlInvalid}
                    placeholder="Full api.php URL (https://wikiname.fandom.com/en/api.php)"
                />
            </div>
            <Divider />
            <div className="w100">
                <Label htmlFor="username" isRequired>
                    Bot Loginname
                </Label>
                <Input
                    id="username"
                    value={profiles[currentProfile].username}
                    onChange={handleInput}
                    isDisabled={logginin || profiles[currentProfile].isOnline}
                    isRequired
                    isInvalid={usernameInvalid}
                    placeholder="Generated via Special:BotPasswords"
                />
            </div>
            <Divider />
            <div className="w100">
                <Label htmlFor="password" isRequired>
                    Bot Password
                </Label>
                <Input
                    id="password"
                    value={profiles[currentProfile].password}
                    onChange={handleInput}
                    isDisabled={logginin || profiles[currentProfile].isOnline}
                    isPassword
                    isRequired
                    isInvalid={passwordInvalid}
                    placeholder="Generated via Special:BotPasswords"
                />
            </div>
            <div className={cls.buttons}>
                <Checkbox
                    id="save-password"
                    name="savePassword"
                    isDisabled={logginin || profiles[currentProfile].isOnline}
                    isChecked={profiles[currentProfile].savePassword}
                    onChange={handleInput}
                >
                    Remember password
                </Checkbox>
                <Divider orientation="vertical" />
                <Button
                    isDisabled={urlInvalid || usernameInvalid || passwordInvalid}
                    isLoading={logginin}
                    onClick={profiles[currentProfile].isOnline ? logout : login}
                >
                    {profiles[currentProfile].isOnline ? 'Log out' : 'Log in'}
                </Button>
            </div>
        </main>
    );
};

export default Account;
