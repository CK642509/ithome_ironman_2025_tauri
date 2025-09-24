// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::sync::Mutex;
use tauri::{Builder, Manager, State};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Default)]
struct AppState {
    counter: i32,
}

#[tauri::command]
fn increment_counter(state: State<Mutex<AppState>>) -> i32 {
    let mut app_state = state.lock().unwrap();
    app_state.counter += 1;
    app_state.counter
}

#[tauri::command]
fn get_counter(state: State<Mutex<AppState>>) -> i32 {
    let app_state = state.lock().unwrap();
    app_state.counter
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![greet, increment_counter, get_counter])
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
