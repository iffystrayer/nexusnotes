<script lang="ts">
  import Sidebar from './Sidebar.svelte';
  import EditorPane from '$lib/components/EditorPane.svelte';
  import ContextPane from '$lib/components/ContextPane.svelte';
  import { Pane, Splitpanes } from 'svelte-splitpanes';
</script>

<Splitpanes theme="modern" class="h-full w-full">
  <Pane minSize={10} maxSize={40}>
    <Sidebar />
  </Pane>

  <Pane minSize={30}>
    <EditorPane />
  </Pane>

  <Pane minSize={15} maxSize={40}>
    <ContextPane />
  </Pane>
</Splitpanes>
