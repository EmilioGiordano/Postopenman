<script lang="ts">
  import { activeRequestStore } from '../../stores/activeRequest.svelte';

  const methods = ['GET', 'POST', 'PUT', 'DELETE', 'PATCH', 'HEAD', 'OPTIONS'] as const;

  const methodColors: Record<string, string> = {
    GET: 'var(--color-get)',
    POST: 'var(--color-post)',
    PUT: 'var(--color-put)',
    DELETE: 'var(--color-delete)',
    PATCH: 'var(--color-patch)',
    HEAD: 'var(--color-head)',
    OPTIONS: 'var(--color-options)',
  };

  let currentMethod = $derived(activeRequestStore.activeRequest?.method ?? 'GET');
  let currentUrl = $derived(activeRequestStore.activeRequest?.url ?? '');

  function handleMethodChange(e: Event) {
    const val = (e.target as HTMLSelectElement).value;
    activeRequestStore.updateField('method', val);
  }

  function handleUrlInput(e: Event) {
    const val = (e.target as HTMLInputElement).value;
    activeRequestStore.updateField('url', val);
  }

  function handleSend() {
    activeRequestStore.send();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' && (e.ctrlKey || e.metaKey)) {
      e.preventDefault();
      handleSend();
    }
  }
</script>

<div class="url-bar">
  <select
    class="method-select"
    value={currentMethod}
    onchange={handleMethodChange}
    style="color: {methodColors[currentMethod] ?? 'var(--color-text-primary)'}"
  >
    {#each methods as m}
      <option value={m} style="color: {methodColors[m]}">{m}</option>
    {/each}
  </select>

  <input
    class="url-input"
    type="text"
    placeholder="Enter request URL..."
    value={currentUrl}
    oninput={handleUrlInput}
    onkeydown={handleKeydown}
  />

  <button
    class="send-btn"
    onclick={handleSend}
    disabled={activeRequestStore.loading}
  >
    {#if activeRequestStore.loading}
      Sending...
    {:else}
      Send
    {/if}
  </button>
</div>

<style>
  .url-bar {
    display: flex;
    gap: 8px;
    align-items: center;
  }

  .method-select {
    width: 110px;
    padding: 8px 28px 8px 10px;
    font-size: 13px;
    font-weight: 700;
    background: var(--color-bg-input);
    border: 1px solid var(--color-border);
    border-radius: 6px;
    flex-shrink: 0;
  }

  .url-input {
    flex: 1;
    padding: 8px 12px;
    font-size: 13px;
    background: var(--color-bg-input);
    border: 1px solid var(--color-border);
    border-radius: 6px;
    color: var(--color-text-primary);
  }

  .url-input::placeholder {
    color: var(--color-text-secondary);
  }

  .send-btn {
    padding: 8px 20px;
    font-size: 13px;
    font-weight: 600;
    background: var(--color-button);
    border: none;
    border-radius: 6px;
    color: var(--color-text-primary);
    cursor: pointer;
    flex-shrink: 0;
    min-width: 80px;
  }

  .send-btn:hover:not(:disabled) {
    filter: brightness(1.2);
  }

  .send-btn:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }
</style>
