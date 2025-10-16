use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardSchema {
    pub provider: String,
    pub username: Option<String>,
    pub email: Option<String>,
    pub desc: Option<String>,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Data {
    Card(CardSchema),
    Vault(VaultSchema),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultSchema {
    pub name: String,
    pub desc: Option<String>,
    pub email: Option<String>,
    pub last_accesed: u64,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    Card,
    Vault,
}
