import { invoke } from '@tauri-apps/api/core';
import { writable } from 'svelte/store';
import type { Note } from './notes';

export const backlinks = writable<Note[]>([]);

import { addNotification } from './ui';

export async function loadBacklinks(noteId: string) {
  try {
    const data: Note[] = await invoke('get_backlinks', { note_id: noteId });
    backlinks.set(data);
  } catch (error) {
    addNotification({ message: `Failed to load backlinks: ${error}`, type: 'error', timeout: 5000 });
    backlinks.set([]);
  }
}