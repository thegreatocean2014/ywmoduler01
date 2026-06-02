/**
 * 检测应用是否运行在 Tauri 环境中
 */

export const isTauriEnv = (): boolean => {
    return '__TAURI__' in window;
};

export const getTauriVersion = (): string | null => {
    if (isTauriEnv()) {
        return (window as any).__TAURI_METADATA__?.version || null;
    }
    return null;
};


