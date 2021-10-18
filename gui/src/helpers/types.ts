type FocusableElement = {
    focus(options?: FocusOptions): void;
};

type Profile = {
    // fix dynamic indexing in Account.tsx
    [index: string]: string | boolean;
    profile: string;
    username: string;
    password: string;
    url: string;
    savePassword: boolean;
    isOnline: boolean;
};

export type { FocusableElement, Profile };
