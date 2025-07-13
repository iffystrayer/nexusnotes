<script lang="ts">
  import { onMount } from 'svelte';
  import { loadNotebooks, notebooks, createNotebook, moveNotebook } from '$lib/stores/notebooks';
  import TreeNode from '$lib/components/TreeNode.svelte';
  import { Plus } from 'lucide-svelte';

  onMount(loadNotebooks);

  // Debug: log notebooks when they change
  $: console.log('Notebooks in sidebar:', $notebooks);

  function addRootNotebook() {
    console.log('Add root notebook clicked');
    const title = prompt('New notebook name:');
    if (title) {
      console.log('Creating notebook with title:', title);
      createNotebook(title);
    }
  }

  async function handleDrop(draggedId: string, ontoId: string) {
    console.log('Sidebar handleDrop called with:', draggedId, 'onto:', ontoId);
    try {
      await moveNotebook(draggedId, ontoId);
      console.log('Move completed successfully');
    } catch (error) {
      console.error('Move failed:', error);
      alert('Failed to move notebook: ' + error);
    }
  }
</script>

<aside
  class="bg-gradient-to-b from-slate-50 to-slate-100 dark:from-slate-900 dark:to-slate-800
         w-72 flex flex-col shadow-lg"
>
  <header 
    class="flex items-center justify-between p-3 border-b
           border-slate-300 dark:border-slate-700 bg-indigo-500 text-white"
  >
    <span class="font-bold text-sm">Nexus Tree</span>
    <button
      on:click={addRootNotebook}
      class="p-1 rounded hover:bg-indigo-600 transition-colors"
      title="Add root notebook"
    >
      <Plus class="h-5 w-5" />
    </button>
  </header>

  <ul class="flex-1 overflow-y-auto p-2 space-y-1" role="tree" aria-label="Notebook tree">
    {#each $notebooks.filter(n => !n.parent_id) as root (root.id)}
      <TreeNode node={root} depth={0} onDrop={handleDrop} />
    {:else}
      <li class="text-gray-500 text-sm p-2 text-center">
        No notebooks yet.<br>
        Click the + button to create one!
      </li>
    {/each}
  </ul>
</aside>
