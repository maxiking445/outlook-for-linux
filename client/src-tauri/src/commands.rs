use tauri::AppHandle;
use tauri_plugin_opener::OpenerExt;
use tauri_plugin_store::StoreExt;

#[cfg(target_os = "linux")]
use notify_rust::Notification;
#[cfg(not(target_os = "linux"))]
use tauri_plugin_notification::NotificationExt;

/// Sends a native notification, retaining its D-Bus connection on Linux.
#[tauri::command]
pub fn send_notification(app: AppHandle, title: String, body: String) -> Result<(), String> {
    let store = app
        .store("settings.json")
        .map_err(|error| error.to_string())?;
    let enabled = store
        .get("notifications_enabled")
        .and_then(|value| value.as_bool())
        .unwrap_or(true);

    if !enabled {
        return Ok(());
    }

    #[cfg(target_os = "linux")]
    {
        let handle = Notification::new()
            .appname("Outlook for Linux")
            .summary(&title)
            .body(&body)
            .icon("mail-unread")
            .hint(notify_rust::Hint::SuppressSound(true))
            .show()
            .map_err(|error| {
                eprintln!("System notification failed: {error}");
                error.to_string()
            })?;
        // GNOME 46+ closes the notification when the D-Bus connection is
        // dropped. Keep the handle alive until the desktop closes it.
        std::thread::spawn(move || handle.on_close(|_| {}));
        return Ok(());
    }

    #[cfg(not(target_os = "linux"))]
    app.notification()
        .builder()
        .title(title)
        .body(body)
        .sound("message-new-instant")
        .show()
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn open_in_browser(app: tauri::AppHandle, url: String) -> Result<(), String> {
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err("Invalid URL".into());
    }
    let _ = app.opener().open_url(url, None::<&str>);
    Ok(())
}
