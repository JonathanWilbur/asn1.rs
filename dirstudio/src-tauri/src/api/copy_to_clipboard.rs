use tauri::ClipboardManager;
use log::{info, error};

#[tauri::command]
pub async fn copy_to_clipboard(
    app_handle: tauri::AppHandle,
    text: &str,
) -> Result<(), ()> {
    println!("Writing to clipboard.");
    info!("Writing to clipboard.");
    let mut cm = app_handle.clipboard_manager();
    if let Err(e) = cm.write_text(text) {
        println!("Failed to write to clipboard: {}", e);
        error!("Failed to write to clipboard: {}", e);
    }
    Ok(())
}
