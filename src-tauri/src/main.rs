#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rand::thread_rng;
use rand::Rng;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn calculate_comp(name: &str, name2: &str) -> String {
    format!(
        "{} and {}, are {}% compatible",
        name,
        name2,
        rand::thread_rng().gen_range(0..100)
    )
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![calculate_comp])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
