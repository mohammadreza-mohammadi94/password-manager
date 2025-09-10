use crate::models::{Credential, EncryptedVault};
use crate::storage::StorageService;
use crate::crypto::CryptoService;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct PasswordManager {
    storage: StorageService,
    credentials: Arc<Mutex<HashMap<String, Credential>>>,
    master_key: Option<Vec<u8>>,
    salt: Option<Vec<u8>>,
}

impl PasswordManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            storage: StorageService::new()?,
            credentials: Arc::new(Mutex::new(HashMap::new())),
            master_key: None,
            salt: None,
        })
    }

    pub fn unlock(&mut self, password: &str) -> Result<bool, Box<dyn std::error::Error>> {
        // Try to load existing vault
        if let Some(vault) = self.storage.load_vault()? {
            let key = CryptoService::derive_key(password.as_bytes(), &vault.salt);
            
            // Try to decrypt the vault with this key
            match CryptoService::decrypt(&vault.ciphertext, &vault.nonce, &key) {
                Ok(decrypted) => {
                    match bincode::deserialize(&decrypted) {
                        Ok(credentials) => {
                            *self.credentials.lock().unwrap() = credentials;
                            self.master_key = Some(key);
                            self.salt = Some(vault.salt.clone());
                            Ok(true)
                        },
                        Err(_) => {
                            // If we can't deserialize the decrypted data, clear the vault
                            self.storage.clear()?;
                            Ok(false)
                        }
                    }
                }
                Err(_) => Ok(false) // Wrong password
            }
        } else {
            // New vault - create with this password
            let salt = CryptoService::generate_salt();
            let key = CryptoService::derive_key(password.as_bytes(), &salt);
            self.master_key = Some(key.clone());
            self.salt = Some(salt.clone());
            
            // Initialize and save an empty vault
            let credentials = self.credentials.lock().unwrap();
            let serialized = bincode::serialize(&*credentials)?;
            let (nonce, ciphertext) = CryptoService::encrypt(&serialized, &key)?;
            let vault = EncryptedVault {
                salt,
                nonce,
                ciphertext,
            };
            self.storage.save_vault(&vault)?;
            Ok(true)
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let (Some(ref key), Some(ref salt)) = (&self.master_key, &self.salt) {
            let credentials = self.credentials.lock().unwrap();
            let serialized = bincode::serialize(&*credentials)?;
            let (nonce, ciphertext) = CryptoService::encrypt(&serialized, key)?;
            let vault = EncryptedVault {
                salt: salt.clone(),
                nonce,
                ciphertext,
            };
            self.storage.save_vault(&vault)?;
            Ok(())
        } else {
            Err("Vault is locked".into())
        }
    }

    pub fn add_password(
        &self,
        service: String,
        username: String,
        password: String,
        notes: String,
        tags: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.master_key.is_none() {
            return Err("Vault is locked".into());
        }

        let credential = Credential::new_password(service, username, password.into_bytes(), notes, tags);
        self.credentials
            .lock()
            .unwrap()
            .insert(credential.id.clone(), credential);
        self.save()?;
        Ok(())
    }

    pub fn add_api_key(
        &self,
        service: String,
        account_name: String,
        api_key: String,
        notes: String,
        is_active: bool,
        tags: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.master_key.is_none() {
            return Err("Vault is locked".into());
        }

        let credential = Credential::new_api_key(service, account_name, api_key.into_bytes(), notes, is_active, tags);
        self.credentials
            .lock()
            .unwrap()
            .insert(credential.id.clone(), credential);
        self.save()?;
        Ok(())
    }

    pub fn remove_credential(&self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.master_key.is_none() {
            return Err("Vault is locked".into());
        }

        self.credentials.lock().unwrap().remove(id);
        self.save()?;
        Ok(())
    }

    pub fn update_credential(
        &self,
        id: &str,
        service: Option<String>,
        username: Option<String>,
        secret: Option<String>,
        notes: Option<String>,
        is_active: Option<bool>,
        tags: Option<Vec<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.master_key.is_none() {
            return Err("Vault is locked".into());
        }

        // Scope the lock to release it before saving
        let updated = {
            let mut credentials = self.credentials.lock().unwrap();
            if let Some(credential) = credentials.get_mut(id) {
                credential.update(
                    service,
                    username,
                    secret.map(|s| s.into_bytes()),
                    notes,
                    is_active,
                    tags,
                );
                true
            } else {
                false
            }
        };

        if updated {
            self.save()
        } else {
            Err("Credential not found".into())
        }
    }

    pub fn get_credentials(&self) -> Result<Vec<Credential>, Box<dyn std::error::Error>> {
        if self.master_key.is_none() {
            return Err("Vault is locked".into());
        }
        Ok(self.credentials.lock().unwrap().values().cloned().collect())
    }

    // This function is currently unused but is kept for potential future features
    // like searching or viewing a single credential without loading all of them.
    #[allow(dead_code)]
    pub fn get_credential(&self, id: &str) -> Result<Option<Credential>, Box<dyn std::error::Error>> {
        if self.master_key.is_none() {
            return Err("Vault is locked".into());
        }
        Ok(self.credentials.lock().unwrap().get(id).cloned())
    }

    pub fn reset(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.storage.reset()?;
        self.credentials.lock().unwrap().clear();
        self.master_key = None;
        self.salt = None;
        Ok(())
    }

    pub fn lock(&mut self) {
        self.credentials.lock().unwrap().clear();
        self.master_key = None;
        self.salt = None;
    }
}
