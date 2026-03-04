use crate::models::Folder;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn create_folder(
    workspace_id: String,
    parent_folder_id: Option<String>,
    name: String,
    db: State<'_, Mutex<Connection>>,
) -> Result<Folder, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let id = uuid::Uuid::new_v4().to_string();

    let max_sort: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(sort_order), -1) FROM folders WHERE workspace_id = ?1",
            rusqlite::params![workspace_id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let sort_order = max_sort + 1;

    conn.execute(
        "INSERT INTO folders (id, workspace_id, parent_folder_id, name, sort_order) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![id, workspace_id, parent_folder_id, name, sort_order],
    )
    .map_err(|e| e.to_string())?;

    Ok(Folder {
        id,
        workspace_id,
        parent_folder_id,
        name,
        sort_order,
    })
}

#[tauri::command]
pub fn list_folders(
    workspace_id: String,
    db: State<'_, Mutex<Connection>>,
) -> Result<Vec<Folder>, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, workspace_id, parent_folder_id, name, sort_order FROM folders WHERE workspace_id = ?1 ORDER BY sort_order ASC",
        )
        .map_err(|e| e.to_string())?;

    let folders = stmt
        .query_map(rusqlite::params![workspace_id], |row| {
            Ok(Folder {
                id: row.get(0)?,
                workspace_id: row.get(1)?,
                parent_folder_id: row.get(2)?,
                name: row.get(3)?,
                sort_order: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(folders)
}

#[tauri::command]
pub fn rename_folder(
    id: String,
    name: String,
    db: State<'_, Mutex<Connection>>,
) -> Result<Folder, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    let updated = conn
        .execute(
            "UPDATE folders SET name = ?1 WHERE id = ?2",
            rusqlite::params![name, id],
        )
        .map_err(|e| e.to_string())?;

    if updated == 0 {
        return Err("folder not found".to_string());
    }

    let mut stmt = conn
        .prepare("SELECT id, workspace_id, parent_folder_id, name, sort_order FROM folders WHERE id = ?1")
        .map_err(|e| e.to_string())?;

    let folder = stmt
        .query_row(rusqlite::params![id], |row| {
            Ok(Folder {
                id: row.get(0)?,
                workspace_id: row.get(1)?,
                parent_folder_id: row.get(2)?,
                name: row.get(3)?,
                sort_order: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    Ok(folder)
}

#[tauri::command]
pub fn delete_folder(id: String, db: State<'_, Mutex<Connection>>) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    fn collect_child_folder_ids(conn: &Connection, parent_id: &str) -> Result<Vec<String>, String> {
        let mut stmt = conn
            .prepare("SELECT id FROM folders WHERE parent_folder_id = ?1")
            .map_err(|e| e.to_string())?;

        let direct_children: Vec<String> = stmt
            .query_map(rusqlite::params![parent_id], |row| row.get(0))
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        let mut all_ids = Vec::new();
        for child_id in &direct_children {
            all_ids.extend(collect_child_folder_ids(conn, child_id)?);
        }
        all_ids.extend(direct_children);
        Ok(all_ids)
    }

    let mut folder_ids_to_delete = collect_child_folder_ids(&conn, &id)?;
    folder_ids_to_delete.push(id.clone());

    for fid in &folder_ids_to_delete {
        conn.execute(
            "DELETE FROM requests WHERE folder_id = ?1",
            rusqlite::params![fid],
        )
        .map_err(|e| e.to_string())?;
    }

    for fid in &folder_ids_to_delete {
        conn.execute("DELETE FROM folders WHERE id = ?1", rusqlite::params![fid])
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}
