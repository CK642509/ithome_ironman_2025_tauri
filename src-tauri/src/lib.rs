// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{Builder, WebviewUrl, WebviewWindowBuilder};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn open_chat_window(
    app: tauri::AppHandle,
    partner_name: String,
) -> Result<(), String> {
    let window_label = format!("chat-{}", chrono::Utc::now().timestamp());
    let url = format!("/chat?partner={}", urlencoding::encode(&partner_name));
    let title = format!("與 {} 聊天", partner_name);

    let _window = WebviewWindowBuilder::new(
        &app,
        &window_label,
        WebviewUrl::App(url.parse().unwrap())
    )
    .title(&title)
    .inner_size(400.0, 600.0)
    .center()
    .closable(true)
    .build()
    .map_err(|e| format!("無法創建視窗: {}", e))?;

    println!("成功創建聊天視窗: {}", window_label);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, open_chat_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
