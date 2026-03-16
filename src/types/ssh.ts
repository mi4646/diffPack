// SSH 相关类型定义

export interface SshConfig {
    host: string;
    port: number;
    username: string;
    authMethod: AuthMethod;
}

export type AuthMethod =
    | { type: "password"; password: string }
    | { type: "keyFile"; keyPath: string; passphrase?: string }
    | { type: "sshAgent" };

export interface SshConfigEntry {
    host: string;
    hostname?: string;
    port?: number;
    user?: string;
    identityFile?: string;
}

export interface ConnectionStatus {
    connected: boolean;
    serverInfo?: string;
    error?: string;
}

export interface SavedHost {
    id: string;
    name: string;
    config: Omit<SshConfig, "authMethod"> & {
        authType: AuthMethod["type"];
    };
    lastUsed?: string;
}
