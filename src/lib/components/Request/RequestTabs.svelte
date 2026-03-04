<script lang="ts">
  import ParamsEditor from './ParamsEditor.svelte';
  import HeadersEditor from './HeadersEditor.svelte';
  import AuthEditor from './AuthEditor.svelte';
  import BodyEditor from './BodyEditor.svelte';
  import { activeRequestStore } from '../../stores/activeRequest.svelte';

  type TabName = 'params' | 'headers' | 'auth' | 'body';

  let activeTab = $state<TabName>('body');

  let paramCount = $derived(
    activeRequestStore.activeRequest?.params.filter((p) => p.key !== '').length ?? 0
  );

  let headerCount = $derived(
    activeRequestStore.activeRequest?.headers.filter((h) => h.key !== '').length ?? 0
  );

  const tabs: { id: TabName; label: string }[] = [
    { id: 'body', label: 'Body' },
    { id: 'params', label: 'Params' },
    { id: 'headers', label: 'Headers' },
    { id: 'auth', label: 'Auth' },
  ];

  function getBadge(tabId: TabName): number {
    if (tabId === 'params') return paramCount;
    if (tabId === 'headers') return headerCount;
    return 0;
  }
</script>

<div class="request-tabs">
  <div class="tab-bar">
    {#each tabs as tab}
      <button
        class="tab-btn"
        class:active={activeTab === tab.id}
        onclick={() => (activeTab = tab.id)}
      >
        {tab.label}
        {#if getBadge(tab.id) > 0}
          <span class="badge">{getBadge(tab.id)}</span>
        {/if}
      </button>
    {/each}
  </div>

  <div class="tab-content">
    {#if activeTab === 'params'}
      <ParamsEditor />
    {:else if activeTab === 'headers'}
      <HeadersEditor />
    {:else if activeTab === 'auth'}
      <AuthEditor />
    {:else if activeTab === 'body'}
      <BodyEditor />
    {/if}
  </div>
</div>

<style>
  .request-tabs {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .tab-bar {
    display: flex;
    gap: 0;
    border-bottom: 1px solid var(--color-border);
    padding: 0 16px;
    flex-shrink: 0;
  }

  .tab-btn {
    padding: 10px 16px;
    font-size: 12px;
    font-weight: 500;
    background: transparent;
    border: none;
    border-bottom: 2px solid transparent;
    color: var(--color-text-secondary);
    cursor: pointer;
    border-radius: 0;
    display: flex;
    align-items: center;
    gap: 6px;
    transition: color 0.15s ease, border-color 0.15s ease;
  }

  .tab-btn:hover {
    color: var(--color-text-primary);
    background: transparent;
  }

  .tab-btn.active {
    color: var(--color-text-primary);
    border-bottom-color: var(--color-button);
  }

  .badge {
    font-size: 10px;
    background: var(--color-button);
    color: var(--color-text-primary);
    padding: 1px 6px;
    border-radius: 8px;
    font-weight: 600;
  }

  .tab-content {
    flex: 1;
    overflow-y: auto;
    padding: 12px 16px;
  }
</style>
