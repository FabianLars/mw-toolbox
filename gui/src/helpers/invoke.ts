import { invoke } from '@tauri-apps/api/tauri';

/**
 * Get an object from the cache managed by rust.
 */
const getCache = async <T>(key: string) => {
    return await invoke<T | null>('cache_get', { key });
};

/**
 * Store an object in rust.
 *
 * Returns a boolean indicating if a value behind the key already existed and therefore got updated.
 */
const setCache = async (key: string, value: any) => {
    return await invoke('cache_set', { key, value });
};

export { getCache, setCache };
