<script lang="ts">
  import KeyValueEditor from '../shared/KeyValueEditor.svelte';
  import JsonHighlight from '../shared/JsonHighlight.svelte';
  import { activeRequestStore } from '../../stores/activeRequest.svelte';
  import { exportFile } from '../../api/tauri';
  import type { KeyValuePair } from '../../api/tauri';

  let bodyType = $derived(activeRequestStore.activeRequest?.body_type ?? 'none');
  let bodyContent = $derived(activeRequestStore.activeRequest?.body ?? '');

  let textareaEl = $state<HTMLTextAreaElement | null>(null);
  let preEl = $state<HTMLElement | null>(null);
  let copyLabel = $state('Copy');

  function handleTypeChange(e: Event) {
    const val = (e.target as HTMLSelectElement).value;
    activeRequestStore.updateField('body_type', val);
    if (val === 'none') {
      activeRequestStore.updateField('body', '');
    }
  }

  function handleBodyInput(e: Event) {
    const val = (e.target as HTMLTextAreaElement).value;
    activeRequestStore.updateField('body', val);
  }

  function syncScroll() {
    if (textareaEl && preEl) {
      preEl.scrollTop = textareaEl.scrollTop;
      preEl.scrollLeft = textareaEl.scrollLeft;
    }
  }

  let formPairs = $derived.by<KeyValuePair[]>(() => {
    if (bodyType !== 'form-urlencoded') return [];
    try {
      return JSON.parse(bodyContent);
    } catch {
      return [];
    }
  });

  function handleFormChange(updated: KeyValuePair[]) {
    activeRequestStore.updateField('body', JSON.stringify(updated));
  }

  let displayContent = $derived(bodyContent || ' ');

  let hasContent = $derived(bodyType !== 'none' && bodyContent.trim().length > 0);

  async function handleCopy() {
    if (!bodyContent) return;
    await navigator.clipboard.writeText(bodyContent);
    copyLabel = 'Copied!';
    setTimeout(() => (copyLabel = 'Copy'), 1500);
  }

  function timestamp(): string {
    const d = new Date();
    const pad = (n: number) => String(n).padStart(2, '0');
    return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}_${pad(d.getHours())}.${pad(d.getMinutes())}.${pad(d.getSeconds())}`;
  }

  async function handleExport() {
    if (!bodyContent) return;
    const ext = bodyType === 'json' ? 'json' : 'txt';
    const name = `body_${timestamp()}.${ext}`;
    const filters = [{ name: ext.toUpperCase(), extensions: [ext] }];
    if (ext !== 'txt') filters.push({ name: 'Text', extensions: ['txt'] });
    await exportFile(bodyContent, name, filters);
  }
</script>

<div class="body-editor">
  <div class="body-type-row">
    <label class="body-label" for="body-type-select">Content Type</label>
    <select id="body-type-select" class="body-select" value={bodyType} onchange={handleTypeChange}>
      <option value="none">None</option>
      <option value="json">JSON</option>
      <option value="text">Text</option>
      <option value="form-urlencoded">Form URL-Encoded</option>
    </select>
    {#if hasContent}
      <div class="body-actions">
        <button class="action-btn" onclick={handleCopy}>{copyLabel}</button>
        <button class="action-btn" onclick={handleExport}>Export</button>
      </div>
    {/if}
  </div>

  {#if bodyType === 'none'}
    <div class="body-none">This request does not have a body</div>
  {:else if bodyType === 'json'}
    <div class="editor-container">
      <div class="highlight-layer" bind:this={preEl}>
        <JsonHighlight json={displayContent} />
      </div>
      <textarea
        class="editor-textarea"
        bind:this={textareaEl}
        placeholder={'{\n  "key": "value"\n}'}
        value={bodyContent}
        oninput={handleBodyInput}
        onscroll={syncScroll}
        spellcheck="false"
      ></textarea>
    </div>
  {:else if bodyType === 'text'}
    <textarea
      class="body-textarea"
      placeholder="Enter text body..."
      value={bodyContent}
      oninput={handleBodyInput}
      spellcheck="false"
    ></textarea>
  {:else if bodyType === 'form-urlencoded'}
    <KeyValueEditor pairs={formPairs} onchange={handleFormChange} />
  {/if}
</div>

<style>
  .body-editor {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .body-type-row {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .body-label {
    font-size: 12px;
    color: var(--color-text-secondary);
    min-width: 90px;
    font-weight: 500;
  }

  .body-select {
    padding: 6px 28px 6px 10px;
    font-size: 12px;
    min-width: clamp(120px, 20%, 200px);
  }

  .body-actions {
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

  .body-none {
    color: var(--color-text-secondary);
    font-size: 12px;
    padding: 8px 0;
  }

  .editor-container {
    position: relative;
    min-height: 180px;
    border: 1px solid var(--color-border);
    border-radius: 6px;
    background: var(--color-bg-input);
    overflow: hidden;
  }

  .highlight-layer {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    padding: 10px;
    overflow: hidden;
    pointer-events: none;
    z-index: 1;
  }

  .editor-textarea {
    position: relative;
    width: 100%;
    min-height: 180px;
    z-index: 2;
    resize: vertical;
    padding: 10px;
    font-family: 'Consolas', 'Courier New', monospace;
    font-size: 12px;
    line-height: 1.6;
    background: transparent;
    border: none;
    border-radius: 6px;
    color: transparent;
    -webkit-text-fill-color: transparent;
    caret-color: var(--color-text-primary);
    tab-size: 2;
    outline: none;
  }

  .editor-textarea::placeholder {
    color: var(--color-text-secondary);
  }

  .editor-textarea:focus {
    outline: none;
  }

  .body-textarea {
    width: 100%;
    min-height: 180px;
    resize: vertical;
    padding: 10px;
    font-family: 'Consolas', 'Courier New', monospace;
    font-size: 12px;
    line-height: 1.5;
    background: var(--color-bg-input);
    border: 1px solid var(--color-border);
    border-radius: 6px;
    color: var(--color-text-primary);
    tab-size: 2;
  }

  .body-textarea::placeholder {
    color: var(--color-text-secondary);
  }
</style>
