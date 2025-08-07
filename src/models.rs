use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    pub id: String,
    pub service: String,
    pub username: String,
    pub password: Vec<u8>, // Encrypted
    pub notes: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Credential {
    pub fn new(service: String, username: String, password: Vec<u8>, notes: String) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            service,
            username,
            password,
            notes,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedVault {
    pub salt: Vec<u8>,
    pub nonce: Vec<u8>,
    pub ciphertext: Vec<u8>,
}