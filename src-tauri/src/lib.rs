// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{Builder, AppHandle, Manager, App};
use tauri::menu::{MenuBuilder, SubmenuBuilder, Menu, MenuItem};
use tauri::tray::TrayIconBuilder;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn show_version_info(app_handle: &AppHandle) {
    let version = app_handle.package_info().version.to_string();
    let app_name = &app_handle.package_info().name;
    
    println!("應用程式版本資訊: {} v{}", app_name, version);
    
    // 嘗試透過前端顯示版本資訊
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.eval(&format!(
            "alert('應用程式版本資訊\\n{} v{}');", 
            app_name, version
        ));
    }
}

fn create_menu(app: &App) -> tauri::Result<Menu<tauri::Wry>> {
    // 建立 File submenu
    let file_submenu = SubmenuBuilder::new(app, "File")
        .text("open", "Open")
        .text("close", "Close")
        .build()?;

    // 建立 Help submenu
    let help_submenu = SubmenuBuilder::new(app, "Help")
        .text("info", "About")
        .build()?;

    // 建立主選單
    let menu = MenuBuilder::new(app)
        .item(&file_submenu)
        .item(&help_submenu)
        .build()?;

    Ok(menu)
}

fn setup_menu_handlers(app: &App) {
    app.on_menu_event(move |app_handle: &tauri::AppHandle, event| {
        println!("menu event: {:?}", event.id());

        match event.id().0.as_str() {
            "open" => {
                println!("open event");
                // 這裡可以加入開啟檔案的邏輯
            }
            "close" => {
                println!("close event");
                // 這裡可以加入關閉應用程式的邏輯
                let _ = app_handle.exit(0);
            }
            "info" => {
                println!("info event - showing version");
                show_version_info(app_handle);
            }
            _ => {
                println!("unexpected menu event");
            }
        }
    });
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
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            // 設定系統托盤圖示
            setup_tray_icon(app)?;

            // 設定視窗關閉事件處理器
            if let Some(window) = app.get_webview_window("main") {
                let window_clone = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        // 阻止預設的關閉行為
                        api.prevent_close();
                        // 隱藏視窗而不是關閉程式
                        let _ = window_clone.hide();
                    }
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
