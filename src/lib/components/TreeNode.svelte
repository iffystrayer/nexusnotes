<script lang="ts">
  import { notebooks, moveNotebook, createNotebook, renameNotebook, deleteNotebook } from '$lib/stores/notebooks';
  import { createNote } from '$lib/stores/notes';
  import type { Notebook } from '$lib/stores/notebooks';
  import { Plus, Edit, Trash2 } from 'lucide-svelte';

  export let node: Notebook;
  export let depth: number = 0;
  export let onDrop: (draggedId: string, ontoId: string) => void;

  let editing = false;
  let newTitle = node.title;

  // Focus action for input element
  function focus(element: HTMLInputElement) {
    element.focus();
    element.select();
  }

  const children = () => $notebooks.filter(n => n.parent_id === node.id);

  async function commitRename() {
    if (newTitle.trim()) {
      await renameNotebook(node.id, newTitle.trim());
    }
    editing = false;
  }

  function addChild() {
    const title = prompt('Notebook name:');
    if (title) createNotebook(title, node.id);
  }

  async function addNote() {
    const title = prompt('Note title:');
    if (title) {
      try {
        await createNote(node.id, title);
      } catch (error) {
        alert('Failed to create note');
      }
    }
  }

  async function remove() {
    if (confirm(`Delete "${node.title}" and all its children?`)) {
      await deleteNotebook(node.id);
    }
  }

  function handleDropzone(e: DragEvent) {
    e.preventDefault();
    const draggedId = e.dataTransfer?.getData('text/plain');
    if (draggedId && draggedId !== node.id) {
      onDrop(draggedId, node.id);
    }
  }
</script>

<li class="list-none group" role="treeitem" aria-expanded={children().length > 0} aria-selected="false">
  <div
    class="flex items-center space-x-2 rounded px-2 py-1 text-sm
           hover:bg-indigo-100 dark:hover:bg-indigo-700/40 cursor-pointer transition-colors"
    style:padding-left="{12 + depth * 16}px"
    draggable="true"
    role="button"
    tabindex="0"
    aria-label="{node.title} - {children().length > 0 ? 'expandable folder' : 'folder'}"
    on:dragstart={(e) => e.dataTransfer?.setData('text/plain', node.id)}
    on:drop={handleDropzone}
    on:dragover={(e) => e.preventDefault()}
    on:keydown={(e) => {
      if (e.key === 'Enter' || e.key === ' ') {
        e.preventDefault();
        editing = true;
      }
    }}
  >
    <span class="w-4 text-center">
      {node.icon || '📁'}
    </span>

    {#if editing}
      <input
        type="text"
        bind:value={newTitle}
        on:blur={commitRename}
        on:keydown={(e) => {
          if (e.key === 'Enter') {
            commitRename();
          } else if (e.key === 'Escape') {
            editing = false;
            newTitle = node.title;
          }
        }}
        class="flex-1 bg-transparent outline-none text-indigo-700 dark:text-indigo-300"
        aria-label="Edit notebook name"
        use:focus
      />
    {:else}
      <span 
        class="flex-1 select-none cursor-pointer text-gray-800 dark:text-gray-200" 
        role="button"
        tabindex="0"
        aria-label="{node.title} - double click to edit"
        on:dblclick={() => editing = true}
        on:keydown={(e) => {
          if (e.key === 'Enter' || e.key === ' ') {
            e.preventDefault();
            editing = true;
          }
        }}
      >
        {node.title}
      </span>
    {/if}

    <div class="flex space-x-1 opacity-0 group-hover:opacity-100 transition-opacity" role="toolbar" aria-label="Notebook actions">
      <button 
        on:click={addChild} 
        class="p-1 rounded hover:bg-indigo-200 dark:hover:bg-indigo-600 text-indigo-600 dark:text-indigo-300"
        title="Add child notebook to {node.title}"
        aria-label="Add child notebook to {node.title}"
      >
        <Plus class="h-3 w-3" aria-hidden="true" />
      </button>
      <button 
        on:click={addNote} 
        class="p-1 rounded hover:bg-green-200 dark:hover:bg-green-600 text-green-600 dark:text-green-300"
        title="Add note to {node.title}"
        aria-label="Add note to {node.title}"
      >
        <Edit class="h-3 w-3" aria-hidden="true" />
      </button>
      <button 
        on:click={remove} 
        class="p-1 rounded hover:bg-red-200 dark:hover:bg-red-600 text-red-600 dark:text-red-300"
        title="Delete {node.title}"
        aria-label="Delete {node.title}"
      >
        <Trash2 class="h-3 w-3" aria-hidden="true" />
      </button>
    </div>
  </div>

  {#if children().length}
    <ul class="mt-1 border-l border-gray-200 dark:border-gray-700 ml-3" role="group">
      {#each children() as child (child.id)}
        <svelte:self node={child} depth={depth + 1} {onDrop} />
      {/each}
    </ul>
  {/if}
</li>
