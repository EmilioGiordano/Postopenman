<script lang="ts">
  import type { KeyValuePair } from '../../api/tauri';

  let { pairs, onchange }: { pairs: KeyValuePair[]; onchange: (pairs: KeyValuePair[]) => void } = $props();

  let localPairs = $state<KeyValuePair[]>([]);

  $effect(() => {
    const hasEmpty = pairs.some((p) => p.key === '' && p.value === '');
    if (hasEmpty) {
      localPairs = [...pairs];
    } else {
      localPairs = [...pairs, { key: '', value: '', enabled: true }];
    }
  });

  function updatePair(index: number, field: keyof KeyValuePair, val: string | boolean) {
    const updated = localPairs.map((p, i) => {
      if (i !== index) return p;
      return { ...p, [field]: val };
    });

    const withoutTrailingEmpty = updated.filter(
      (p, i) => i === updated.length - 1 || p.key !== '' || p.value !== ''
    );

    const hasEmptyLast =
      withoutTrailingEmpty.length > 0 &&
      withoutTrailingEmpty[withoutTrailingEmpty.length - 1].key === '' &&
      withoutTrailingEmpty[withoutTrailingEmpty.length - 1].value === '';

    if (!hasEmptyLast) {
      withoutTrailingEmpty.push({ key: '', value: '', enabled: true });
    }

    localPairs = withoutTrailingEmpty;
    onchange(withoutTrailingEmpty.filter((p) => p.key !== '' || p.value !== ''));
  }

  function removePair(index: number) {
    const updated = localPairs.filter((_, i) => i !== index);
    if (updated.length === 0 || (updated[updated.length - 1].key !== '' || updated[updated.length - 1].value !== '')) {
      updated.push({ key: '', value: '', enabled: true });
    }
    localPairs = updated;
    onchange(updated.filter((p) => p.key !== '' || p.value !== ''));
  }
</script>

<div class="kv-editor">
  <div class="kv-header">
    <span class="kv-col-check"></span>
    <span class="kv-col-key">Key</span>
    <span class="kv-col-value">Value</span>
    <span class="kv-col-action"></span>
  </div>
  {#each localPairs as pair, index}
    <div class="kv-row">
      <div class="kv-col-check">
        <input
          type="checkbox"
          checked={pair.enabled}
          onchange={(e) => updatePair(index, 'enabled', (e.target as HTMLInputElement).checked)}
        />
      </div>
      <div class="kv-col-key">
        <input
          type="text"
          placeholder="Key"
          value={pair.key}
          oninput={(e) => updatePair(index, 'key', (e.target as HTMLInputElement).value)}
        />
      </div>
      <div class="kv-col-value">
        <input
          type="text"
          placeholder="Value"
          value={pair.value}
          oninput={(e) => updatePair(index, 'value', (e.target as HTMLInputElement).value)}
        />
      </div>
      <div class="kv-col-action">
        {#if pair.key !== '' || pair.value !== ''}
          <button class="remove-btn" onclick={() => removePair(index)}>x</button>
        {/if}
      </div>
    </div>
  {/each}
</div>

<style>
  .kv-editor {
    width: 100%;
  }

  .kv-header {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 4px 0;
    border-bottom: 1px solid var(--color-border);
    margin-bottom: 2px;
  }

  .kv-header span {
    font-size: 11px;
    color: var(--color-text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    font-weight: 600;
  }

  .kv-row {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 2px 0;
  }

  .kv-col-check {
    width: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .kv-col-check input[type='checkbox'] {
    width: 14px;
    height: 14px;
    accent-color: var(--color-button);
    cursor: pointer;
  }

  .kv-col-key,
  .kv-col-value {
    flex: 1;
  }

  .kv-col-key input,
  .kv-col-value input {
    width: 100%;
    padding: 5px 8px;
    font-size: 12px;
  }

  .kv-col-action {
    width: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .remove-btn {
    width: 22px;
    height: 22px;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    color: var(--color-text-secondary);
    font-size: 13px;
    border-radius: 4px;
    cursor: pointer;
  }

  .remove-btn:hover {
    background: var(--color-hover);
    color: var(--color-error);
  }
</style>
