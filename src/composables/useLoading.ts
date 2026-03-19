import { ref, computed } from "vue";

const loadingState = ref(false);
const loadingText = ref("");
let loadingCount = 0;

export function useLoading() {
  const isLoading = computed(() => loadingState.value);
  const text = computed(() => loadingText.value);

  function startLoading(text = "加载中...") {
    loadingCount++;
    loadingState.value = true;
    loadingText.value = text;
  }

  function stopLoading() {
    loadingCount--;
    if (loadingCount <= 0) {
      loadingCount = 0;
      loadingState.value = false;
      loadingText.value = "";
    }
  }

  function setLoadingText(text: string) {
    loadingText.value = text;
  }

  return {
    isLoading,
    text,
    startLoading,
    stopLoading,
    setLoadingText,
  };
}
