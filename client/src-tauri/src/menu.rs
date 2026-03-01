use crate::commands::open_in_browser;
use serde_json::json;
use tauri::menu::{CheckMenuItemBuilder, MenuBuilder, MenuItemBuilder, SubmenuBuilder};
use tauri::tray::TrayIconBuilder;
use tauri::Manager;
use tauri_plugin_store::Store;
use tauri_plugin_store::StoreExt;

pub fn setup_tray_menu(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let hide = MenuItemBuilder::new("Hide").id("hide").build(app)?;
    let report = MenuItemBuilder::new("Report an Issue")
        .id("report")
        .build(app)?;
    let version_text = format!("Version: {}", app.package_info().version);

    let version = MenuItemBuilder::new(&version_text)
        .id("version")
        .enabled(false)
        .build(app)?;
    let help_menu = SubmenuBuilder::new(app, "Help")
        .items(&[&report, &version])
        .build()?;

    let store = app.store("settings.json").unwrap();
    let is_notifcation_enabled = store.get("notifications_enabled").unwrap_or(json!(true));

    let notification_checkbox = CheckMenuItemBuilder::new("Enable Notifications")
        .id("notification_checkbox")
        .checked(is_notifcation_enabled.as_bool().unwrap_or(true))
        .build(app)?;

    let settings_menu = SubmenuBuilder::new(app, "Settings")
        .items(&[&notification_checkbox])
        .build()?;
    let show = MenuItemBuilder::new("Show").id("show").build(app)?;
    let quit = MenuItemBuilder::new("Quit").id("quit").build(app)?;

    let menu = MenuBuilder::new(app)
        .items(&[&hide, &show, &settings_menu, &help_menu, &quit])
        .build()?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "quit" => app.exit(0),
            "hide" => {
                let window = app.get_webview_window("main").unwrap();
                window.minimize().unwrap();
            }
            "report" => {
                open_in_browser(
                    app.clone(),
                    "https://github.com/maxiking445/outlook-for-linux/issues".to_string(),
                );
            }
            "notification_checkbox" => {
                let store = app.store("settings.json").unwrap();
                let current = store.get("notifications_enabled").unwrap_or(json!(true));

                store.set(
                    "notifications_enabled",
                    json!(!current.as_bool().unwrap_or(true)),
                );
                let _ = store.save();
            }
            "show" => {
                let window = app.get_webview_window("main").unwrap();
                window.hide().unwrap();
                window.unminimize().unwrap();
                window.show().unwrap();
                let _ = window.set_focus();
            }
            _ => {}
        })
        .build(app)?;

    Ok(())
}
