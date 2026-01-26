use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time::Duration;
use tauri::menu::MenuItemBuilder;
use tauri::AppHandle;
use tauri::Listener;
use tauri::WebviewWindow;
use tauri::{menu::MenuBuilder, tray::TrayIconBuilder, Manager, WindowEvent};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_dialog::FilePath;
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![send_notification, download])
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_secs(3));
                let window_clone = window.clone();
                inject_notification_js(window_clone);
                let window_clone = window.clone();
                download_hook_js(window_clone);
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

fn download_hook_js(window: WebviewWindow) {
    let js_content =
        fs::read_to_string("../src/download-hook.js").expect("download-hook.js not found!");

    window.eval(&js_content).expect("Injection failed!");
    println!("download-hook.js injected!");
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

use serde_json::Value;
use base64::{Engine as _, engine::general_purpose};

#[tauri::command]
async fn download(app: tauri::AppHandle, payload: serde_json::Value) {
    println!("Payload: {:?}", payload);
    
    let payload_str = payload.to_string();
    
    let data_start = payload_str.find("\"data\":\"").unwrap() + 8;
    let data_end = payload_str[data_start..].find("\"").unwrap();
    let data_b64 = payload_str[data_start..data_start + data_end].to_string();
    let file_name = payload["payload"]["name"].as_str().unwrap_or("attachment");

    let raw_type = payload_str.split("\"type\":")
        .nth(1).unwrap()
        .split('"').nth(1).unwrap_or("pdf")
        .split(',').next().unwrap_or("pdf")
        .to_string();

    let bytes = general_purpose::STANDARD
        .decode(data_b64)
        .expect("Base64 decode failed");


    let file_ext = if raw_type.contains("pdf") {
        "pdf"
    } else if raw_type.contains("image") {
        if raw_type.contains("png") { "png" } else { "jpg" }
    } else if raw_type.contains("zip") {
        "zip"
    } else {
        "bin"
    };
    
    let temp_file = std::env::temp_dir()
        .join(format!("outlook-download.{}", file_ext));
    
    std::fs::write(&temp_file, &bytes)
        .expect("Temp file write failed");
    
    println!("Saved Blob to: {:?}", temp_file);
    
    // Open Dialog
    open_image_dialog(app, temp_file, &format!("{}.{}", file_name, file_ext));


}