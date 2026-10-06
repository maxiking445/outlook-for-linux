use std::fs;
use std::net::TcpStream;
use std::path::PathBuf;
use std::time::Duration;
use tauri::path::BaseDirectory;
use tauri::webview::DownloadEvent;
use tauri::{AppHandle, Manager, Url, WebviewWindow, Window, WindowEvent};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_store::StoreExt;

const MINIMIZED_ARG: &str = "--minimized";

pub fn is_minimized_arg_set(args: impl IntoIterator<Item = impl AsRef<str>>) -> bool {
    args.into_iter().any(|arg| arg.as_ref() == MINIMIZED_ARG)
}

pub fn handle_window_event(window: &Window, event: &WindowEvent) {
    match event {
        WindowEvent::CloseRequested { api, .. } => {
            let store = window.app_handle().store("settings.json").unwrap();
            let quit_on_close = store
                .get("quit_on_close")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            if quit_on_close {
                window.app_handle().exit(0);
            } else {
                api.prevent_close();
                let _ = window.hide();
            }
        }
        WindowEvent::Focused(false) => {
            if window.is_minimized().unwrap_or(false) {
                let store = window.app_handle().store("settings.json").unwrap();
                let minimize_to_bg = store
                    .get("minimize_to_background")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                if minimize_to_bg {
                    let _ = window.hide();
                }
            }
        }
        _ => {}
    }
}

pub fn setup_window(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let mut builder =
        tauri::WebviewWindowBuilder::from_config(app.handle(), &app.config().app.windows[0])?;

    if is_minimized_arg_set(std::env::args()) {
        builder = builder.visible(false);
    }

    builder
        .initialization_script(format!(
            r#"(() => {{
                if (!["outlook.office.com", "outlook.office365.com", "outlook.live.com", "outlook.cloud.microsoft"].includes(location.hostname)) return;
                const install = () => {{
                    {}
                    {}
                }};
                if (document.readyState === "loading") {{
                    document.addEventListener("DOMContentLoaded", install, {{ once: true }});
                }} else {{
                    install();
                }}
            }})();"#,
            include_str!("../../src/notification-extractor.js"),
            include_str!("../../src/notification.js"),
        ))
        .on_page_load(|window, payload| {
            if payload.event() == tauri::webview::PageLoadEvent::Finished
                && matches!(
                    payload.url().host_str(),
                    Some("outlook.office.com" | "outlook.office365.com" | "outlook.live.com" | "outlook.cloud.microsoft")
                )
            {
                inject_js_files(window.clone());
            }
        })
        .on_download(|webview, event| {
            handle_download_event(webview.app_handle().clone(), event);
            true
        })
        .build()?;

    let window = app.get_webview_window("main").unwrap();

    if !check_internet() {
        let offline_path = app
            .handle()
            .path()
            .resolve("offline.html", BaseDirectory::Resource)?;
        let offline_url = Url::from_file_path(&offline_path)
            .map_err(|_| format!("Invalid path: {:?}", offline_path))?;
        window.navigate(offline_url)?;
    }

    Ok(())
}

fn handle_download_event(app_handle: AppHandle, event: DownloadEvent) {
    match event {
        DownloadEvent::Requested { url, destination } => {
            println!("Download requested: {}", url);
            let file_name = destination
                .file_name()
                .map(|name| name.to_owned())
                .unwrap_or_else(|| "download".into());
            *destination = std::env::temp_dir().join(file_name);
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

fn check_internet() -> bool {
    use std::net::ToSocketAddrs;
    let addr = match "outlook.office.com:443".to_socket_addrs() {
        Ok(mut addrs) => match addrs.next() {
            Some(addr) => addr,
            None => return false,
        },
        Err(_) => return false,
    };
    TcpStream::connect_timeout(&addr, Duration::from_secs(3)).is_ok()
}

fn inject_js_files(window: WebviewWindow) {
    for resource in [
        "notification-extractor.js",
        "notification.js",
        "url-change.js",
    ] {
        if let Err(error) = inject_js_resource(&window, resource) {
            eprintln!("Failed to inject {resource}: {error}");
        }
    }
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
    let download_dir = app
        .path()
        .download_dir()
        .unwrap_or_else(|_| std::env::temp_dir());

    app.dialog()
        .file()
        .set_directory(download_dir)
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
