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

export async function loadNotes(notebookId: string) {
  try {
    const data: Note[] = await invoke('get_notes', { notebook_id: notebookId });
    notes.set(data);
  } catch (error) {
    console.error('Failed to load notes:', error);
  }
}

export async function createNote(notebookId: string, title: string, markdown: string = '') {
  try {
    const note: Note = await invoke('create_note', {
      payload: { title, notebook_id: notebookId, markdown }
    });
    notes.update(list => [note, ...list]);
    return note;
  } catch (error) {
    console.error('Failed to create note:', error);
    throw error;
  }
}

export async function updateNote(id: string, title: string, markdown: string) {
  try {
    await invoke('update_note', { id, title, markdown });
    notes.update(list =>
      list.map(n => (n.id === id ? { ...n, title, markdown } : n))
    );
  } catch (error) {
    console.error('Failed to update note:', error);
  }
}

export async function deleteNote(id: string) {
  try {
    await invoke('delete_note', { id });
    notes.update(list => list.filter(n => n.id !== id));
  } catch (error) {
    console.error('Failed to delete note:', error);
  }
}
