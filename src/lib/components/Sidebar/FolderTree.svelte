<script lang="ts">
  import TreeItem from './TreeItem.svelte';
  import { workspaceStore } from '../../stores/workspaces.svelte';

  let rootFolders = $derived(workspaceStore.rootFolders);
  let rootRequests = $derived(workspaceStore.rootRequests);
  let dragOverRoot = $state(false);

  function handleDragOver(e: DragEvent) {
    if (!e.dataTransfer?.types.includes('application/x-request-id')) return;
    e.preventDefault();
    e.dataTransfer.dropEffect = 'move';
    dragOverRoot = true;
  }

  function handleDragLeave(e: DragEvent) {
    const el = e.currentTarget as HTMLElement;
    const related = e.relatedTarget as HTMLElement | null;
    if (related && el.contains(related)) return;
    dragOverRoot = false;
  }

  async function handleDrop(e: DragEvent) {
    dragOverRoot = false;
    if (!e.dataTransfer) return;
    e.preventDefault();
    const requestId = e.dataTransfer.getData('application/x-request-id');
    if (requestId) {
      await workspaceStore.moveRequest(requestId, null);
    }
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="folder-tree"
  class:drag-over-root={dragOverRoot}
  ondragover={handleDragOver}
  ondragleave={handleDragLeave}
  ondrop={handleDrop}
>
  {#each rootFolders as folder (folder.id)}
    <TreeItem type="folder" item={folder} depth={0} />
  {/each}
  {#each rootRequests as request (request.id)}
    <TreeItem type="request" item={request} depth={0} />
  {/each}

  {#if rootFolders.length === 0 && rootRequests.length === 0}
    <div class="empty-tree">No items yet</div>
  {/if}
</div>

<style>
  .folder-tree {
    padding: 2px 0;
    min-height: 50px;
    flex: 1;
  }

  .folder-tree.drag-over-root {
    background: rgba(98, 114, 164, 0.1);
  }

  .empty-tree {
    padding: 16px;
    text-align: center;
    color: var(--color-text-secondary);
    font-size: 12px;
  }
</style>
