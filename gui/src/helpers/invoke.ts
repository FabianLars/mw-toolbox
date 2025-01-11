import { invoke } from '@tauri-apps/api/core';

/**
 * Get an object from the cache managed by rust.
 */
const getCache = async <T>(key: string): Promise<T | null> => {
    return await invoke<T | null>('cache_get', { key });
};

/**
 * Store an object in rust.
 *
 * Returns a boolean indicating if a value behind the key already existed and therefore got updated.
 */
const setCache = async (key: string, value: unknown): Promise<boolean> => {
    return await invoke<boolean>('cache_set', { key, value });
};

export { getCache, setCache };
