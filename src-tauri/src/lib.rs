// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{Builder, AppHandle, Manager, App};
use tauri_plugin_store::StoreExt;
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

#[tauri::command]
fn greet(name: &str, app: AppHandle) -> Result<String, String> {
    // 取得 store 實例
    let store = app.store("user-data.json").map_err(|e| e.to_string())?;
    
    // 儲存姓名到 store
    store.set("name", serde_json::Value::String(name.to_string()));
    
    // 儲存 store 到檔案
    if let Err(e) = store.save() {
        return Err(format!("Failed to save store: {}", e));
    }
    
    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {

            let ans = app.dialog()
                .message("File not found")
                .kind(MessageDialogKind::Error)
                .title("Warning")
                .blocking_show();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
