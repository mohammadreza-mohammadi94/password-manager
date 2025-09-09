use crate::models::EncryptedVault;

pub struct StorageService {
    db: sled::Db,
}

impl StorageService {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("password_manager");
        std::fs::create_dir_all(&data_dir)?;
        let db = sled::open(data_dir.join("vault.db"))?;
        Ok(Self { db })
    }

    pub fn save_vault(
        &self,
        vault: &EncryptedVault,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let data = bincode::serialize(vault)?;
        self.db.insert(b"vault", data)?;
        Ok(())
    }

    pub fn load_vault(&self) -> Result<Option<EncryptedVault>, Box<dyn std::error::Error>> {
        match self.db.get(b"vault")? {
            Some(data) => {
                match bincode::deserialize(&data) {
                    Ok(vault) => Ok(Some(vault)),
                    Err(e) => {
                        // If we can't deserialize, clear the corrupt data
                        eprintln!("Error deserializing vault: {}", e);
                        self.db.remove(b"vault")?;
                        Ok(None)
                    }
                }
            }
            None => Ok(None),
        }
    }

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.db.remove(b"vault")?;
        self.db.flush()?;
        Ok(())
    }

    pub fn reset(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.db.clear()?;
        self.db.flush()?;
        Ok(())
    }
}