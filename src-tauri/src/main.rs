// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod unity;

use tauri::{LogicalSize, Manager, Size};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
use unity::extractor_unity_img;
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,extractor_unity_img])
        .setup(|app| {
            let windows = app.get_window("main").expect("Main windows not found");
            windows.set_size(Size::Logical(LogicalSize::new(1024.0, 512.0)))?;
            windows.set_resizable(false)?;

            #[cfg(debug_assertions)]
            windows.open_devtools();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
