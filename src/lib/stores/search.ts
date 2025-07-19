import { invoke } from '@tauri-apps/api/core';
import { writable } from 'svelte/store';
import type { Note } from './notes';
import type { Notebook } from './notebooks';

export type SearchResult = { type: 'note'; data: Note & { title: string } } | { type: 'notebook'; data: Notebook & { title: string } };

export const searchResults = writable<SearchResult[]>([]);
export const searchQuery = writable<string>('');

import { addNotification } from './ui';

export async function search(query: string) {
  try {
    const results: SearchResult[] = await invoke('search_notes_and_notebooks', { query });
    searchResults.set(results);
    addNotification({ message: `Search completed for "${query}"`, type: 'info', timeout: 3000 });
  } catch (error) {
    addNotification({ message: `Failed to search: ${error}`, type: 'error', timeout: 5000 });
    searchResults.set([]);
  }
}