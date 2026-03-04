<script lang="ts">
  import KeyValueEditor from '../shared/KeyValueEditor.svelte';
  import { activeRequestStore } from '../../stores/activeRequest.svelte';
  import type { KeyValuePair } from '../../api/tauri';

  let pairs = $derived(activeRequestStore.activeRequest?.headers ?? []);

  function handleChange(updated: KeyValuePair[]) {
    activeRequestStore.updateField('headers', updated);
  }
</script>

<KeyValueEditor {pairs} onchange={handleChange} />
