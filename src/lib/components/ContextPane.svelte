<script lang="ts">
  import { backlinks, loadBacklinks } from '$lib/stores/backlinks';
  import type { Note } from '$lib/stores/notes';
  import { addNotification } from '$lib/stores/ui';

  export let noteId: string | undefined;

  $: if (noteId) {
    loadBacklinks(noteId);
  }

  function handleBacklinkClick(note: Note) {
    // TODO: Implement navigation to the clicked note
    addNotification({ message: `Clicked backlink: ${note.title}`, type: 'info', timeout: 3000 });
  }
</script>

<aside class="w-72 flex flex-col bg-gray-50 dark:bg-gray-900 border-l border-gray-200 dark:border-gray-700">
  <header class="flex items-center justify-between p-3 border-b">
    <span class="font-semibold text-sm">Context</span>
  </header>
  <div class="flex-1 p-3 text-sm text-gray-600 dark:text-gray-400">
    {#if noteId}
      <h4 class="font-semibold text-gray-700 dark:text-gray-300 mb-2">Backlinks</h4>
      {#if $backlinks.length > 0}
        <ul class="space-y-1">
          {#each $backlinks as note (note.id)}
            <li>
              <button on:click={() => handleBacklinkClick(note)} class="text-indigo-600 dark:text-indigo-400 hover:underline">
                {note.title}
              </button>
            </li>
          {/each}
        </ul>
      {:else}
        <p>No backlinks found for this note.</p>
      {/if}
    {:else}
      <p>Select a note to see its context.</p>
    {/if}
  </div>
</aside>
