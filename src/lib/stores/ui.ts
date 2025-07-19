import { writable } from 'svelte/store';

// Sidebar width in px; 0 = collapsed on small screens
export const sidebarWidth = writable(256);

export const selectedNoteId = writable<string | undefined>(undefined);

export interface Notification {
  id: string;
  message: string;
  type: 'success' | 'error' | 'info';
  timeout?: number; // in ms
}

export const notifications = writable<Notification[]>([]);

export function addNotification(notification: Omit<Notification, 'id'>) {
  const id = Math.random().toString(36).substring(2, 9); // Simple unique ID
  notifications.update(n => [...n, { ...notification, id }]);

  if (notification.timeout) {
    setTimeout(() => {
      removeNotification(id);
    }, notification.timeout);
  }
}

export function removeNotification(id: string) {
  notifications.update(n => n.filter(notif => notif.id !== id));
}
