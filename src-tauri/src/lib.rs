// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::{Serialize};
use tauri::{Emitter, Listener};
use tokio::time::{sleep, Duration};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn start(app_handle: tauri::AppHandle) {
    // 啟動長時間任務
    let app_handle_clone = app_handle.clone();
    tokio::spawn(async move {
        start_long_task(app_handle_clone).await;
    });
}

#[derive(Clone, Serialize)]
struct ProgressPayload {
  percentage: u8,
  message: String,
}

#[tauri::command]
async fn start_long_task(app_handle: tauri::AppHandle) {
    for i in 1..=100 {
        app_handle.emit("task-progress", ProgressPayload {
            percentage: i,
            message: format!("進度: {}%", i),
        }).unwrap();
        sleep(Duration::from_millis(50)).await;
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, start, start_long_task])
        .setup(|app| {
            app.listen("frontend-event", |event| {
                println!("Got 'frontend-event' with payload: {:?}", event.payload());

                // 啟動一個新執行緒來處理延遲任務，避免阻塞主執行緒
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_secs(3));
                    println!("延遲 3 秒後... 後端確認收到了來自前端的訊息！");
                });
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
