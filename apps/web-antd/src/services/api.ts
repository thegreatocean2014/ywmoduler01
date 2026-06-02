import { invoke } from '@tauri-apps/api/core';

import { isTauriEnv } from '#/utils/tauri-detector';

/**
 * API 通信层 - 支持 Web 和 Tauri 环境
 */
export class ApiClient {
    private baseUrl: string;

    constructor(baseUrl = 'http://localhost:3000') {
        this.baseUrl = baseUrl;
    }

    async get<T>(endpoint: string, options?: RequestInit): Promise<T> {
        if (isTauriEnv()) {
            // 在Tauri中，可以直接调用后端命令或使用HTTP
            return this.tauriRequest<T>('GET', endpoint, options);
        } else {
            return this.webRequest<T>('GET', endpoint, options);
        }
    }

    async post<T>(endpoint: string, body?: any, options?: RequestInit): Promise<T> {
        if (isTauriEnv()) {
            return this.tauriRequest<T>('POST', endpoint, { ...options, body });
        } else {
            return this.webRequest<T>('POST', endpoint, { ...options, body });
        }
    }

    private async tauriRequest<T>(
        method: string,
        endpoint: string,
        options?: RequestInit
    ): Promise<T> {
        // 通过Tauri命令调用后端API
        return invoke<T>('make_http_request', {
            method,
            url: `${this.baseUrl}${endpoint}`,
            ...options,
        });
    }

    private async webRequest<T>(
        method: string,
        endpoint: string,
        options?: RequestInit
    ): Promise<T> {
        const response = await fetch(`${this.baseUrl}${endpoint}`, {
            method,
            ...options,
        });
        if (!response.ok) {
            throw new Error(`HTTP ${response.status}: ${response.statusText}`);
        }
        return response.json();
    }
}

export const apiClient = new ApiClient();

