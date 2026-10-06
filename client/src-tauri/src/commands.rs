use std::process::Command;

use tauri::AppHandle;
use tauri::Url;
use tauri_plugin_opener::OpenerExt;
use tauri_plugin_store::StoreExt;

fn is_supported_external_url(url: &str) -> bool {
    Url::parse(url)
        .map(|parsed| matches!(parsed.scheme(), "http" | "https"))
        .unwrap_or(false)
}

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
    if !is_supported_external_url(&url) {
        return Err("Only HTTP(S) URLs can be opened".into());
    }

    #[cfg(target_os = "linux")]
    {
        // Try the desktop helpers directly. This covers KDE installations
        // where the opener plugin cannot locate the configured browser.
        for (program, args) in [
            ("xdg-open", vec![url.as_str()]),
            ("gio", vec!["open", url.as_str()]),
            ("kde-open5", vec![url.as_str()]),
            ("kde-open", vec![url.as_str()]),
        ] {
            if Command::new(program).args(args).spawn().is_ok() {
                return Ok(());
            }
        }
    }

    app.opener()
        .open_url(url, None::<&str>)
        .map_err(|error| format!("Unable to open URL: {error}"))
}

#[cfg(test)]
mod tests {
    use super::is_supported_external_url;

    #[test]
    fn accepts_http_and_https_urls() {
        assert!(is_supported_external_url("https://example.com/path?q=1"));
        assert!(is_supported_external_url("http://localhost:3000"));
    }

    #[test]
    fn rejects_non_web_urls_and_malformed_input() {
        assert!(!is_supported_external_url("mailto:user@example.com"));
        assert!(!is_supported_external_url("javascript:alert(1)"));
        assert!(!is_supported_external_url("not a URL"));
    }
}
