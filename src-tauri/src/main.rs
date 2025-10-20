// Preventkeys additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

mod bridge;

use bridge::{
    file_exists, get_card, get_cards, get_vaults, init_db, insert_card, insert_vault, open_vault,
    SharedDb,
};

fn main() {
    // app_lib::run();
    tauri::Builder::default()
        .manage(Mutex::new(None) as SharedDb) // initially no DB
        .invoke_handler(tauri::generate_handler![
            init_db,
            insert_vault,
            open_vault,
            get_card,
            get_cards,
            get_vaults,
            insert_card,
            file_exists
        ])
        .run(tauri::generate_context!())
        .expect("error running app");
}
