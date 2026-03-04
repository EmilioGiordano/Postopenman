<script lang="ts">
  import Sidebar from './lib/components/Sidebar/Sidebar.svelte';
  import RequestPanel from './lib/components/Request/RequestPanel.svelte';
  import { activeRequestStore } from './lib/stores/activeRequest.svelte';
  import { workspaceStore } from './lib/stores/workspaces.svelte';

  $effect(() => {
    workspaceStore.loadWorkspaces();
  });
</script>

<div class="app-layout">
  <aside class="sidebar">
    <Sidebar />
  </aside>
  <main class="main-content">
    {#if activeRequestStore.activeRequest}
      <RequestPanel />
    {:else}
      <div class="empty-state">
        <div class="empty-logo">Free Postman</div>
        <p class="empty-hint">Select or create a request to get started</p>
      </div>
    {/if}
  </main>
</div>

<style>
  .app-layout {
    display: flex;
    height: 100vh;
    width: 100vw;
    overflow: hidden;
  }

  .sidebar {
    width: 280px;
    min-width: 280px;
    height: 100%;
    background: var(--color-bg-sidebar);
    border-right: 1px solid var(--color-border);
  }

  .main-content {
    flex: 1;
    height: 100%;
    overflow: hidden;
    background: var(--color-bg-main);
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    gap: 12px;
  }

  .empty-logo {
    font-size: 28px;
    font-weight: 700;
    color: var(--color-text-primary);
    letter-spacing: -0.5px;
  }

  .empty-hint {
    color: var(--color-text-secondary);
    font-size: 14px;
  }
</style>
