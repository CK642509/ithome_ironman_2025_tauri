// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::{Deserialize, Serialize};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Serialize, Deserialize)]
struct UserInfo {
    name: String,
    age: u32,
    email: String,
}

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    success: bool,
    message: String,
    user_id: Option<u32>,
}

#[tauri::command]
fn create_user(user: UserInfo) -> ApiResponse {
    // 模擬建立使用者的邏輯
    println!("正在建立使用者: {} ({}歲)", user.name, user.age);
    
    ApiResponse {
        success: true,
        message: format!("使用者 {} 建立成功！", user.name),
        user_id: Some(12345),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, create_user])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
