<script lang="ts">
  import { onMount } from 'svelte';
  import { loadNotebooks, notebooks, createNotebook, moveNotebook } from '$lib/stores/notebooks';
  import TreeNode from '$lib/components/TreeNode.svelte';
  import { Plus } from 'lucide-svelte';

  onMount(loadNotebooks);

  function addRootNotebook() {
    const title = prompt('New notebook name:');
    if (title) createNotebook(title);
  }

  async function handleDrop(draggedId: string, ontoId: string) {
    await moveNotebook(draggedId, ontoId);
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

  <ul class="flex-1 overflow-y-auto p-2 space-y-1">
    {#each $notebooks.filter(n => !n.parent_id) as root (root.id)}
      <TreeNode node={root} depth={0} onDrop={handleDrop} />
    {/each}
  </ul>
</aside>
