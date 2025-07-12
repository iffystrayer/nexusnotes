<script lang="ts">
  import { Bars3Icon } from 'heroicons/24/outline';
  import { sidebarWidth } from '$lib/stores/ui';

  const toggle = () => sidebarWidth.update(w => (w === 0 ? 256 : 0));
</script>

<aside
  class="bg-gray-100 dark:bg-gray-900 border-r border-gray-200 dark:border-gray-700
         transition-all duration-200 ease-in-out"
  style="width: {$sidebarWidth}px"
>
  <header class="flex items-center justify-between p-3 border-b">
    <span class="font-bold text-sm">Nexus Tree</span>
    <button on:click={toggle} class="lg:hidden">
      <Bars3Icon class="h-5 w-5" />
    </button>
  </header>

  <nav class="p-2 space-y-1">
    <!-- Placeholder tree nodes -->
    <div class="px-2 py-1 text-sm rounded hover:bg-gray-200 dark:hover:bg-gray-800 cursor-pointer">
      📁 Personal
    </div>
    <div class="px-2 py-1 ml-4 text-sm rounded hover:bg-gray-200 dark:hover:bg-gray-800 cursor-pointer">
      📝 Untitled
    </div>
  </nav>
</aside>
