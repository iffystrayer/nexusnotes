<script lang="ts">
  import { SunIcon, MoonIcon } from 'heroicons/24/outline';
  import { toggleTheme } from '$lib/stores/theme';
</script>

<aside class="w-72 flex flex-col bg-gray-50 dark:bg-gray-900 border-l border-gray-200 dark:border-gray-700">
  <header class="flex items-center justify-between p-3 border-b">
    <span class="font-semibold text-sm">Context</span>
    <button on:click={toggleTheme} class="p-1 rounded hover:bg-gray-200 dark:hover:bg-gray-700">
      <SunIcon class="h-5 w-5 dark:hidden" /> <MoonIcon class="h-5 w-5 hidden dark:block" />
    </button>
  </header>
  <div class="flex-1 p-3 text-sm text-gray-600 dark:text-gray-400">Backlinks & suggestions will appear here.</div>
</aside>
