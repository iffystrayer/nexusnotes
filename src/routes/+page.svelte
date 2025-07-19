<script lang="ts">
  import Sidebar from './Sidebar.svelte';
  import EditorPane from '$lib/components/EditorPane.svelte';
  import ContextPane from '$lib/components/ContextPane.svelte';
  import { Pane, Splitpanes } from 'svelte-splitpanes';
  import { notes } from '$lib/stores/notes';
  import { selectedNoteId } from '$lib/stores/ui';
  import { get } from 'svelte/store';

  let currentNoteTitle: string = 'Untitled';
  let currentNoteMarkdown: string = '';
  let currentNotePriority: number = 0;
  let currentNoteDate: string | undefined = undefined;

  // Select the first note if available when notes store changes and no note is selected
  $: if ($notes.length > 0 && !$selectedNoteId) {
    $selectedNoteId = $notes[0].id;
  }

  // Update current note details when selectedNoteId changes
  $: if ($selectedNoteId) {
    const note = get(notes).find(n => n.id === $selectedNoteId);
    if (note) {
      currentNoteTitle = note.title;
      currentNoteMarkdown = note.markdown;
      currentNotePriority = note.priority;
      currentNoteDate = note.date;
    }
  }
</script>

<Splitpanes theme="modern" class="h-full w-full">
  <Pane minSize={10} maxSize={40}>
    <Sidebar />
  </Pane>

  <Pane minSize={30}>
    {#if $selectedNoteId}
      <EditorPane
        noteId={$selectedNoteId}
        bind:title={currentNoteTitle}
        bind:markdown={currentNoteMarkdown}
        bind:priority={currentNotePriority}
        bind:date={currentNoteDate}
      />
    {:else}
      <div class="flex-1 flex items-center justify-center text-gray-500 dark:text-gray-400">
        Select a note or create a new one.
      </div>
    {/if}
  </Pane>

  <Pane minSize={15} maxSize={40}>
    <ContextPane noteId={$selectedNoteId} />
  </Pane>
</Splitpanes>
