import { invoke } from '@tauri-apps/api/core';
import { writable } from 'svelte/store';

export interface Notebook {
  id: string;
  parent_id: string | null;
  title: string;
  icon?: string;
  sort_order: number;
}

export const notebooks = writable<Notebook[]>([]);

// Load on startup
export async function loadNotebooks() {
  try {
    const data: Notebook[] = await invoke('get_notebooks');
    notebooks.set(data);
  } catch (error) {
    console.error('Failed to load notebooks:', error);
  }
}

export async function createNotebook(title: string, parentId: string | null = null) {
  try {
    const nb: Notebook = await invoke('create_notebook', {
      payload: { title, parent_id: parentId, icon: '📁' }
    });
    notebooks.update(list => [...list, nb]);
  } catch (error) {
    console.error('Failed to create notebook:', error);
  }
}

export async function renameNotebook(id: string, title: string) {
  try {
    await invoke('rename_notebook', { id, title });
    notebooks.update(list =>
      list.map(n => (n.id === id ? { ...n, title } : n))
    );
    await loadNotebooks(); // refresh to get accurate data
  } catch (error) {
    console.error('Failed to rename notebook:', error);
  }
}

export async function deleteNotebook(id: string) {
  try {
    await invoke('delete_notebook', { id });
    notebooks.update(list => list.filter(n => n.id !== id && !list.some(c => c.parent_id === id)));
  } catch (error) {
    console.error('Failed to delete notebook:', error);
  }
}

export async function moveNotebook(id: string, newParentId: string | null) {
  try {
    await invoke('move_notebook', { id, new_parent_id: newParentId });
    await loadNotebooks(); // refresh
  } catch (error) {
    console.error('Failed to move notebook:', error);
  }
}
