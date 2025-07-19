<script lang="ts">
  import { onMount } from 'svelte';
  import { loadNotebooks, notebooks, createNotebook, moveNotebook } from '$lib/stores/notebooks';
  import { search, searchResults, searchQuery, type SearchResult } from '$lib/stores/search';
  import { theme, toggleTheme } from '$lib/stores/theme';
  import { addNotification } from '$lib/stores/ui';
  import TreeNode from '$lib/components/TreeNode.svelte';
  import { Plus, Search, Sun, Moon } from 'lucide-svelte';
  import { goto } from '$app/navigation';

  onMount(loadNotebooks);

  function addRootNotebook() {
    const title = prompt('New notebook name:');
    if (title) {
      createNotebook(title);
    } else {
      addNotification({ message: 'Notebook creation cancelled.', type: 'info', timeout: 3000 });
    }
  }

  async function handleDrop(draggedId: string, ontoId: string) {
    try {
      await moveNotebook(draggedId, ontoId);
    } catch (error) {
      addNotification({ message: `Failed to move notebook: ${error}`, type: 'error', timeout: 5000 });
    }
  }

  let searchDebounce: ReturnType<typeof setTimeout>;
  $: if ($searchQuery) {
    clearTimeout(searchDebounce);
    searchDebounce = setTimeout(() => {
      search($searchQuery);
    }, 300);
  } else {
    $searchResults = []; // Clear results if query is empty
  }

  function handleSearchResultClick(result: SearchResult) {
    if (result.type === 'note') {
      goto(`/note/${result.data.id}`); // This needs to be handled by the router
      addNotification({ message: `Navigating to note: ${result.data.title}`, type: 'info', timeout: 3000 });
    } else if (result.type === 'notebook') {
      addNotification({ message: `Clicked notebook search result: ${result.data.title}`, type: 'info', timeout: 3000 });
      $searchQuery = ''; // Clear search and focus on tree
    }
  }
</script>

<aside
  class="bg-gradient-to-b from-slate-50 to-slate-100 dark:from-slate-900 dark:to-slate-800
         w-80 flex flex-col shadow-lg"
>
  <header
    class="flex items-center justify-between p-3 border-b
           border-slate-300 dark:border-slate-700 bg-indigo-500 text-white"
  >
    <span class="font-bold text-sm">Nexus Tree</span>
    <div class="flex items-center space-x-2">
      <button
        on:click={toggleTheme}
        class="p-1 rounded hover:bg-indigo-600 transition-colors"
        title="Toggle theme"
      >
        {#if $theme === 'dark'}
          <Sun class="h-6 w-6" />
        {:else}
          <Moon class="h-6 w-6" />
        {/if}
      </button>
      <button
        on:click={addRootNotebook}
        class="p-1 rounded hover:bg-indigo-600 transition-colors"
        title="Add root notebook"
      >
        <Plus class="h-6 w-6" />
      </button>
    </div>
  </header>

  <div class="p-3 border-b border-gray-200 dark:border-gray-700">
    <div class="relative">
      <input
        type="text"
        bind:value={$searchQuery}
        placeholder="Search notes and notebooks..."
        class="w-full pl-8 pr-2 py-1 rounded bg-gray-100 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 focus:outline-none focus:ring-1 focus:ring-indigo-500 text-sm"
      />
      <Search class="absolute left-2 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" />
    </div>
  </div>

  {#if $searchQuery && $searchResults.length > 0}
    <div class="flex-1 overflow-y-auto p-2 space-y-1">
      <h3 class="text-xs font-semibold text-gray-500 uppercase mb-1">Search Results</h3>
      {#each $searchResults as result (result.data.id)}
        <div
          class="flex items-center space-x-2 rounded px-2 py-1 text-sm
                 hover:bg-indigo-100 dark:hover:bg-indigo-700/40 cursor-pointer transition-colors"
          on:click={() => handleSearchResultClick(result)}
        >
          <span class="w-4 text-center">
            {#if result.type === 'note'}📝{:else}📁{/if}
          </span>
          <span class="flex-1 text-gray-800 dark:text-gray-200">
            {result.data.title}
          </span>
        </div>
      {/each}
    </div>
  {:else if $searchQuery && $searchResults.length === 0}
    <div class="flex-1 p-2 text-center text-gray-500 text-sm">
      No results found for "{$searchQuery}".
    </div>
  {:else}
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
  {/if}
</aside>
