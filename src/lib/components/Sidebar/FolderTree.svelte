<script lang="ts">
  import TreeItem from './TreeItem.svelte';
  import { workspaceStore } from '../../stores/workspaces.svelte';

  let rootFolders = $derived(workspaceStore.rootFolders);
  let rootRequests = $derived(workspaceStore.rootRequests);
</script>

<div class="folder-tree">
  {#each rootFolders as folder (folder.id)}
    <TreeItem type="folder" item={folder} depth={0} />
  {/each}
  {#each rootRequests as request (request.id)}
    <TreeItem type="request" item={request} depth={0} />
  {/each}
</div>

<style>
  .folder-tree {
    padding: 2px 0;
  }
</style>
