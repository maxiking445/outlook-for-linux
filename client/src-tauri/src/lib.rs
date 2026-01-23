use std::fs;
use std::process::Command;
use std::thread;
use std::time::Duration;
use tauri::menu::MenuItemBuilder;
use tauri::AppHandle;
use tauri::WebviewWindow;
use tauri::{menu::MenuBuilder, tray::TrayIconBuilder, Manager, WindowEvent};
use tauri_plugin_notification::NotificationExt;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![send_notification])
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window
                .eval("window.location.href = 'https://outlook.office.com'")
                .unwrap();

            let window_clone = window.clone();
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_secs(3));
                inject_notification_js(window_clone);
            });

            let hide = MenuItemBuilder::new("Hide").id("hide").build(app).unwrap();
            let show = MenuItemBuilder::new("Show").id("show").build(app).unwrap();
            let quit = MenuItemBuilder::new("Quit").id("quit").build(app).unwrap();
            let menu = MenuBuilder::new(app)
                .items(&[&hide, &show, &quit])
                .build()
                .unwrap();

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "quit" => app.exit(0),
                    "hide" => {
                        let window = app.get_webview_window("main").unwrap();
                        window.hide().unwrap();
                    }
                    "show" => {
                        let window = app.get_webview_window("main").unwrap();
                        window.show().unwrap();
                        let _ = window.set_focus();
                    }
                    _ => {}
                })
                .build(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Something wrong when running tauri application");
}

fn inject_notification_js(window: WebviewWindow) {
    let js_content =
        fs::read_to_string("../src/notification-extractor.js").expect("extractor.js not found!");

    window.eval(&js_content).expect("Injection failed!");
    println!("extractor.js injected!");

    let js_content =
        fs::read_to_string("../src/notification.js").expect("notification.js not found!");

    window.eval(&js_content).expect("Injection failed!");
    println!("notification.js injected!");
}

// Sends a desktop notification -> workarround for linux see -> https://github.com/tauri-apps/plugins-workspace/issues/2566
#[tauri::command]
fn send_notification(title: String, body: String) -> Result<(), String> {
    thread::spawn(move || {
        let result = Command::new("notify-send")
            .args([
                "--app-name=wngtools",
                "--urgency=normal",
                "--expire-time=5000",
                "--hint=string:sound-name:message-new-instant",
                &title,
                &body,
            ])
            .output();

        match result {
            Ok(output) if output.status.success() => {
                println!("✅ Notification sent: {}", title);
            }
            _ => {
                eprintln!("❌ Notification failed for: {}", title);
            }
        }
    });
    Ok(())
}
