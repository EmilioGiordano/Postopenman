#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod models;

use std::sync::Mutex;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let conn = db::init_db(app.handle())?;
            app.manage(Mutex::new(conn));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::workspace::create_workspace,
            commands::workspace::list_workspaces,
            commands::workspace::rename_workspace,
            commands::workspace::delete_workspace,
            commands::folder::create_folder,
            commands::folder::list_folders,
            commands::folder::rename_folder,
            commands::folder::delete_folder,
            commands::request::create_request,
            commands::request::get_request,
            commands::request::update_request,
            commands::request::delete_request,
            commands::request::list_requests,
            commands::request::move_request,
            commands::request::rename_request,
            commands::http::send_request,
        ])
        .run(tauri::generate_context!())
        .expect("error running tauri application");
}
