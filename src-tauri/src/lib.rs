// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{Builder, AppHandle, Manager};
use tauri::menu::{MenuBuilder, SubmenuBuilder};

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
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

            app.set_menu(menu)?;

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

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
