<script lang="ts">
  import KeyValueEditor from '../shared/KeyValueEditor.svelte';
  import { activeRequestStore } from '../../stores/activeRequest.svelte';
  import type { KeyValuePair } from '../../api/tauri';

  let bodyType = $derived(activeRequestStore.activeRequest?.body_type ?? 'none');
  let bodyContent = $derived(activeRequestStore.activeRequest?.body ?? '');

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

  let formPairs = $derived.by<KeyValuePair[]>(() => {
    if (bodyType !== 'form') return [];
    try {
      return JSON.parse(bodyContent);
    } catch {
      return [];
    }
  });

  function handleFormChange(updated: KeyValuePair[]) {
    activeRequestStore.updateField('body', JSON.stringify(updated));
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
  </div>

  {#if bodyType === 'none'}
    <div class="body-none">This request does not have a body</div>
  {:else if bodyType === 'json' || bodyType === 'text'}
    <textarea
      class="body-textarea"
      placeholder={bodyType === 'json' ? '{\n  "key": "value"\n}' : 'Enter text body...'}
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
    min-width: 180px;
  }

  .body-none {
    color: var(--color-text-secondary);
    font-size: 12px;
    padding: 8px 0;
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
