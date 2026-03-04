<script lang="ts">
  import { activeRequestStore } from '../../stores/activeRequest.svelte';

  let authType = $derived(activeRequestStore.activeRequest?.auth_type ?? 'none');
  let authData = $derived(activeRequestStore.activeRequest?.auth_data ?? '');

  let parsed = $derived.by<{ token?: string; username?: string; password?: string }>(() => {
    try {
      return JSON.parse(authData);
    } catch {
      return {};
    }
  });

  function handleTypeChange(e: Event) {
    const val = (e.target as HTMLSelectElement).value;
    activeRequestStore.updateField('auth_type', val);
    activeRequestStore.updateField('auth_data', '{}');
  }

  function updateAuthData(key: string, value: string) {
    let current: Record<string, string>;
    try {
      current = JSON.parse(authData);
    } catch {
      current = {};
    }
    current[key] = value;
    activeRequestStore.updateField('auth_data', JSON.stringify(current));
  }
</script>

<div class="auth-editor">
  <div class="auth-type-row">
    <label class="auth-label" for="auth-type-select">Type</label>
    <select id="auth-type-select" class="auth-select" value={authType} onchange={handleTypeChange}>
      <option value="none">None</option>
      <option value="bearer">Bearer Token</option>
      <option value="basic">Basic Auth</option>
    </select>
  </div>

  {#if authType === 'bearer'}
    <div class="auth-field">
      <label class="auth-label" for="auth-token">Token</label>
      <input
        id="auth-token"
        class="auth-input"
        type="text"
        placeholder="Enter bearer token..."
        value={parsed.token ?? ''}
        oninput={(e) => updateAuthData('token', (e.target as HTMLInputElement).value)}
      />
    </div>
  {:else if authType === 'basic'}
    <div class="auth-field">
      <label class="auth-label" for="auth-username">Username</label>
      <input
        id="auth-username"
        class="auth-input"
        type="text"
        placeholder="Username"
        value={parsed.username ?? ''}
        oninput={(e) => updateAuthData('username', (e.target as HTMLInputElement).value)}
      />
    </div>
    <div class="auth-field">
      <label class="auth-label" for="auth-password">Password</label>
      <input
        id="auth-password"
        class="auth-input"
        type="password"
        placeholder="Password"
        value={parsed.password ?? ''}
        oninput={(e) => updateAuthData('password', (e.target as HTMLInputElement).value)}
      />
    </div>
  {:else}
    <div class="auth-none">No authentication configured for this request.</div>
  {/if}
</div>

<style>
  .auth-editor {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .auth-type-row {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .auth-label {
    font-size: 12px;
    color: var(--color-text-secondary);
    min-width: 70px;
    font-weight: 500;
  }

  .auth-select {
    padding: 6px 28px 6px 10px;
    font-size: 12px;
    min-width: clamp(120px, 18%, 180px);
  }

  .auth-field {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .auth-input {
    flex: 1;
    padding: 7px 10px;
    font-size: 12px;
  }

  .auth-none {
    color: var(--color-text-secondary);
    font-size: 12px;
    padding: 8px 0;
  }
</style>
