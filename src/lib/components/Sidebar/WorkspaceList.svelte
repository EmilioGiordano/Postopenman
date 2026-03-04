<script lang="ts">
  import { workspaceStore } from '../../stores/workspaces.svelte';

  let creating = $state(false);
  let newName = $state('');

  async function handleSelect(e: Event) {
    const target = e.target as HTMLSelectElement;
    await workspaceStore.selectWorkspace(target.value);
  }

  async function handleCreate() {
    if (newName.trim()) {
      await workspaceStore.createWorkspace(newName.trim());
      newName = '';
    }
    creating = false;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') handleCreate();
    if (e.key === 'Escape') {
      creating = false;
      newName = '';
    }
  }
</script>

<div class="workspace-section">
  <div class="workspace-row">
    {#if workspaceStore.workspaces.length > 0}
      <select
        class="workspace-select"
        value={workspaceStore.activeWorkspaceId ?? ''}
        onchange={handleSelect}
      >
        {#each workspaceStore.workspaces as ws}
          <option value={ws.id}>{ws.name}</option>
        {/each}
      </select>
    {:else}
      <span class="no-workspaces">No workspaces</span>
    {/if}
    <button class="add-ws-btn" onclick={() => (creating = true)} title="New workspace">+</button>
  </div>

  {#if creating}
    <div class="create-row">
      <input
        class="create-input"
        type="text"
        placeholder="Workspace name"
        bind:value={newName}
        onkeydown={handleKeydown}
      />
      <button class="confirm-btn" onclick={handleCreate}>OK</button>
    </div>
  {/if}
</div>

<style>
  .workspace-section {
    padding: 8px 10px;
    border-bottom: 1px solid var(--color-border);
  }

  .workspace-row {
    display: flex;
    gap: 6px;
    align-items: center;
  }

  .workspace-select {
    flex: 1;
    font-size: 12px;
    padding: 5px 28px 5px 8px;
    background: var(--color-bg-input);
    border: 1px solid var(--color-border);
    border-radius: 4px;
    color: var(--color-text-primary);
  }

  .no-workspaces {
    flex: 1;
    color: var(--color-text-secondary);
    font-size: 12px;
    padding: 5px 0;
  }

  .add-ws-btn {
    width: 28px;
    height: 28px;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: 1px solid var(--color-border);
    color: var(--color-text-secondary);
    border-radius: 4px;
    font-size: 16px;
    cursor: pointer;
  }

  .add-ws-btn:hover {
    background: var(--color-hover);
    color: var(--color-text-primary);
  }

  .create-row {
    display: flex;
    gap: 6px;
    margin-top: 6px;
  }

  .create-input {
    flex: 1;
    padding: 4px 8px;
    font-size: 12px;
  }

  .confirm-btn {
    padding: 4px 10px;
    background: var(--color-button);
    border: none;
    color: var(--color-text-primary);
    font-size: 12px;
    border-radius: 4px;
    cursor: pointer;
  }

  .confirm-btn:hover {
    filter: brightness(1.15);
  }
</style>
