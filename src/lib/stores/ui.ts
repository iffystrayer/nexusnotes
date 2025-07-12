import { writable } from 'svelte/store';

// Sidebar width in px; 0 = collapsed on small screens
export const sidebarWidth = writable(256);
