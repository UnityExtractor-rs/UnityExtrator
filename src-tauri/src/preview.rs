use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use tauri::{command, AppHandle, Manager, Window, WindowBuilder, WindowUrl};

const PREVIEW_IMG: &str = "preview_image";
const PREVIEW_TEXT: &str = "preview_text";
const WINDOWS_NAME: &str = "Preview";
static WINDOW_PATH: Lazy<&'static Path> = Lazy::new(|| Path::new("/preview"));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagePreviewPayload {
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    pub name: String,
    pub description: Option<String>,
    pub fromat: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

#[command(async)]
pub async fn preview_image(app: AppHandle, payload: ImagePreviewPayload) -> tauri::Result<()> {
    let window = init_window(&app).await?;
    window.emit(PREVIEW_IMG, payload)?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextPreviewPayload {
    pub payload: String,
    pub name: String,
    pub description: Option<String>,
}

#[command(async)]
pub async fn preview_text(app: AppHandle, payload: TextPreviewPayload) -> tauri::Result<()> {
    let window = init_window(&app).await?;
    window.emit(PREVIEW_TEXT, payload)?;
    Ok(())
}

pub async fn init_window(app: &AppHandle) -> tauri::Result<Window> {
    let window = if let Some(window) = app.get_window(WINDOWS_NAME) {
        window
    } else {
        let w = WindowBuilder::new(app, WINDOWS_NAME, WindowUrl::App(WINDOW_PATH.to_path_buf()))
            .title("Preview")
            .focused(true)
            .center()
            .inner_size(816f64, 648f64)
            .build()?;

        // wait window open
        sleep(Duration::from_millis(500));
        w
    };

    #[cfg(debug_assertions)]
    if !window.is_devtools_open() {
        window.open_devtools()
    }

    Ok(window)
}
