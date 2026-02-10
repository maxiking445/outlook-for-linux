use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use tauri::path::BaseDirectory;
use tauri::webview::DownloadEvent;
use tauri::{AppHandle, Manager, Window, WebviewWindow, WindowEvent};
use tauri_plugin_dialog::DialogExt;

pub fn handle_window_event(window: &Window, event: &WindowEvent) {
    if let WindowEvent::CloseRequested { api, .. } = event {
        api.prevent_close();
        let _ = window.minimize().unwrap();
    }
}

pub fn setup_window(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let builder = tauri::WebviewWindowBuilder::from_config(
        app.handle(),
        &app.config().app.windows[0],
    )?;

    builder
        .on_download(|webview, event| {
            handle_download_event(webview.app_handle().clone(), event);
            true
        })
        .build()?;

    let window = app.get_webview_window("main").unwrap();
    inject_js_files(window);

    Ok(())
}

fn handle_download_event(app_handle: AppHandle, event: DownloadEvent) {
    match event {
        DownloadEvent::Requested { url, destination } => {
            println!("Download requested: {}", url);
            *destination = std::env::temp_dir().join(destination.file_name().unwrap());
        }
        DownloadEvent::Finished { path, success, .. } => {
            println!("Download finished: {:?}, success={}", path, success);

            if let Some(path) = path {
                let app_handle = app_handle.clone();
                let file_name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                open_image_dialog(app_handle, path.clone(), &file_name);
            } else {
                eprintln!("Download finished, with invalid path!!");
            }
        }
        _ => {}
    }
}

fn inject_js_files(window: WebviewWindow) {
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(3));

        inject_js_resource(&window, "notification.js")
            .expect("failed to inject notification.js");
        inject_js_resource(&window, "notification-extractor.js")
            .expect("failed to inject notification-extractor.js");
        inject_js_resource(&window, "url-change.js")
            .expect("failed to inject url-change.js");
    });
}

fn inject_js_resource(
    window: &WebviewWindow,
    relative_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let app = window.app_handle();
    let path = app.path().resolve(relative_path, BaseDirectory::Resource)?;

    let js_content = fs::read_to_string(&path)?;
    window.eval(&js_content)?;
    println!("injected resource JS: {}", relative_path);
    Ok(())
}

fn open_image_dialog(app: AppHandle, source_file: PathBuf, file_name: &str) {
    app.dialog()
        .file()
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