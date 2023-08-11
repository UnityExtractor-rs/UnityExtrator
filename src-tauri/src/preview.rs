use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use tauri::api::dir::with_temp_dir;
use tauri::{command, AppHandle, Manager, WindowBuilder, WindowUrl};

const PREVIEW_IMG: &str = "preview-img";
const WINDOWS_NAME: &str = "Preview";
static WINDOW_PATH: Lazy<&'static Path> = Lazy::new(|| Path::new("/preview"));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewPayload {
    obj_url: String,
    name: String,
    description: Option<String>,
    ty: Option<String>,
    width: Option<i32>,
    height: Option<i32>,
}

#[command]
pub async fn preview_image(app: AppHandle, payload: PreviewPayload) -> tauri::Result<()> {
    let window = if let Some(window) = app.get_window(WINDOWS_NAME) {
        window
    } else {
        WindowBuilder::new(
            &app,
            WINDOWS_NAME,
            WindowUrl::App(WINDOW_PATH.to_path_buf()),
        )
        .title("Preview")
        .focused(true)
        .center()
        .inner_size(816f64, 648f64)
        .build()?
    };
    #[cfg(debug_assertions)]
    if !window.is_devtools_open() {
        window.open_devtools()
    }
    // wait window open
    sleep(Duration::from_millis(500));
    window.emit(PREVIEW_IMG, payload)?;

    Ok(())
}
