import { invoke } from '@tauri-apps/api/core';
import { writable } from 'svelte/store';

export interface Tag {
  id: string;
  name: string;
}

export const allTags = writable<Tag[]>([]);
export const noteTags = writable<Tag[]>([]);

import { addNotification } from './ui';

export async function loadAllTags() {
  try {
    const data: Tag[] = await invoke('get_all_tags');
    allTags.set(data);
  } catch (error) {
    addNotification({ message: `Failed to load all tags: ${error}`, type: 'error', timeout: 5000 });
  }
}

export async function loadNoteTags(noteId: string) {
  try {
    const data: Tag[] = await invoke('get_tags_for_note', { note_id: noteId });
    noteTags.set(data);
  } catch (error) {
    addNotification({ message: `Failed to load note tags: ${error}`, type: 'error', timeout: 5000 });
  }
}

export async function addTagToNote(noteId: string, tagName: string) {
  try {
    await invoke('add_tag_to_note', { note_id: noteId, tag_name: tagName });
    await loadAllTags(); // Refresh all tags
    await loadNoteTags(noteId); // Refresh tags for the current note
    addNotification({ message: `Tag "${tagName}" added successfully!`, type: 'success', timeout: 3000 });
  } catch (error) {
    addNotification({ message: `Failed to add tag to note: ${error}`, type: 'error', timeout: 5000 });
    throw error;
  }
}

export async function removeTagFromNote(noteId: string, tagName: string) {
  try {
    await invoke('remove_tag_from_note', { note_id: noteId, tag_name: tagName });
    await loadAllTags(); // Refresh all tags
    await loadNoteTags(noteId); // Refresh tags for the current note
    addNotification({ message: `Tag "${tagName}" removed successfully!`, type: 'success', timeout: 3000 });
  } catch (error) {
    addNotification({ message: `Failed to remove tag from note: ${error}`, type: 'error', timeout: 5000 });
    throw error;
  }
}