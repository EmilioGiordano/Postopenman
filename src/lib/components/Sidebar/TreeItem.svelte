<script lang="ts">
  import type { Folder, SavedRequest } from '../../api/tauri';
  import { workspaceStore } from '../../stores/workspaces.svelte';
  import { activeRequestStore } from '../../stores/activeRequest.svelte';
  import ContextMenu from '../shared/ContextMenu.svelte';
  import TreeItem from './TreeItem.svelte';

  let { type, item, depth }: { type: 'folder' | 'request'; item: Folder | SavedRequest; depth: number } = $props();

  let expanded = $state(true);
  let editing = $state(false);
  let editName = $state('');
  let contextMenu = $state<{ x: number; y: number } | null>(null);

  let childFolders = $derived(
    type === 'folder' ? workspaceStore.getChildFolders(item.id) : []
  );

  let childRequests = $derived(
    type === 'folder' ? workspaceStore.getFolderRequests(item.id) : []
  );

  let isActive = $derived(
    type === 'request' && activeRequestStore.activeRequest?.id === item.id
  );

  const methodColors: Record<string, string> = {
    GET: 'var(--color-get)',
    POST: 'var(--color-post)',
    PUT: 'var(--color-put)',
    DELETE: 'var(--color-delete)',
    PATCH: 'var(--color-patch)',
    HEAD: 'var(--color-head)',
    OPTIONS: 'var(--color-options)',
  };

  function getMethodColor(method: string): string {
    return methodColors[method.toUpperCase()] ?? 'var(--color-text-secondary)';
  }

  function handleClick() {
    if (type === 'folder') {
      expanded = !expanded;
    } else {
      activeRequestStore.selectRequest(item.id);
    }
  }

  function handleDblClick() {
    editName = item.name;
    editing = true;
  }

  function handleContextMenu(e: MouseEvent) {
    e.preventDefault();
    contextMenu = { x: e.clientX, y: e.clientY };
  }

  async function handleRename() {
    if (editName.trim() && editName.trim() !== item.name) {
      if (type === 'folder') {
        await workspaceStore.renameFolder(item.id, editName.trim());
      } else {
        const req = item as SavedRequest;
        activeRequestStore.updateField('name', editName.trim());
        await activeRequestStore.save();
      }
    }
    editing = false;
  }

  function handleEditKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') handleRename();
    if (e.key === 'Escape') (editing = false);
  }

  async function handleDelete() {
    if (type === 'folder') {
      await workspaceStore.deleteFolder(item.id);
    } else {
      if (activeRequestStore.activeRequest?.id === item.id) {
        activeRequestStore.clear();
      }
      await workspaceStore.deleteRequest(item.id);
    }
  }

  async function handleNewRequest() {
    if (type !== 'folder') return;
    const req = await workspaceStore.createRequest(item.id);
    if (req) {
      expanded = true;
      await activeRequestStore.selectRequest(req.id);
    }
  }

  async function handleNewSubfolder() {
    if (type !== 'folder') return;
    await workspaceStore.createFolder(item.id);
    expanded = true;
  }

  let contextItems = $derived(
    type === 'folder'
      ? [
          { label: 'New Request', action: handleNewRequest },
          { label: 'New Subfolder', action: handleNewSubfolder },
          { label: 'Rename', action: () => { editName = item.name; editing = true; } },
          { label: 'Delete', action: handleDelete },
        ]
      : [
          { label: 'Rename', action: () => { editName = item.name; editing = true; } },
          { label: 'Delete', action: handleDelete },
        ]
  );
</script>

<div class="tree-item-wrapper">
  {#if editing}
    <div class="tree-row" style="padding-left: {depth * 16 + 10}px;">
      <input
        class="rename-input"
        type="text"
        bind:value={editName}
        onkeydown={handleEditKeydown}
        onblur={handleRename}
      />
    </div>
  {:else}
    <button
      class="tree-row"
      class:active={isActive}
      style="padding-left: {depth * 16 + 10}px;"
      onclick={handleClick}
      ondblclick={handleDblClick}
      oncontextmenu={handleContextMenu}
    >
      {#if type === 'folder'}
        <span class="arrow" class:expanded>{'\u25B6'}</span>
        <span class="folder-icon">{'\uD83D\uDCC1'}</span>
        <span class="item-name">{item.name}</span>
        <span class="child-count">{childFolders.length + childRequests.length}</span>
      {:else}
        {@const req = item as SavedRequest}
        <span class="method-badge" style="color: {getMethodColor(req.method)}">
          {req.method.length > 3 ? req.method.substring(0, 3) : req.method}
        </span>
        <span class="item-name">{item.name}</span>
      {/if}
    </button>
  {/if}

  {#if type === 'folder' && expanded}
    <div class="tree-children">
      {#each childFolders as folder (folder.id)}
        <TreeItem type="folder" item={folder} depth={depth + 1} />
      {/each}
      {#each childRequests as request (request.id)}
        <TreeItem type="request" item={request} depth={depth + 1} />
      {/each}
    </div>
  {/if}

  {#if contextMenu}
    <ContextMenu
      x={contextMenu.x}
      y={contextMenu.y}
      items={contextItems}
      onclose={() => (contextMenu = null)}
    />
  {/if}
</div>

<style>
  .tree-item-wrapper {
    user-select: none;
  }

  .tree-row {
    display: flex;
    align-items: center;
    gap: 6px;
    width: 100%;
    padding: 4px 10px;
    border: none;
    background: transparent;
    color: var(--color-text-primary);
    font-size: 12px;
    cursor: pointer;
    text-align: left;
    border-radius: 0;
    white-space: nowrap;
    overflow: hidden;
  }

  .tree-row:hover {
    background: var(--color-hover);
  }

  .tree-row.active {
    background: var(--color-selection);
  }

  .arrow {
    font-size: 8px;
    color: var(--color-text-secondary);
    transition: transform 0.15s ease;
    display: inline-block;
    width: 10px;
    text-align: center;
    flex-shrink: 0;
  }

  .arrow.expanded {
    transform: rotate(90deg);
  }

  .folder-icon {
    font-size: 12px;
    flex-shrink: 0;
  }

  .item-name {
    overflow: hidden;
    text-overflow: ellipsis;
    flex: 1;
  }

  .child-count {
    font-size: 10px;
    color: var(--color-text-secondary);
    background: var(--color-bg-main);
    padding: 0 5px;
    border-radius: 8px;
    flex-shrink: 0;
  }

  .method-badge {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.3px;
    min-width: 28px;
    text-align: center;
    flex-shrink: 0;
  }

  .rename-input {
    flex: 1;
    padding: 2px 6px;
    font-size: 12px;
    background: var(--color-bg-input);
    border: 1px solid var(--color-button);
    border-radius: 3px;
    color: var(--color-text-primary);
  }

  .tree-children {
    /* children inherit indentation via depth prop */
  }
</style>
