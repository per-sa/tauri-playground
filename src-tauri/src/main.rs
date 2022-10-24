#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

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

#[tauri::command]
async fn get_crypto(name: String) -> String {
    println!("get_crypto called with {}", name);
    let url = format!("https://api.coincap.io/v2/assets/{}", name);
    let res = reqwest::get(url);
    let body = res.await;
    if body.is_err() {
        return "Error".to_string();
    }

    let body = body.unwrap().text().await;
    if body.is_err() {
        return "Error".to_string();
    }

    let body = body.unwrap();
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    let price = json["data"]["priceUsd"].as_str().unwrap();
    let price = price.parse::<f64>().unwrap();
    let symbol = json["data"]["symbol"].as_str().unwrap();
    let name = json["data"]["name"].as_str().unwrap();
    let change = json["data"]["changePercent24Hr"].as_str().unwrap();

    format!(
        "Name: {}, Symbol: {}, Price: {}, Change 24H: {}",
        name, symbol, price, change
    )
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![calculate_comp, get_crypto])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
