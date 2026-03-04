use serde::Deserialize;

#[derive(Deserialize)]
pub struct FileFilter {
    pub name: String,
    pub extensions: Vec<String>,
}

#[tauri::command]
pub async fn export_file(
    content: String,
    default_name: String,
    filters: Vec<FileFilter>,
) -> Result<bool, String> {
    let mut dialog = rfd::AsyncFileDialog::new().set_file_name(&default_name);

    for filter in &filters {
        let ext_refs: Vec<&str> = filter.extensions.iter().map(|s| s.as_str()).collect();
        dialog = dialog.add_filter(&filter.name, &ext_refs);
    }

    match dialog.save_file().await {
        Some(handle) => {
            std::fs::write(handle.path(), content.as_bytes()).map_err(|e| e.to_string())?;
            Ok(true)
        }
        None => Ok(false),
    }
}
