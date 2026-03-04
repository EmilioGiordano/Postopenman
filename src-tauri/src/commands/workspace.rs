use crate::models::Workspace;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn create_workspace(
    name: String,
    db: State<'_, Mutex<Connection>>,
) -> Result<Workspace, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO workspaces (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![id, name, now, now],
    )
    .map_err(|e| e.to_string())?;

    Ok(Workspace {
        id,
        name,
        created_at: now.clone(),
        updated_at: now,
    })
}

#[tauri::command]
pub fn list_workspaces(db: State<'_, Mutex<Connection>>) -> Result<Vec<Workspace>, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, name, created_at, updated_at FROM workspaces ORDER BY created_at ASC")
        .map_err(|e| e.to_string())?;

    let workspaces = stmt
        .query_map([], |row| {
            Ok(Workspace {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get(2)?,
                updated_at: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(workspaces)
}

#[tauri::command]
pub fn rename_workspace(
    id: String,
    name: String,
    db: State<'_, Mutex<Connection>>,
) -> Result<Workspace, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let now = chrono::Utc::now().to_rfc3339();

    let updated = conn
        .execute(
            "UPDATE workspaces SET name = ?1, updated_at = ?2 WHERE id = ?3",
            rusqlite::params![name, now, id],
        )
        .map_err(|e| e.to_string())?;

    if updated == 0 {
        return Err("workspace not found".to_string());
    }

    let mut stmt = conn
        .prepare("SELECT id, name, created_at, updated_at FROM workspaces WHERE id = ?1")
        .map_err(|e| e.to_string())?;

    let workspace = stmt
        .query_row(rusqlite::params![id], |row| {
            Ok(Workspace {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get(2)?,
                updated_at: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?;

    Ok(workspace)
}

#[tauri::command]
pub fn delete_workspace(id: String, db: State<'_, Mutex<Connection>>) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    let deleted = conn
        .execute("DELETE FROM workspaces WHERE id = ?1", rusqlite::params![id])
        .map_err(|e| e.to_string())?;

    if deleted == 0 {
        return Err("workspace not found".to_string());
    }

    Ok(())
}
