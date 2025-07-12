<script lang="ts">
  export let title = 'Untitled';
</script>

<main class="flex-1 flex flex-col bg-white dark:bg-gray-950">
  <header class="border-b border-gray-200 dark:border-gray-700 px-4 py-2">
    <input
      type="text"
      class="w-full bg-transparent focus:outline-none text-lg font-semibold"
      bind:value={title}
    />
  </header>

  <!-- Tiptap placeholder -->
  <div class="flex-1 p-4">
    <div contenteditable="true" class="prose dark:prose-invert max-w-none">
      Start writing in Markdown…
    </div>
  </div>
</main>
