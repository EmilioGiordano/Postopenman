import type { Workspace, Folder, SavedRequest } from '../api/tauri';
import * as api from '../api/tauri';

class WorkspaceStore {
  workspaces = $state<Workspace[]>([]);
  activeWorkspaceId = $state<string | null>(null);
  folders = $state<Folder[]>([]);
  requests = $state<SavedRequest[]>([]);

  activeWorkspace = $derived(
    this.workspaces.find((w) => w.id === this.activeWorkspaceId) ?? null
  );

  rootFolders = $derived(
    this.folders.filter((f) => f.parent_folder_id === null)
  );

  rootRequests = $derived(
    this.requests.filter((r) => r.folder_id === null)
  );

  async loadWorkspaces() {
    try {
      this.workspaces = await api.listWorkspaces();
      if (this.workspaces.length > 0 && !this.activeWorkspaceId) {
        await this.selectWorkspace(this.workspaces[0].id);
      }
    } catch (err) {
      console.error('Failed to load workspaces:', err);
    }
  }

  async selectWorkspace(id: string) {
    try {
      this.activeWorkspaceId = id;
      this.folders = await api.listFolders(id);
      this.requests = await api.listRequests(id);
    } catch (err) {
      console.error('Failed to select workspace:', err);
    }
  }

  async createWorkspace(name: string) {
    try {
      const ws = await api.createWorkspace(name);
      this.workspaces = [...this.workspaces, ws];
      await this.selectWorkspace(ws.id);
    } catch (err) {
      console.error('Failed to create workspace:', err);
    }
  }

  async deleteWorkspace(id: string) {
    try {
      await api.deleteWorkspace(id);
      this.workspaces = this.workspaces.filter((w) => w.id !== id);
      if (this.activeWorkspaceId === id) {
        this.activeWorkspaceId = this.workspaces[0]?.id ?? null;
        if (this.activeWorkspaceId) {
          await this.selectWorkspace(this.activeWorkspaceId);
        } else {
          this.folders = [];
          this.requests = [];
        }
      }
    } catch (err) {
      console.error('Failed to delete workspace:', err);
    }
  }

  async renameWorkspace(id: string, name: string) {
    try {
      const updated = await api.renameWorkspace(id, name);
      this.workspaces = this.workspaces.map((w) => (w.id === id ? updated : w));
    } catch (err) {
      console.error('Failed to rename workspace:', err);
    }
  }

  async createFolder(parentId: string | null) {
    if (!this.activeWorkspaceId) return;
    try {
      const folder = await api.createFolder(this.activeWorkspaceId, parentId, 'New Folder');
      this.folders = [...this.folders, folder];
    } catch (err) {
      console.error('Failed to create folder:', err);
    }
  }

  async deleteFolder(id: string) {
    try {
      await api.deleteFolder(id);
      const childIds = this.getAllDescendantFolderIds(id);
      const allIds = [id, ...childIds];
      this.folders = this.folders.filter((f) => !allIds.includes(f.id));
      this.requests = this.requests.filter((r) => !r.folder_id || !allIds.includes(r.folder_id));
    } catch (err) {
      console.error('Failed to delete folder:', err);
    }
  }

  async renameFolder(id: string, name: string) {
    try {
      const updated = await api.renameFolder(id, name);
      this.folders = this.folders.map((f) => (f.id === id ? updated : f));
    } catch (err) {
      console.error('Failed to rename folder:', err);
    }
  }

  async createRequest(folderId: string | null): Promise<SavedRequest | null> {
    if (!this.activeWorkspaceId) return null;
    try {
      const req = await api.createRequest(this.activeWorkspaceId, folderId, 'New Request');
      this.requests = [...this.requests, req];
      return req;
    } catch (err) {
      console.error('Failed to create request:', err);
      return null;
    }
  }

  async moveRequest(id: string, folderId: string | null) {
    try {
      await api.moveRequest(id, folderId);
      this.requests = this.requests.map((r) =>
        r.id === id ? { ...r, folder_id: folderId } : r
      );
    } catch (err) {
      console.error('Failed to move request:', err);
    }
  }

  async renameRequest(id: string, name: string) {
    try {
      await api.renameRequest(id, name);
      this.requests = this.requests.map((r) =>
        r.id === id ? { ...r, name } : r
      );
    } catch (err) {
      console.error('Failed to rename request:', err);
    }
  }

  async deleteRequest(id: string) {
    try {
      await api.deleteRequest(id);
      this.requests = this.requests.filter((r) => r.id !== id);
    } catch (err) {
      console.error('Failed to delete request:', err);
    }
  }

  getChildFolders(parentId: string): Folder[] {
    return this.folders.filter((f) => f.parent_folder_id === parentId);
  }

  getFolderRequests(folderId: string): SavedRequest[] {
    return this.requests.filter((r) => r.folder_id === folderId);
  }

  private getAllDescendantFolderIds(parentId: string): string[] {
    const children = this.folders.filter((f) => f.parent_folder_id === parentId);
    const ids: string[] = [];
    for (const child of children) {
      ids.push(child.id);
      ids.push(...this.getAllDescendantFolderIds(child.id));
    }
    return ids;
  }
}

export const workspaceStore = new WorkspaceStore();
