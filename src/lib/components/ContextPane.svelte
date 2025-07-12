<script lang="ts">
  import { Sun, Moon } from 'lucide-svelte';
  import { toggleTheme } from '$lib/stores/theme';
</script>

<aside class="w-72 flex flex-col bg-gray-50 dark:bg-gray-900 border-l border-gray-200 dark:border-gray-700">
  <header class="flex items-center justify-between p-3 border-b">
    <span class="font-semibold text-sm">Context</span>
    <button on:click={toggleTheme} class="p-1 rounded hover:bg-gray-200 dark:hover:bg-gray-700">
      <Sun class="h-5 w-5 dark:hidden" /> <Moon class="h-5 w-5 hidden dark:block" />
    </button>
  </header>
  <div class="flex-1 p-3 text-sm text-gray-600 dark:text-gray-400">Backlinks & suggestions will appear here.</div>
</aside>
