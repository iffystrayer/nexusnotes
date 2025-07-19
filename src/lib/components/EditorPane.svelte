<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { Editor } from '@tiptap/core';
  import StarterKit from '@tiptap/starter-kit';
  import { marked } from 'marked';
  import htmlToMd from 'html-to-md';
  import { updateNote } from '$lib/stores/notes';
  import { allTags, noteTags, loadAllTags, loadNoteTags, addTagToNote, removeTagFromNote } from '$lib/stores/tags';

  export let noteId: string;
  export let title = 'Untitled';
  export let markdown: string;
  export let priority: number;
  export let date: string | undefined;

  let editor: Editor;
  let editorElement: HTMLDivElement;
  let saveTimeout: ReturnType<typeof setTimeout>;
  let newTagInput = '';

  import { addNotification } from '$lib/stores/ui';

  const saveNote = () => {
    clearTimeout(saveTimeout);
    saveTimeout = setTimeout(async () => {
      try {
        await updateNote(noteId, title, markdown, priority, date);
        addNotification({ message: 'Note saved successfully!', type: 'success', timeout: 3000 });
      } catch (error) {
        addNotification({ message: `Failed to save note: ${error}`, type: 'error', timeout: 5000 });
      }
    }, 1000); // Save after 1 second of inactivity
  };

  onMount(() => {
    editor = new Editor({
      element: editorElement,
      extensions: [
        StarterKit,
      ],
      content: marked.parse(markdown), // Convert markdown to HTML for Tiptap
      onUpdate: ({ editor }) => {
        markdown = htmlToMd(editor.getHTML()); // Convert HTML back to markdown
        saveNote(); // Trigger save on content change
      },
    });

    // Custom renderer for links to open in external browser
    const renderer = new marked.Renderer();
    renderer.link = (href, title, text) => {
      // Use the imported 'open' function from @tauri-apps/plugin-opener
      return `<a href="#" onclick="event.preventDefault(); window.__TAURI_INVOKE__('plugin:opener|open', { path: '${href}' });" title="${title || ''}">${text}</a>`;
    };
    marked.setOptions({ renderer });
    loadAllTags();
  });

  onDestroy(() => {
    editor?.destroy();
    clearTimeout(saveTimeout); // Clear any pending save on destroy
  });

  // Reactively update editor content when markdown prop changes
  $: if (editor && markdown !== htmlToMd(editor.getHTML())) {
    editor.commands.setContent(marked.parse(markdown)); // Convert markdown to HTML
  }

  // Reactively save when title changes
  $: if (title) {
    saveNote();
  }

  // Load note-specific tags when noteId changes
  $: if (noteId) {
    loadNoteTags(noteId);
  }

  async function handleAddTag() {
    const tagName = newTagInput.trim();
    if (tagName && noteId) {
      try {
        await addTagToNote(noteId, tagName);
        newTagInput = ''; // Clear input
      } catch (error) {
        addNotification({ message: `Error adding tag: ${error}`, type: 'error', timeout: 5000 });
      }
    }
  }

  async function handleRemoveTag(tagName: string) {
    if (noteId) {
      try {
        await removeTagFromNote(noteId, tagName);
      } catch (error) {
        addNotification({ message: `Error removing tag: ${error}`, type: 'error', timeout: 5000 });
      }
    }
  }
</script>

<main class="flex-1 flex flex-col bg-white dark:bg-gray-950">
  <header class="border-b border-gray-200 dark:border-gray-700 px-4 py-2">
    <input
      type="text"
      class="w-full bg-transparent focus:outline-none text-lg font-semibold"
      bind:value={title}
    />
  </header>

  <div class="flex flex-col flex-1">
    <div class="flex items-center border-b border-gray-200 dark:border-gray-700 px-4 py-2 space-x-2">
      <button on:click={() => editor.chain().focus().toggleBold().run()} class="p-1 rounded" class:is-active={editor?.isActive('bold')}>
        Bold
      </button>
      <button on:click={() => editor.chain().focus().toggleItalic().run()} class="p-1 rounded" class:is-active={editor?.isActive('italic')}>
        Italic
      </button>
      <button on:click={() => editor.chain().focus().toggleHeading({ level: 1 }).run()} class="p-1 rounded" class:is-active={editor?.isActive('heading', { level: 1 })}>
        H1
      </button>
    </div>
    <div class="flex-1 p-4 overflow-y-auto">
      <div bind:this={editorElement} class="prose dark:prose-invert max-w-none" />
    </div>
  </div>

  <footer class="border-t border-gray-200 dark:border-gray-700 px-4 py-2">
    <div class="flex flex-wrap gap-2 mb-2">
      {#each $noteTags as tag (tag.id)}
        <span class="bg-blue-100 dark:bg-blue-800 text-blue-800 dark:text-blue-100 text-xs font-semibold px-2.5 py-0.5 rounded-full flex items-center">
          {tag.name}
          <button on:click={() => handleRemoveTag(tag.name)} class="ml-1 text-blue-800 dark:text-blue-100 hover:text-blue-900 dark:hover:text-blue-200">
            &times;
          </button>
        </span>
      {/each}
    </div>
    <div class="flex">
      <input
        type="text"
        bind:value={newTagInput}
        on:keydown={(e) => { if (e.key === 'Enter') handleAddTag(); }}
        placeholder="Add tag..."
        class="flex-1 bg-gray-100 dark:bg-gray-800 border border-gray-300 dark:border-gray-700 rounded-l px-3 py-1 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
      />
      <button on:click={handleAddTag} class="bg-indigo-500 hover:bg-indigo-600 text-white px-3 py-1 rounded-r text-sm">
        Add
      </button>
    </div>
  </footer>
</main>

<style>
  .is-active {
    background-color: #e0e0e0; /* Light gray for active state */
    color: #333;
  }
  .dark .is-active {
    background-color: #333; /* Darker gray for active state in dark mode */
    color: #e0e0e0;
  }
</style>
