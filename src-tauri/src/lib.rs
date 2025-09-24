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

#[tauri::command]
fn read_sample_file(app: tauri::AppHandle) -> Result<String, String> {
    // Try to read from the resource directory first
    match app.path().resource_dir() {
        Ok(resource_dir) => {
            let file_path = resource_dir.join("sample.txt");
            match std::fs::read_to_string(&file_path) {
                Ok(content) => Ok(content),
                Err(_) => {
                    // Fallback to hardcoded content if file doesn't exist
                    let content = "Hello from Tauri!\nThis is a sample text file that can be read by the application.\nCurrent time: 2025-09-24\nFeature: File reading demonstration\n\nThis file demonstrates how to read text files in a Tauri application using a custom command.\n\nNote: This is fallback content since the resource file couldn't be loaded.";
                    Ok(content.to_string())
                }
            }
        }
        Err(_) => {
            // Fallback content if resource directory is not accessible
            let content = "Hello from Tauri!\nThis is a sample text file that can be read by the application.\nCurrent time: 2025-09-24\nFeature: File reading demonstration\n\nThis file demonstrates how to read text files in a Tauri application using a custom command.\n\nNote: This is fallback content since the resource directory couldn't be accessed.";
            Ok(content.to_string())
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![greet, increment_counter, get_counter, read_sample_file])
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
