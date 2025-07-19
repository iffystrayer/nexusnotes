<script lang="ts">
  import { notifications, removeNotification } from '$lib/stores/ui';
  import { fade } from 'svelte/transition';
</script>

<div class="fixed bottom-4 right-4 z-50 space-y-2">
  {#each $notifications as notification (notification.id)}
    <div
      in:fade={{ duration: 150 }}
      out:fade={{ duration: 150 }}
      class="p-3 rounded-lg shadow-lg flex items-center justify-between text-white
        {notification.type === 'success' ? 'bg-green-500' : ''}
        {notification.type === 'error' ? 'bg-red-500' : ''}
        {notification.type === 'info' ? 'bg-blue-500' : ''}"
      role="alert"
    >
      <span>{notification.message}</span>
      <button on:click={() => removeNotification(notification.id)} class="ml-4 text-white opacity-75 hover:opacity-100">
        &times;
      </button>
    </div>
  {/each}
</div>