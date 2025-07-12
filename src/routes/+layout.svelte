<script>
  import '$lib/stores/theme'; // runs once to set dark class
  import './styles.css';
</script>

<svelte:head>
  <title>NexusNotes</title>
</svelte:head>

<div class="h-screen flex flex-col bg-white dark:bg-gray-950">
  <slot />
</div>
