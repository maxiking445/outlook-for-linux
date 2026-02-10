use tauri::menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder};
use tauri::tray::TrayIconBuilder;
use tauri::Manager;

use crate::commands::open_in_browser;

pub fn setup_tray_menu(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let hide = MenuItemBuilder::new("Hide").id("hide").build(app)?;
    let report = MenuItemBuilder::new("Report an Issue")
        .id("report")
        .build(app)?;
    let help_menu = SubmenuBuilder::new(app, "Help")
        .items(&[&report])
        .build()?;
    let show = MenuItemBuilder::new("Show").id("show").build(app)?;
    let quit = MenuItemBuilder::new("Quit").id("quit").build(app)?;
    
    let menu = MenuBuilder::new(app)
        .items(&[&hide, &show, &help_menu, &quit])
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