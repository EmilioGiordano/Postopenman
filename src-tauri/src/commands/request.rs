use crate::models::SavedRequest;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn create_request(
    workspace_id: String,
    folder_id: Option<String>,
    name: String,
    db: State<'_, Mutex<Connection>>,
) -> Result<SavedRequest, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let id = uuid::Uuid::new_v4().to_string();

    let max_sort: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(sort_order), -1) FROM requests WHERE workspace_id = ?1",
            rusqlite::params![workspace_id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let sort_order = max_sort + 1;

    conn.execute(
        "INSERT INTO requests (id, folder_id, workspace_id, name, method, url, headers, params, body, body_type, auth_type, auth_data, sort_order) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
        rusqlite::params![id, folder_id, workspace_id, name, "GET", "", "[]", "[]", "", "none", "none", "{}", sort_order],
    )
    .map_err(|e| e.to_string())?;

    Ok(SavedRequest {
        id,
        folder_id,
        workspace_id,
        name,
        method: "GET".to_string(),
        url: String::new(),
        headers: "[]".to_string(),
        params: "[]".to_string(),
        body: String::new(),
        body_type: "none".to_string(),
        auth_type: "none".to_string(),
        auth_data: "{}".to_string(),
        sort_order,
    })
}

#[tauri::command]
pub fn get_request(id: String, db: State<'_, Mutex<Connection>>) -> Result<SavedRequest, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, folder_id, workspace_id, name, method, url, headers, params, body, body_type, auth_type, auth_data, sort_order FROM requests WHERE id = ?1",
        )
        .map_err(|e| e.to_string())?;

    let request = stmt
        .query_row(rusqlite::params![id], |row| {
            Ok(SavedRequest {
                id: row.get(0)?,
                folder_id: row.get(1)?,
                workspace_id: row.get(2)?,
                name: row.get(3)?,
                method: row.get(4)?,
                url: row.get(5)?,
                headers: row.get(6)?,
                params: row.get(7)?,
                body: row.get(8)?,
                body_type: row.get(9)?,
                auth_type: row.get(10)?,
                auth_data: row.get(11)?,
                sort_order: row.get(12)?,
            })
        })
        .map_err(|e| e.to_string())?;

    Ok(request)
}

#[tauri::command]
pub fn update_request(
    request: SavedRequest,
    db: State<'_, Mutex<Connection>>,
) -> Result<SavedRequest, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    let updated = conn
        .execute(
            "UPDATE requests SET folder_id = ?1, workspace_id = ?2, name = ?3, method = ?4, url = ?5, headers = ?6, params = ?7, body = ?8, body_type = ?9, auth_type = ?10, auth_data = ?11, sort_order = ?12 WHERE id = ?13",
            rusqlite::params![
                request.folder_id,
                request.workspace_id,
                request.name,
                request.method,
                request.url,
                request.headers,
                request.params,
                request.body,
                request.body_type,
                request.auth_type,
                request.auth_data,
                request.sort_order,
                request.id,
            ],
        )
        .map_err(|e| e.to_string())?;

    if updated == 0 {
        return Err("request not found".to_string());
    }

    Ok(request)
}

#[tauri::command]
pub fn move_request(
    id: String,
    folder_id: Option<String>,
    db: State<'_, Mutex<Connection>>,
) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    let updated = conn
        .execute(
            "UPDATE requests SET folder_id = ?1 WHERE id = ?2",
            rusqlite::params![folder_id, id],
        )
        .map_err(|e| e.to_string())?;

    if updated == 0 {
        return Err("request not found".to_string());
    }

    Ok(())
}

#[tauri::command]
pub fn rename_request(
    id: String,
    name: String,
    db: State<'_, Mutex<Connection>>,
) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    let updated = conn
        .execute(
            "UPDATE requests SET name = ?1 WHERE id = ?2",
            rusqlite::params![name, id],
        )
        .map_err(|e| e.to_string())?;

    if updated == 0 {
        return Err("request not found".to_string());
    }

    Ok(())
}

#[tauri::command]
pub fn delete_request(id: String, db: State<'_, Mutex<Connection>>) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    let deleted = conn
        .execute("DELETE FROM requests WHERE id = ?1", rusqlite::params![id])
        .map_err(|e| e.to_string())?;

    if deleted == 0 {
        return Err("request not found".to_string());
    }

    Ok(())
}

#[tauri::command]
pub fn list_requests(
    workspace_id: String,
    db: State<'_, Mutex<Connection>>,
) -> Result<Vec<SavedRequest>, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, folder_id, workspace_id, name, method, url, headers, params, body, body_type, auth_type, auth_data, sort_order FROM requests WHERE workspace_id = ?1 ORDER BY sort_order ASC",
        )
        .map_err(|e| e.to_string())?;

    let requests = stmt
        .query_map(rusqlite::params![workspace_id], |row| {
            Ok(SavedRequest {
                id: row.get(0)?,
                folder_id: row.get(1)?,
                workspace_id: row.get(2)?,
                name: row.get(3)?,
                method: row.get(4)?,
                url: row.get(5)?,
                headers: row.get(6)?,
                params: row.get(7)?,
                body: row.get(8)?,
                body_type: row.get(9)?,
                auth_type: row.get(10)?,
                auth_data: row.get(11)?,
                sort_order: row.get(12)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(requests)
}
