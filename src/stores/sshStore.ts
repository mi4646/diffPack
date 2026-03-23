import { defineStore } from "pinia";
import { ref, computed } from "vue";
import type { SshConfig, SshConfigEntry, ConnectionStatus, SavedHost } from "@/types";
import * as sshService from "@/services/sshService";

export const useSshStore = defineStore("ssh", () => {
    // 状态
    const sessionId = ref<string | null>(null);
    const connectionStatus = ref<ConnectionStatus>({ connected: false });
    const currentConfig = ref<SshConfig | null>(null);
    const tempConfig = ref<Partial<SshConfig>>({
        host: "",
        port: 22,
        username: "",
        authMethod: { type: "password", password: "" }
    });
    const savedHosts = ref<SavedHost[]>([]);
    const sshConfigEntries = ref<SshConfigEntry[]>([]);
    const isLoading = ref(false);
    const error = ref<string | null>(null);

    // 计算属性
    const isConnected = computed(() => connectionStatus.value.connected);

    // Actions
    async function loadSshConfig() {
        isLoading.value = true;
        error.value = null;
        try {
            const entries = await sshService.parseSshConfig();
            sshConfigEntries.value = entries;
        } catch (e) {
            error.value = e instanceof Error ? e.message : String(e);
        } finally {
            isLoading.value = false;
        }
    }

    async function connect(config: SshConfig) {
        isLoading.value = true;
        error.value = null;
        try {
            const id = await sshService.connectSsh(config);
            sessionId.value = id;
            currentConfig.value = config;
            connectionStatus.value = { connected: true };
            return id;
        } catch (e) {
            connectionStatus.value = {
                connected: false,
                error: e instanceof Error ? e.message : String(e),
            };
            throw e;
        } finally {
            isLoading.value = false;
        }
    }

    async function testConnection(config: SshConfig) {
        isLoading.value = true;
        try {
            const status = await sshService.testSshConnection(config);
            return status;
        } finally {
            isLoading.value = false;
        }
    }

    async function disconnect() {
        if (!sessionId.value) return;
        isLoading.value = true;
        try {
            await sshService.disconnectSsh(sessionId.value);
            sessionId.value = null;
            currentConfig.value = null;
            connectionStatus.value = { connected: false };
        } catch (e) {
            error.value = e instanceof Error ? e.message : String(e);
            throw e;
        } finally {
            isLoading.value = false;
        }
    }

    function saveHost(host: SavedHost) {
        const index = savedHosts.value.findIndex((h) => h.id === host.id);
        if (index === -1) {
            savedHosts.value.push(host);
        } else {
            savedHosts.value[index] = host;
        }
        // 持久化到本地存储
        localStorage.setItem("savedHosts", JSON.stringify(savedHosts.value));
    }

    function removeHost(id: string) {
        savedHosts.value = savedHosts.value.filter((h) => h.id !== id);
        localStorage.setItem("savedHosts", JSON.stringify(savedHosts.value));
    }

    function loadSavedHosts() {
        const stored = localStorage.getItem("savedHosts");
        if (stored) {
            try {
                savedHosts.value = JSON.parse(stored);
            } catch {
                savedHosts.value = [];
            }
        }
    }

    function reset() {
        sessionId.value = null;
        connectionStatus.value = { connected: false };
        currentConfig.value = null;
        error.value = null;
    }

    return {
        // 状态
        sessionId,
        connectionStatus,
        currentConfig,
        tempConfig,
        savedHosts,
        sshConfigEntries,
        isLoading,
        error,
        // 计算属性
        isConnected,
        // Actions
        loadSshConfig,
        connect,
        testConnection,
        disconnect,
        saveHost,
        removeHost,
        loadSavedHosts,
        reset,
    };
});
