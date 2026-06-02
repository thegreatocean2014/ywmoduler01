import { invoke } from '@tauri-apps/api/core';

import { isTauriEnv } from '#/utils/tauri-detector';

export const storageService = {
    /**
     * 获取数据
     */
    async getItem(key: string): Promise<string | null> {
        if (isTauriEnv()) {
            try {
                return await invoke<string | null>('get_storage', { key });
            } catch {
                return null;
            }
        } else {
            return localStorage.getItem(key);
        }
    },

    /**
     * 设置数据
     */
    async setItem(key: string, value: string): Promise<void> {
        if (isTauriEnv()) {
            await invoke('set_storage', { key, value });
        } else {
            localStorage.setItem(key, value);
        }
    },

    /**
     * 删除数据
     */
    async removeItem(key: string): Promise<void> {
        if (isTauriEnv()) {
            await invoke('remove_storage', { key });
        } else {
            localStorage.removeItem(key);
        }
    },

    /**
     * 清空数据
     */
    async clear(): Promise<void> {
        if (isTauriEnv()) {
            await invoke('clear_storage');
        } else {
            localStorage.clear();
        }
    },
};


