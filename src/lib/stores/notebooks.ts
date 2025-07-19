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
import { addNotification } from './ui';

export async function loadNotebooks() {
  try {
    const data: Notebook[] = await invoke('get_notebooks');
    notebooks.set(data);
  } catch (error) {
    addNotification({ message: `Failed to load notebooks: ${error}`, type: 'error', timeout: 5000 });
  }
}

export async function createNotebook(title: string, parentId: string | null = null) {
  try {
    const nb: Notebook = await invoke('create_notebook', {
      payload: { title, parent_id: parentId, icon: '📁' }
    });
    notebooks.update(list => [...list, nb]);
    addNotification({ message: `Notebook "${title}" created successfully!`, type: 'success', timeout: 3000 });
  } catch (error) {
    addNotification({ message: `Failed to create notebook: ${error}`, type: 'error', timeout: 5000 });
  }
}

export async function renameNotebook(id: string, title: string) {
  try {
    await invoke('rename_notebook', { id, title });
    notebooks.update(list =>
      list.map(n => (n.id === id ? { ...n, title } : n))
    );
    await loadNotebooks(); // refresh to get accurate data
    addNotification({ message: `Notebook renamed to "${title}" successfully!`, type: 'success', timeout: 3000 });
  } catch (error) {
    addNotification({ message: `Failed to rename notebook: ${error}`, type: 'error', timeout: 5000 });
  }
}

export async function deleteNotebook(id: string) {
  try {
    await invoke('delete_notebook', { id });
    notebooks.update(list => list.filter(n => n.id !== id && !list.some(c => c.parent_id === id)));
    addNotification({ message: `Notebook deleted successfully!`, type: 'success', timeout: 3000 });
  } catch (error) {
    addNotification({ message: `Failed to delete notebook: ${error}`, type: 'error', timeout: 5000 });
  }
}

export async function moveNotebook(id: string, newParentId: string | null) {
  try {
    await invoke('move_notebook', { id, new_parent_id: newParentId });
    await loadNotebooks(); // refresh
    addNotification({ message: `Notebook moved successfully!`, type: 'success', timeout: 3000 });
  } catch (error) {
    addNotification({ message: `Failed to move notebook: ${error}`, type: 'error', timeout: 5000 });
  }
}
