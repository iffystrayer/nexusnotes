<script lang="ts">
  import { dndzone } from 'svelte-dnd-action';
  import { flip } from 'svelte/animate';
  import { notebooks, moveNotebook, createNotebook, renameNotebook, deleteNotebook } from '$lib/stores/notebooks';
  import type { Notebook } from '$lib/stores/notebooks';

  export let node: Notebook;
  export let depth: number = 0;

  let editing = false;
  let newTitle = node.title;

  const children = () => $notebooks.filter(n => n.parent_id === node.id);

  function handleDrag(e: CustomEvent<{ info: any; items: Notebook[] }>) {
    const { items } = e.detail;
    // items is the reordered list; find new parent
    moveNotebook(node.id, items.find(i => i.id !== node.id)?.parent_id ?? null);
  }

  function commitRename() {
    if (newTitle.trim()) renameNotebook(node.id, newTitle.trim());
    editing = false;
  }

  function addChild() {
    const title = prompt('Notebook name:');
    if (title) createNotebook(title, node.id);
  }

  function remove() {
    if (confirm(`Delete "${node.title}" and all its children?`)) deleteNotebook(node.id);
  }
</script>

<li
  class="list-none group"
  use:dndzone={{ items: children(), flipDurationMs: 200 }}
  on:consider={handleDrag}
  on:finalize={handleDrag}
>
  <div
    class="flex items-center space-x-1 rounded px-2 py-1 text-sm
           hover:bg-gray-200 dark:hover:bg-gray-700 cursor-pointer"
    style:padding-left="{12 + depth * 16}px"
  >
    <span class="w-4 text-center">
      {node.icon || '📁'}
    </span>

    {#if editing}
      <input
        type="text"
        bind:value={newTitle}
        on:blur={commitRename}
        on:keydown={(e) => e.key === 'Enter' && commitRename()}
        class="flex-1 bg-transparent outline-none"
      />
    {:else}
      <span class="flex-1 select-none" on:dblclick={() => editing = true}>{node.title}</span>
    {/if}

    <button on:click={addChild} class="opacity-0 group-hover:opacity-100 text-xs" title="Add child">+</button>
    <button on:click={remove} class="opacity-0 group-hover:opacity-100 text-xs" title="Delete">🗑</button>
  </div>

  {#if children().length}
    <ul class="mt-1">
      {#each children() as child (child.id)}
        <svelte:self node={child} depth={depth + 1} />
      {/each}
    </ul>
  {/if}
</li>
