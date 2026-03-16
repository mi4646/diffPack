import { defineStore } from "pinia";
import { ref } from "vue";

const STORAGE_KEY = "appSettings";

function loadFromStorage(): { showFullCommitId: boolean } {
    try {
        const raw = localStorage.getItem(STORAGE_KEY);
        if (raw) return JSON.parse(raw);
    } catch {
        // ignore
    }
    return { showFullCommitId: false };
}

export const useAppStore = defineStore("app", () => {
    const saved = loadFromStorage();

    // 是否展示完整 Commit ID
    const showFullCommitId = ref<boolean>(saved.showFullCommitId);

    function saveToStorage() {
        localStorage.setItem(
            STORAGE_KEY,
            JSON.stringify({ showFullCommitId: showFullCommitId.value })
        );
    }

    function setShowFullCommitId(val: boolean) {
        showFullCommitId.value = val;
        saveToStorage();
    }

    return {
        showFullCommitId,
        setShowFullCommitId,
    };
});
