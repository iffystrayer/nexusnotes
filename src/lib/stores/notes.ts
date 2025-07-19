import { invoke } from '@tauri-apps/api/core';
import { writable } from 'svelte/store';

export interface Note {
  id: string;
  notebook_id: string;
  title: string;
  markdown: string;
  priority: number;
  date?: string;
  created_at: string;
  updated_at: string;
}

export const notes = writable<Note[]>([]);

import { addNotification } from './ui';

export async function loadNotes(notebookId: string) {
  try {
    const data: Note[] = await invoke('get_notes', { notebook_id: notebookId });
    notes.set(data);
  } catch (error) {
    addNotification({ message: `Failed to load notes: ${error}`, type: 'error', timeout: 5000 });
  }
}

export async function createNote(notebookId: string, title: string, markdown: string = '') {
  try {
    const note: Note = await invoke('create_note', {
      payload: { title, notebook_id: notebookId, markdown }
    });
    notes.update(list => [note, ...list]);
    addNotification({ message: `Note "${title}" created successfully!`, type: 'success', timeout: 3000 });
    return note;
  } catch (error) {
    addNotification({ message: `Failed to create note: ${error}`, type: 'error', timeout: 5000 });
    throw error;
  }
}

export async function updateNote(id: string, title: string, markdown: string, priority: number, date?: string) {
  try {
    await invoke('update_note', { id, title, markdown, priority, date });
    notes.update(list =>
      list.map(n => (n.id === id ? { ...n, title, markdown, priority, date } : n))
    );
    addNotification({ message: `Note "${title}" updated successfully!`, type: 'success', timeout: 3000 });
  } catch (error) {
    addNotification({ message: `Failed to update note: ${error}`, type: 'error', timeout: 5000 });
  }
}

export async function deleteNote(id: string) {
  try {
    await invoke('delete_note', { id });
    notes.update(list => list.filter(n => n.id !== id));
    addNotification({ message: `Note deleted successfully!`, type: 'success', timeout: 3000 });
  } catch (error) {
    addNotification({ message: `Failed to delete note: ${error}`, type: 'error', timeout: 5000 });
  }
}
