use serde::{Deserialize, Serialize};

// vault data
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Vault {
    pub id: Option<i64>,
    pub name: String,
    pub desc: Option<String>,
    pub created_at: usize,
    pub last_accessed: usize,
    pub img: Option<Vec<u8>>,
}

// this is what u sent to front-end
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Card {
    pub id: Option<i64>,
    pub vault_id: i64,
    pub provider: String,
    pub data: Option<Vec<u8>>,
}

// this is what u decrypt with password on unlock vault
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VaultMd {
    pub id: i64,
    pub vault_id: i64,
    pub data: Vec<u8>,
}
