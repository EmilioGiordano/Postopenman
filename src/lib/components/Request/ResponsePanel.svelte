<script lang="ts">
  import { activeRequestStore } from '../../stores/activeRequest.svelte';
  import { exportFile } from '../../api/tauri';
  import JsonHighlight from '../shared/JsonHighlight.svelte';

  type ResponseTab = 'body' | 'headers';
  let activeTab = $state<ResponseTab>('body');
  let copyLabel = $state('Copy');

  let response = $derived(activeRequestStore.response);
  let loading = $derived(activeRequestStore.loading);

  let statusColor = $derived.by(() => {
    if (!response) return 'var(--color-text-secondary)';
    const s = response.status;
    if (s >= 200 && s < 300) return 'var(--color-success)';
    if (s >= 300 && s < 400) return 'var(--color-warning)';
    if (s >= 400) return 'var(--color-error)';
    return 'var(--color-text-secondary)';
  });

  let isJson = $derived.by(() => {
    if (!response) return false;
    try { JSON.parse(response.body); return true; } catch { return false; }
  });

  let isCsv = $derived.by(() => {
    if (!response) return false;
    const body = response.body.trim();
    if (!body) return false;
    const lines = body.split('\n');
    if (lines.length < 2) return false;
    const commaCount = (lines[0].match(/,/g) || []).length;
    if (commaCount === 0) return false;
    return lines.slice(1, 4).every(
      (line) => (line.match(/,/g) || []).length === commaCount
    );
  });

  let canExportCsv = $derived.by(() => {
    if (isCsv) return true;
    if (!isJson || !response) return false;
    try {
      const parsed = JSON.parse(response.body);
      return Array.isArray(parsed) && parsed.length > 0 && typeof parsed[0] === 'object' && parsed[0] !== null;
    } catch {
      return false;
    }
  });

  let formattedBody = $derived.by(() => {
    if (!response) return '';
    try {
      const parsed = JSON.parse(response.body);
      return JSON.stringify(parsed, null, 2);
    } catch {
      return response.body;
    }
  });

  function formatSize(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  }

  async function handleCopy() {
    if (!response) return;
    await navigator.clipboard.writeText(formattedBody);
    copyLabel = 'Copied!';
    setTimeout(() => (copyLabel = 'Copy'), 1500);
  }

  function jsonToCsv(data: Record<string, unknown>[]): string {
    if (data.length === 0) return '';
    const keys = Object.keys(data[0]);
    const escape = (val: unknown): string => {
      const str = val === null || val === undefined ? '' : String(val);
      return str.includes(',') || str.includes('"') || str.includes('\n')
        ? `"${str.replace(/"/g, '""')}"`
        : str;
    };
    const header = keys.map(escape).join(',');
    const rows = data.map((row) => keys.map((k) => escape(row[k])).join(','));
    return [header, ...rows].join('\n');
  }

  function timestamp(): string {
    const d = new Date();
    const pad = (n: number) => String(n).padStart(2, '0');
    return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}_${pad(d.getHours())}.${pad(d.getMinutes())}.${pad(d.getSeconds())}`;
  }

  async function handleExport(format?: 'csv') {
    if (!response) return;
    const ts = timestamp();

    if (format === 'csv') {
      let csvContent: string;
      if (isCsv) {
        csvContent = response.body;
      } else {
        const parsed = JSON.parse(response.body);
        csvContent = jsonToCsv(parsed);
      }
      await exportFile(csvContent, `response_${ts}.csv`, [{ name: 'CSV', extensions: ['csv'] }]);
      return;
    }

    const ext = isJson ? 'json' : 'txt';
    const filters = [{ name: ext.toUpperCase(), extensions: [ext] }];
    if (ext !== 'txt') filters.push({ name: 'Text', extensions: ['txt'] });
    await exportFile(formattedBody, `response_${ts}.${ext}`, filters);
  }
</script>

<div class="response-panel">
  {#if loading}
    <div class="response-loading">
      <div class="spinner"></div>
      <span>Sending request...</span>
    </div>
  {:else if response}
    <div class="response-header">
      <div class="response-meta">
        <span class="status-badge" style="color: {statusColor}">
          {response.status} {response.status_text}
        </span>
        <span class="meta-item">{response.time_ms} ms</span>
        <span class="meta-item">{formatSize(response.size_bytes)}</span>
      </div>
      <div class="response-actions-row">
        <div class="response-tabs">
          <button
            class="resp-tab"
            class:active={activeTab === 'body'}
            onclick={() => (activeTab = 'body')}
          >
            Body
          </button>
          <button
            class="resp-tab"
            class:active={activeTab === 'headers'}
            onclick={() => (activeTab = 'headers')}
          >
            Headers
            {#if response.headers.length > 0}
              <span class="header-count">({response.headers.length})</span>
            {/if}
          </button>
        </div>
        <div class="resp-actions">
          <button class="action-btn" onclick={handleCopy}>{copyLabel}</button>
          <button class="action-btn" onclick={() => handleExport()}>Export</button>
          {#if canExportCsv}
            <button class="action-btn" onclick={() => handleExport('csv')}>CSV</button>
          {/if}
        </div>
      </div>
    </div>

    <div class="response-body">
      {#if activeTab === 'body'}
        {#if isJson}
          <JsonHighlight json={formattedBody} />
        {:else}
          <pre class="response-pre">{formattedBody}</pre>
        {/if}
      {:else}
        <div class="response-headers-list">
          {#each response.headers as header}
            <div class="header-row">
              <span class="header-key">{header.key}</span>
              <span class="header-value">{header.value}</span>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {:else}
    <div class="response-empty">
      <span>Send a request to see the response</span>
    </div>
  {/if}
</div>

<style>
  .response-panel {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
  }

  .response-loading {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    height: 100%;
    color: var(--color-text-secondary);
    font-size: 13px;
  }

  .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid var(--color-border);
    border-top-color: var(--color-button);
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .response-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 16px;
    border-bottom: 1px solid var(--color-border);
    flex-shrink: 0;
    gap: 12px;
  }

  .response-meta {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .status-badge {
    font-weight: 700;
    font-size: 13px;
  }

  .meta-item {
    font-size: 12px;
    color: var(--color-text-secondary);
  }

  .response-actions-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .resp-actions {
    display: flex;
    gap: 6px;
    margin-left: auto;
  }

  .action-btn {
    padding: 4px 10px;
    font-size: 11px;
    background: transparent;
    border: 1px solid var(--color-border);
    color: var(--color-text-secondary);
    border-radius: 4px;
    cursor: pointer;
  }

  .action-btn:hover {
    background: var(--color-hover);
    color: var(--color-text-primary);
  }

  .response-tabs {
    display: flex;
    gap: 0;
  }

  .resp-tab {
    padding: 6px 14px;
    font-size: 12px;
    background: transparent;
    border: none;
    color: var(--color-text-secondary);
    cursor: pointer;
    border-bottom: 2px solid transparent;
    border-radius: 0;
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .resp-tab:hover {
    color: var(--color-text-primary);
    background: transparent;
  }

  .resp-tab.active {
    color: var(--color-text-primary);
    border-bottom-color: var(--color-button);
  }

  .header-count {
    font-size: 10px;
    color: var(--color-text-secondary);
  }

  .response-body {
    flex: 1;
    overflow: auto;
    padding: 12px 16px;
  }

  .response-pre {
    margin: 0;
    font-family: 'Consolas', 'Courier New', monospace;
    font-size: 12px;
    line-height: 1.6;
    color: var(--color-text-primary);
    white-space: pre-wrap;
    word-break: break-word;
  }

  .response-headers-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .header-row {
    display: flex;
    gap: 12px;
    padding: 4px 0;
    font-size: 12px;
    border-bottom: 1px solid var(--color-border);
  }

  .header-key {
    color: var(--color-info);
    font-weight: 600;
    min-width: clamp(100px, 25%, 220px);
    flex-shrink: 0;
  }

  .header-value {
    color: var(--color-text-primary);
    word-break: break-all;
  }

  .response-empty {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: var(--color-text-secondary);
    font-size: 13px;
  }
</style>
