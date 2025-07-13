import { writable } from 'svelte/store';
import { browser } from '$app/environment';

const stored = browser ? localStorage.getItem('theme') : null;
export const theme = writable<'light' | 'dark'>(stored === 'dark' ? 'dark' : 'light');

theme.subscribe((t) => {
  if (browser) {
    document.documentElement.classList.toggle('dark', t === 'dark');
    localStorage.setItem('theme', t);
    console.log('Theme changed to:', t);
  }
});

export function toggleTheme() {
  console.log('Toggle theme called');
  theme.update((t) => {
    const newTheme = t === 'light' ? 'dark' : 'light';
    console.log('Switching from', t, 'to', newTheme);
    return newTheme;
  });
}
