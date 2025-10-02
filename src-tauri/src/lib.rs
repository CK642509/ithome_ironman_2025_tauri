// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{Builder, AppHandle, Manager, App};
use tauri::menu::{MenuBuilder, SubmenuBuilder, Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
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

fn create_tray_menu(app: &App) -> tauri::Result<Menu<tauri::Wry>> {
    let open_i = MenuItem::with_id(app, "open", "Open", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&open_i, &quit_i])?;
    Ok(menu)
}

fn setup_tray_icon(app: &App) -> tauri::Result<()> {
    let tray_menu = create_tray_menu(app)?;

    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&tray_menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "open" => {
                println!("open menu item was clicked");
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "quit" => {
                println!("quit menu item was clicked");
                app.exit(0);
            }
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
        .show_menu_on_left_click(false)
        .build(app)?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            // 設定系統托盤圖示
            setup_tray_icon(app)?;

            let ans = app.dialog()
                .message("File not found")
                .kind(MessageDialogKind::Error)
                .title("Warning")
                .blocking_show();

            Ok(())
        })
        .on_window_event(|window, event| {
            // 設定視窗關閉事件處理器
            match event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    window.hide().unwrap();
                    api.prevent_close();
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
