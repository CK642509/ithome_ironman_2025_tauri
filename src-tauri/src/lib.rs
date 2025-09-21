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

#[tauri::command]
async fn check_file_exists(path: String) -> Result<bool, String> {
    // 這裡是一個模擬的耗時操作
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    if std::path::Path::new(&path).exists() {
        Ok(true)
    } else {
        // 當回傳 Err 時，前端的 invoke Promise 會被 reject
        Err("找不到指定的檔案".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, create_user, check_file_exists])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
