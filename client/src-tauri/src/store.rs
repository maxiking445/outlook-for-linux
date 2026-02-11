use serde_json::json;
use tauri::{AppHandle, Manager};
use tauri_plugin_store::StoreExt;

pub fn setup_store(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let store = app.store("settings.json")?;
    store.close_resource();
    Ok(())
}
