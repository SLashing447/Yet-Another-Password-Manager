use std::{path::PathBuf, sync::Mutex};

// to talk to backend library
use tauri::{command, State};
use yapm_backend::{
    db::Database,
    types::{Card, Vault, VaultMd},
};

pub type SharedDb = Mutex<Option<Database>>;
pub const DB_FILE: &str = "this_is_actual_encrypted_database_file.lol";

#[command]
pub fn insert_vault(state: State<SharedDb>, vault: Vault, password: &str) -> Result<i64, String> {
    let db_guard = state.lock().unwrap();
    let db = db_guard.as_ref().ok_or("Database not initialized")?;

    // insert vault
    db.insert_vault(vault, password.into())
        .map_err(|e| e.to_string())
}

#[command]
pub fn insert_card(
    state: State<SharedDb>,
    provider: &str,
    data: Vec<u8>,
    vault_id: i64,
) -> Result<i64, String> {
    let db_guard = state.lock().unwrap();
    let db = db_guard.as_ref().ok_or("Database not initialized")?;

    // insert vault
    db.add_conn(provider.into(), data, vault_id)
        .map_err(|e| e.to_string())
}

#[command]
pub fn open_vault(
    state: State<SharedDb>,
    vault_id: i64,
    password: &str,
) -> Result<VaultMd, String> {
    let db_guard = state.lock().unwrap();
    let db = db_guard.as_ref().ok_or("Database not initialized")?;

    // insert vault
    db.open_vault(vault_id, password.into())
        .map_err(|e| e.to_string())
}

#[command]
pub fn get_card(state: State<SharedDb>, card_id: i64, vault_id: i64) -> Result<Card, String> {
    let db_guard = state.lock().unwrap();
    let db = db_guard.as_ref().ok_or("Database not initialized")?;

    // insert vault
    db.get_card(card_id, vault_id).map_err(|e| e.to_string())
}

#[command]
pub fn get_cards(state: State<SharedDb>, vault_id: i64) -> Result<Vec<Card>, String> {
    let db_guard = state.lock().unwrap();
    let db = db_guard.as_ref().ok_or("Database not initialized")?;

    // insert vault
    db.get_cards(vault_id).map_err(|e| e.to_string())
}

#[command]
pub fn get_vaults(state: State<SharedDb>, limit: Option<i64>) -> Result<Vec<Vault>, String> {
    let db_guard = state.lock().unwrap();
    let db = db_guard.as_ref().ok_or("Database not initialized")?;

    // insert vault
    db.get_vaults(limit).map_err(|e| e.to_string())
}

#[command]
pub fn init_db(state: State<SharedDb>, password: &str) -> Result<(), String> {
    let db = Database::new(DB_FILE, password).map_err(|e| e.to_string())?;
    *state.lock().unwrap() = Some(db);
    println!("Database initialized!");

    Ok(())
}

// utility commands
#[command]
pub fn file_exists(frontend_path: String) -> bool {
    let mut path = PathBuf::from(frontend_path);
    path.push(DB_FILE);

    path.exists()
}
