<script lang="ts">
  import KeyValueEditor from '../shared/KeyValueEditor.svelte';
  import { activeRequestStore } from '../../stores/activeRequest.svelte';
  import type { KeyValuePair } from '../../api/tauri';

  let pairs = $derived(activeRequestStore.activeRequest?.params ?? []);

  function handleChange(updated: KeyValuePair[]) {
    activeRequestStore.updateField('params', updated);
  }
</script>

<KeyValueEditor {pairs} onchange={handleChange} />
