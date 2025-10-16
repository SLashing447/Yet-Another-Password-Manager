use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardOnCreateSchema {
    pub provider: String,
    pub username: Option<String>,
    pub email: Option<String>,
    pub desc: Option<String>,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardAppSchema {
    pub id: Uuid,
    pub provider: String,
    pub username: Option<String>,
    pub email: Option<String>,
    pub desc: Option<String>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultOnCreateSchema {
    pub name: String,
    pub desc: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultAppSchema {
    pub id: Uuid,
    pub name: String,
    pub desc: Option<String>,
    pub created_at: u64,
    pub last_accesed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    Card,
    Vault,
}
