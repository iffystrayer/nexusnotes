<script lang="ts">
  import { onMount } from 'svelte';
  import { loadNotebooks, notebooks, createNotebook } from '$lib/stores/notebooks';
  import TreeNode from '$lib/components/TreeNode.svelte';
  import { Plus } from 'lucide-svelte';

  onMount(loadNotebooks);

  function addRootNotebook() {
    const title = prompt('New notebook name:');
    if (title) createNotebook(title);
  }
</script>

<aside
  class="bg-gray-100 dark:bg-gray-900 border-r border-gray-200 dark:border-gray-700
         w-64 flex flex-col"
>
  <header
    class="flex items-center justify-between p-3 border-b border-gray-200 dark:border-gray-700"
  >
    <span class="font-bold text-sm">Nexus Tree</span>
    <button
      on:click={addRootNotebook}
      class="p-1 rounded hover:bg-gray-200 dark:hover:bg-gray-700"
      title="Add root notebook"
    >
      <Plus class="h-5 w-5" />
    </button>
  </header>

  <ul class="flex-1 overflow-y-auto p-2">
    {#each $notebooks.filter(n => !n.parent_id) as root (root.id)}
      <TreeNode node={root} depth={0} />
    {/each}
  </ul>
</aside>
