import type { SavedRequest, HttpResponse } from '../api/tauri';
import * as api from '../api/tauri';
import { workspaceStore } from './workspaces.svelte';

class ActiveRequestStore {
  activeRequest = $state<SavedRequest | null>(null);
  response = $state<HttpResponse | null>(null);
  loading = $state(false);
  private responseCache = new Map<string, HttpResponse>();
  private saveFlags = new Map<string, boolean>();

  getSaveFlag(): boolean {
    if (!this.activeRequest) return true;
    return this.saveFlags.get(this.activeRequest.id) ?? true;
  }

  setSaveFlag(value: boolean) {
    if (!this.activeRequest) return;
    this.saveFlags.set(this.activeRequest.id, value);
    if (!value) {
      this.responseCache.delete(this.activeRequest.id);
    }
  }

  async selectRequest(id: string) {
    if (this.activeRequest && this.response && this.getSaveFlag()) {
      this.responseCache.set(this.activeRequest.id, this.response);
    }
    this.loading = false;
    this.activeRequest = await api.getRequest(id);
    const saved = this.saveFlags.get(id) ?? true;
    this.response = saved ? (this.responseCache.get(id) ?? null) : null;
  }

  updateField<K extends keyof SavedRequest>(field: K, value: SavedRequest[K]) {
    if (!this.activeRequest) return;
    this.activeRequest = { ...this.activeRequest, [field]: value };
  }

  async save() {
    if (!this.activeRequest) return;
    const updated = await api.updateRequest(this.activeRequest);
    this.activeRequest = updated;
    workspaceStore.requests = workspaceStore.requests.map((r) =>
      r.id === updated.id ? updated : r
    );
  }

  async send() {
    if (!this.activeRequest) return;
    this.loading = true;
    this.response = null;
    try {
      await this.save();
      const req = this.activeRequest;
      this.response = await api.sendHttpRequest(
        req.method,
        req.url,
        req.headers.filter((h) => h.enabled),
        req.params.filter((p) => p.enabled),
        req.body,
        req.body_type,
        req.auth_type,
        req.auth_data
      );
    } catch (err) {
      this.response = {
        status: 0,
        status_text: String(err),
        headers: [],
        body: String(err),
        time_ms: 0,
        size_bytes: 0,
      };
    } finally {
      this.loading = false;
      if (this.activeRequest && this.response && this.getSaveFlag()) {
        this.responseCache.set(this.activeRequest.id, this.response);
      }
    }
  }

  clearResponse() {
    this.response = null;
  }

  clear() {
    this.activeRequest = null;
    this.response = null;
    this.loading = false;
  }
}

export const activeRequestStore = new ActiveRequestStore();
