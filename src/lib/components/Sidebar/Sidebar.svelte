<script lang="ts">
  import WorkspaceList from './WorkspaceList.svelte';
  import FolderTree from './FolderTree.svelte';
  import { workspaceStore } from '../../stores/workspaces.svelte';
  import { activeRequestStore } from '../../stores/activeRequest.svelte';

  async function handleNewFolder() {
    await workspaceStore.createFolder(null);
  }

  async function handleNewRequest() {
    const req = await workspaceStore.createRequest(null);
    if (req) {
      await activeRequestStore.selectRequest(req.id);
    }
  }
</script>

<div class="sidebar-container">
  <div class="sidebar-header">
    <span class="sidebar-title">Free Postman</span>
  </div>

  <WorkspaceList />

  <div class="sidebar-tree">
    {#if workspaceStore.activeWorkspaceId}
      <FolderTree />
    {:else}
      <div class="no-workspace">Create a workspace to begin</div>
    {/if}
  </div>

  {#if workspaceStore.activeWorkspaceId}
    <div class="sidebar-actions">
      <button class="action-btn" onclick={handleNewFolder}>+ Folder</button>
      <button class="action-btn" onclick={handleNewRequest}>+ Request</button>
    </div>
  {/if}
</div>

<style>
  .sidebar-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
  }

  .sidebar-header {
    padding: 14px 16px 10px;
    border-bottom: 1px solid var(--color-border);
  }

  .sidebar-title {
    font-size: 16px;
    font-weight: 700;
    color: var(--color-text-primary);
    letter-spacing: -0.3px;
  }

  .sidebar-tree {
    flex: 1;
    overflow-y: auto;
    padding: 6px 0;
  }

  .no-workspace {
    padding: 20px 16px;
    color: var(--color-text-secondary);
    font-size: 12px;
    text-align: center;
  }

  .sidebar-actions {
    display: flex;
    gap: 6px;
    padding: 8px 10px;
    border-top: 1px solid var(--color-border);
  }

  .action-btn {
    flex: 1;
    padding: 6px 8px;
    font-size: 12px;
    background: transparent;
    border: 1px solid var(--color-border);
    color: var(--color-text-secondary);
    border-radius: 4px;
    cursor: pointer;
    text-align: center;
  }

  .action-btn:hover {
    background: var(--color-hover);
    color: var(--color-text-primary);
  }
</style>
