import { useNotificationStore } from "@/stores/notificationStore";

export function useNotifier() {
  const store = useNotificationStore();

  function success(title: string, message?: string) {
    return store.success(title, message);
  }

  function error(title: string, message?: string) {
    return store.error(title, message);
  }

  function warning(title: string, message?: string) {
    return store.warning(title, message);
  }

  function info(title: string, message?: string) {
    return store.info(title, message);
  }

  return {
    success,
    error,
    warning,
    info,
  };
}
