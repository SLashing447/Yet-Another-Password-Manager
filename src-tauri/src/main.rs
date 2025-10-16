// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod bridge;
mod database;
mod types;

use crate::bridge::post;

fn main() {
    // app_lib::run();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![post])
        .run(tauri::generate_context!())
        .expect("error while running Tauri app");
}
