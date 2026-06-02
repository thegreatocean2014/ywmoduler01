import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-shell';

/**
 * Tauri 服务模块
 */

// 调用后端命令示例
export async function getUserData(): Promise<string> {
    try {
        const result = await invoke<string>('get_user_data');
        return result;
    } catch (error) {
        console.error('Failed to get user data:', error);
        throw error;
    }
}

// 保存用户偏好设置
export async function savePreferences(key: string, value: string): Promise<void> {
    try {
        await invoke('save_preferences', { key, value });
    } catch (error) {
        console.error('Failed to save preferences:', error);
        throw error;
    }
}

// 打开外部链接
export async function openExternalUrl(url: string): Promise<void> {
    try {
        await open(url);
    } catch (error) {
        console.error('Failed to open URL:', error);
        throw error;
    }
}


