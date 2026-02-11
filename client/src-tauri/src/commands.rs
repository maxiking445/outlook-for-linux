use std::process::Command;
use std::thread;
use tauri::AppHandle;
use tauri_plugin_opener::OpenerExt;
use tauri_plugin_store::StoreExt;

/// Sends a desktop notification -> workaround for linux
/// See: https://github.com/tauri-apps/plugins-workspace/issues/2566
#[tauri::command]
pub fn send_notification(app: AppHandle, title: String, body: String) -> Result<(), String> {
    let store = app.store("settings.json").unwrap();
    let enabled = store
        .get("notifications_enabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    if enabled {
        thread::spawn(move || {
            let result = Command::new("notify-send")
                .args([
                    "--app-name=wngtools",
                    "--urgency=normal",
                    "--expire-time=8000",
                    "--hint=string:sound-name:message-new-instant",
                    &title,
                    &body,
                ])
                .output();

            match result {
                Ok(output) if output.status.success() => {
                    println!("Notification sent: {}", title);
                }
                _ => {
                    eprintln!("Notification failed for: {}", title);
                }
            }
        });
    }
    Ok(())
}

#[tauri::command]
pub fn open_in_browser(app: tauri::AppHandle, url: String) -> Result<(), String> {
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err("Invalid URL".into());
    }
    let _ = app.opener().open_url(url, None::<&str>);
    Ok(())
}