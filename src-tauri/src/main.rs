// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// use tauri::theme::Theme;


mod bridge;
mod database;
mod types;

use crate::bridge::{
    add_card, add_vault, fetch_card, fetch_cards, query_card, unlock_db, unlock_vault,
};

fn main() {
    // app_lib::run();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            add_card,
            add_vault,
            unlock_vault,
            fetch_card,
            query_card,
            unlock_db,
            fetch_cards,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri app");
}
