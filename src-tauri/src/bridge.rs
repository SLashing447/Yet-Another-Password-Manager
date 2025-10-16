// all the bride functions goes here lol

use crate::types::{Data, DataType};

// Option 2: Use enums carrying data (safer)

#[tauri::command]
pub fn post(data_type: DataType) {
    print!("type {:#?} and", data_type);
}
