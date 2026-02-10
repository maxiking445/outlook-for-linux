mod commands;
mod menu;
mod window;

use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                window.set_focus().unwrap();
                window.show().unwrap();
            }
        }))
        .invoke_handler(tauri::generate_handler![
            commands::send_notification,
            commands::open_in_browser
        ])
        .on_window_event(|window, event| {
            window::handle_window_event(window, event);
        })
        .setup(|app| {
            window::setup_window(app)?;
            menu::setup_tray_menu(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Something wrong when running tauri application");
}