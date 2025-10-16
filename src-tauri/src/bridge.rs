// all the bride functions goes here lol

use uuid::Uuid;

use crate::types::{CardOnCreateSchema, VaultOnCreateSchema};

// Option 2: Use enums carrying data (safer)

#[tauri::command]
pub fn add_card(data: CardOnCreateSchema) {
    println!("Hello World : {:#?}", data);
}

#[tauri::command]
pub fn add_vault(data: VaultOnCreateSchema) {
    println!("add_vault, data : {:#?}", data);
}

#[tauri::command]
pub fn unlock_vault(id: Uuid, password: String) {}

#[tauri::command]
pub fn unlock_db(password: String) {}

/// get specific functional data for specific card with id and vault_key
#[tauri::command]
pub fn fetch_card(id: Uuid, vault_key: String) {}

/// returns generic card(s) info from the selected vault
#[tauri::command]
pub fn fetch_cards(vault_key: String, lm: Option<u64>) {}

/// works if u have opened the db
#[tauri::command]
pub fn query_card(name: String) {}
