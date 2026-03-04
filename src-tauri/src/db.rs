use rusqlite::Connection;
use tauri::Manager;

pub fn init_db(app_handle: &tauri::AppHandle) -> Result<Connection, Box<dyn std::error::Error>> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("failed to resolve app data dir: {e}"))?;

    std::fs::create_dir_all(&app_dir)
        .map_err(|e| format!("failed to create app data dir: {e}"))?;

    let db_path = app_dir.join("free_postman.db");

    let conn =
        Connection::open(&db_path).map_err(|e| format!("failed to open database: {e}"))?;

    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")
        .map_err(|e| format!("failed to set pragmas: {e}"))?;

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS workspaces (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            created_at TEXT,
            updated_at TEXT
        );

        CREATE TABLE IF NOT EXISTS folders (
            id TEXT PRIMARY KEY,
            workspace_id TEXT NOT NULL,
            parent_folder_id TEXT,
            name TEXT NOT NULL,
            sort_order INTEGER DEFAULT 0,
            FOREIGN KEY (workspace_id) REFERENCES workspaces(id) ON DELETE CASCADE,
            FOREIGN KEY (parent_folder_id) REFERENCES folders(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS requests (
            id TEXT PRIMARY KEY,
            folder_id TEXT,
            workspace_id TEXT NOT NULL,
            name TEXT NOT NULL,
            method TEXT DEFAULT 'GET',
            url TEXT DEFAULT '',
            headers TEXT DEFAULT '[]',
            params TEXT DEFAULT '[]',
            body TEXT DEFAULT '',
            body_type TEXT DEFAULT 'none',
            auth_type TEXT DEFAULT 'none',
            auth_data TEXT DEFAULT '{}',
            sort_order INTEGER DEFAULT 0,
            FOREIGN KEY (folder_id) REFERENCES folders(id) ON DELETE SET NULL,
            FOREIGN KEY (workspace_id) REFERENCES workspaces(id) ON DELETE CASCADE
        );",
    )
    .map_err(|e| format!("failed to create tables: {e}"))?;

    Ok(conn)
}
