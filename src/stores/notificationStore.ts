import { defineStore } from "pinia";
import { ref } from "vue";

export type NotificationType = "success" | "error" | "warning" | "info";

export interface Notification {
  id: number;
  type: NotificationType;
  title: string;
  message?: string;
  duration?: number;
}

let notificationId = 0;

export const useNotificationStore = defineStore("notification", () => {
  const notifications = ref<Notification[]>([]);

  function addNotification(type: NotificationType, title: string, message?: string, duration = 5000) {
    const id = ++notificationId;
    const notification: Notification = { id, type, title, message, duration };
    notifications.value.push(notification);

    if (duration > 0) {
      setTimeout(() => removeNotification(id), duration);
    }

    return id;
  }

  function removeNotification(id: number) {
    const index = notifications.value.findIndex((n) => n.id === id);
    if (index !== -1) {
      notifications.value.splice(index, 1);
    }
  }

  function success(title: string, message?: string, duration?: number) {
    return addNotification("success", title, message, duration);
  }

  function error(title: string, message?: string, duration?: number) {
    return addNotification("error", title, message, duration ?? 8000);
  }

  function warning(title: string, message?: string, duration?: number) {
    return addNotification("warning", title, message, duration);
  }

  function info(title: string, message?: string, duration?: number) {
    return addNotification("info", title, message, duration);
  }

  return {
    notifications,
    addNotification,
    removeNotification,
    success,
    error,
    warning,
    info,
  };
});
