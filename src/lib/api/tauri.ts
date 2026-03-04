import { invoke } from '@tauri-apps/api/core';

export interface Workspace {
  id: string;
  name: string;
  created_at: string;
  updated_at: string;
}

export interface Folder {
  id: string;
  workspace_id: string;
  parent_folder_id: string | null;
  name: string;
  sort_order: number;
}

export interface KeyValuePair {
  key: string;
  value: string;
  enabled: boolean;
}

export interface SavedRequest {
  id: string;
  folder_id: string | null;
  workspace_id: string;
  name: string;
  method: string;
  url: string;
  headers: KeyValuePair[];
  params: KeyValuePair[];
  body: string;
  body_type: string;
  auth_type: string;
  auth_data: string;
  sort_order: number;
}

interface RawSavedRequest {
  id: string;
  folder_id: string | null;
  workspace_id: string;
  name: string;
  method: string;
  url: string;
  headers: string;
  params: string;
  body: string;
  body_type: string;
  auth_type: string;
  auth_data: string;
  sort_order: number;
}

export interface HttpResponse {
  status: number;
  status_text: string;
  headers: KeyValuePair[];
  body: string;
  time_ms: number;
  size_bytes: number;
}

function parseRequest(raw: RawSavedRequest): SavedRequest {
  let headers: KeyValuePair[] = [];
  let params: KeyValuePair[] = [];
  try { headers = JSON.parse(raw.headers); } catch { /* empty */ }
  try { params = JSON.parse(raw.params); } catch { /* empty */ }
  return { ...raw, headers, params };
}

function serializeRequest(req: SavedRequest): RawSavedRequest {
  return {
    ...req,
    headers: JSON.stringify(req.headers),
    params: JSON.stringify(req.params),
  };
}

export async function createWorkspace(name: string): Promise<Workspace> {
  return invoke('create_workspace', { name });
}

export async function listWorkspaces(): Promise<Workspace[]> {
  return invoke('list_workspaces');
}

export async function renameWorkspace(id: string, name: string): Promise<Workspace> {
  return invoke('rename_workspace', { id, name });
}

export async function deleteWorkspace(id: string): Promise<void> {
  return invoke('delete_workspace', { id });
}

export async function createFolder(
  workspaceId: string,
  parentFolderId: string | null,
  name: string
): Promise<Folder> {
  return invoke('create_folder', { workspaceId, parentFolderId, name });
}

export async function listFolders(workspaceId: string): Promise<Folder[]> {
  return invoke('list_folders', { workspaceId });
}

export async function renameFolder(id: string, name: string): Promise<Folder> {
  return invoke('rename_folder', { id, name });
}

export async function deleteFolder(id: string): Promise<void> {
  return invoke('delete_folder', { id });
}

export async function createRequest(
  workspaceId: string,
  folderId: string | null,
  name: string
): Promise<SavedRequest> {
  const raw: RawSavedRequest = await invoke('create_request', { workspaceId, folderId, name });
  return parseRequest(raw);
}

export async function getRequest(id: string): Promise<SavedRequest> {
  const raw: RawSavedRequest = await invoke('get_request', { id });
  return parseRequest(raw);
}

export async function updateRequest(request: SavedRequest): Promise<SavedRequest> {
  const raw: RawSavedRequest = await invoke('update_request', { request: serializeRequest(request) });
  return parseRequest(raw);
}

export async function deleteRequest(id: string): Promise<void> {
  return invoke('delete_request', { id });
}

export async function listRequests(workspaceId: string): Promise<SavedRequest[]> {
  const raws: RawSavedRequest[] = await invoke('list_requests', { workspaceId });
  return raws.map(parseRequest);
}

export async function moveRequest(id: string, folderId: string | null): Promise<void> {
  return invoke('move_request', { id, folderId });
}

export async function renameRequest(id: string, name: string): Promise<void> {
  return invoke('rename_request', { id, name });
}

export async function sendHttpRequest(
  method: string,
  url: string,
  headers: KeyValuePair[],
  params: KeyValuePair[],
  body: string,
  bodyType: string,
  authType: string,
  authData: string
): Promise<HttpResponse> {
  return invoke('send_request', {
    method,
    url,
    headers,
    params,
    body,
    bodyType,
    authType,
    authData,
  });
}
