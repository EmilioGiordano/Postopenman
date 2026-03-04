<script lang="ts">
  import Sidebar from './lib/components/Sidebar/Sidebar.svelte';
  import RequestPanel from './lib/components/Request/RequestPanel.svelte';
  import { activeRequestStore } from './lib/stores/activeRequest.svelte';
  import { workspaceStore } from './lib/stores/workspaces.svelte';

  let zoomLevel = $state(100);

  $effect(() => {
    workspaceStore.loadWorkspaces();
  });

  $effect(() => {
    (document.documentElement.style as any).zoom = `${zoomLevel}%`;
  });

  $effect(() => {
    function handleWheel(e: WheelEvent) {
      if (!e.ctrlKey) return;
      e.preventDefault();
      e.stopPropagation();
      if (e.deltaY < 0) {
        zoomLevel = Math.min(200, zoomLevel + 10);
      } else {
        zoomLevel = Math.max(50, zoomLevel - 10);
      }
    }

    function handleKeydown(e: KeyboardEvent) {
      if (!e.ctrlKey) return;
      if (e.key === '=' || e.key === '+') {
        e.preventDefault();
        zoomLevel = Math.min(200, zoomLevel + 10);
      } else if (e.key === '-') {
        e.preventDefault();
        zoomLevel = Math.max(50, zoomLevel - 10);
      } else if (e.key === '0') {
        e.preventDefault();
        zoomLevel = 100;
      }
    }

    window.addEventListener('wheel', handleWheel, { capture: true, passive: false });
    window.addEventListener('keydown', handleKeydown, { capture: true });

    return () => {
      window.removeEventListener('wheel', handleWheel, { capture: true } as EventListenerOptions);
      window.removeEventListener('keydown', handleKeydown, { capture: true } as EventListenerOptions);
    };
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
    height: 100%;
    width: 100%;
    overflow: hidden;
  }

  .sidebar {
    width: clamp(200px, 20%, 320px);
    min-width: 200px;
    height: 100%;
    background: var(--color-bg-sidebar);
    border-right: 1px solid var(--color-border);
    flex-shrink: 0;
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
