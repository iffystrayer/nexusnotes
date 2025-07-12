import { writable } from 'svelte/store';

const stored = typeof localStorage !== 'undefined' ? localStorage.getItem('theme') : null;
export const theme = writable<'light' | 'dark'>(stored === 'dark' ? 'dark' : 'light');

theme.subscribe((t) => {
  if (typeof document !== 'undefined') {
    document.documentElement.classList.toggle('dark', t === 'dark');
    localStorage.setItem('theme', t);
  }
});

export function toggleTheme() {
  theme.update((t) => (t === 'light' ? 'dark' : 'light'));
}
