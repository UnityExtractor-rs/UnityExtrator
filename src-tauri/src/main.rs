// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod preview;
mod unity;

use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
use preview::{preview_image, preview_text};
use unity::{load_unity_asset, preview_object};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            load_unity_asset,
            preview_image,
            preview_text,
            preview_object
        ])
        .setup(|app| {
            let windows = app.get_window("main").expect("Main windows not found");
            #[cfg(debug_assertions)]
            windows.open_devtools();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
