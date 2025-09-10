use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EntryType {
    Password,
    ApiKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    pub id: String,
    pub entry_type: EntryType,
    pub service: String,
    pub username: String,
    pub secret: Vec<u8>, // Encrypted (password or API key)
    pub notes: String,
    pub tags: Vec<String>,
    pub is_active: bool, // Used for API keys
    pub custom_fields: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Credential {
    pub fn new_password(service: String, username: String, password: Vec<u8>, notes: String, tags: Vec<String>) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            entry_type: EntryType::Password,
            service,
            username,
            secret: password,
            notes,
            tags,
            is_active: true,
            custom_fields: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn new_api_key(service: String, account_name: String, api_key: Vec<u8>, notes: String, is_active: bool, tags: Vec<String>) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            entry_type: EntryType::ApiKey,
            service,
            username: account_name,
            secret: api_key,
            notes,
            tags,
            is_active,
            custom_fields: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update(&mut self, service: Option<String>, username: Option<String>, secret: Option<Vec<u8>>, 
                 notes: Option<String>, is_active: Option<bool>, tags: Option<Vec<String>>, custom_fields: Option<HashMap<String, String>>) {
        if let Some(s) = service {
            self.service = s;
        }
        if let Some(u) = username {
            self.username = u;
        }
        if let Some(p) = secret {
            self.secret = p;
        }
        if let Some(n) = notes {
            self.notes = n;
        }
        if let Some(a) = is_active {
            self.is_active = a;
        }
        if let Some(t) = tags {
            self.tags = t;
        }
        if let Some(cf) = custom_fields {
            self.custom_fields = cf;
        }
        self.updated_at = Utc::now();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedVault {
    pub salt: Vec<u8>,
    pub nonce: Vec<u8>,
    pub ciphertext: Vec<u8>,
}
