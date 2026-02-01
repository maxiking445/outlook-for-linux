use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time::Duration;
use tauri::menu::MenuItemBuilder;
use tauri::WebviewWindow;
use tauri::{menu::MenuBuilder, tray::TrayIconBuilder, Manager, WindowEvent};
use tauri_plugin_dialog::DialogExt;
use tauri::path::BaseDirectory;
use tauri::webview::DownloadEvent;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
                if let Some(window) = app.get_webview_window("main") {
                    window.set_focus().unwrap();
                    window.show().unwrap();
                }
        }))
        
        .invoke_handler(tauri::generate_handler![send_notification])
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.minimize().unwrap();
            }
        })
        
        .setup(|app| {
            let builder = tauri::WebviewWindowBuilder::from_config(
                app.handle(),
                &app.config().app.windows[0],
            )?;

            builder
                .on_download(|_webview, event| {
                    match event {
                        DownloadEvent::Requested { url, destination } => {
                            println!("Download requested: {}", url);
                            *destination = std::env::temp_dir().join(destination.file_name().unwrap());
                        }
                        DownloadEvent::Finished { path, success, .. } => {
                            println!("Download finished: {:?}, success={}", path, success);

                            if let Some(path) = path {
                                let app_handle = _webview.app_handle().clone();
                                let file_name = path.file_name()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                                    .to_string();

                                open_image_dialog(app_handle, path.clone(), &file_name);
                            } else {
                                eprintln!("Download finihsed, with invalid path!!");
                            }
                        }
                        _ => {}
                    }
                    true 
                })
                .build()?;


            let window = app.get_webview_window("main").unwrap();

            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_secs(3));

                inject_js_resource(&window, "notification.js")
                    .expect("failed to inject notification.js");
                inject_js_resource(&window, "notification-extractor.js")
                    .expect("failed to inject notification-extractor.js");
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
                        window.minimize().unwrap();
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
        })
        .run(tauri::generate_context!())
        .expect("Something wrong when running tauri application");
}


pub fn inject_js_resource(
    window: &WebviewWindow,
    relative_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {

    let app = window.app_handle();
    let path = app
        .path()
        .resolve(relative_path, BaseDirectory::Resource)?;

    let js_content = fs::read_to_string(&path)?;
    window.eval(&js_content)?;
    println!("injected resource JS: {}", relative_path);
    Ok(())
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

pub fn open_image_dialog(app: tauri::AppHandle, source_file: PathBuf, file_name: &str) {
    app.dialog().file()
        .set_file_name(file_name)
        .save_file(move |target_path| {
            if let Some(target) = target_path {
                match target {
                    tauri_plugin_dialog::FilePath::Path(path) => {
                        if let Err(err) = std::fs::copy(&source_file, &path) {
                            eprintln!("Copy failed!: {}", err);
                        } else {
                            println!("Data saved under: {:?}", path);
                        }
                    }
                    tauri_plugin_dialog::FilePath::Url(url) => {
                        eprintln!("URL Path not supported!: {}", url);
                    }
                }
            }
        });
}