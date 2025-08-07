use ring::{aead, pbkdf2, rand::{self, SecureRandom}};
use std::num::NonZeroU32;

pub struct CryptoService;

impl CryptoService {
    pub fn derive_key(password: &[u8], salt: &[u8]) -> Vec<u8> {
        let mut key = [0u8; 32];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            NonZeroU32::new(100_000).unwrap(),
            salt,
            password,
            &mut key,
        );
        key.to_vec()
    }

    pub fn encrypt(
        data: &[u8],
        key: &[u8],
    ) -> Result<(Vec<u8>, Vec<u8>), Box<dyn std::error::Error>> {
        let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, key)
            .map_err(|_| "Key error")?;
            
        // Use LessSafeKey which provides the encryption methods we need
        let sealing_key = aead::LessSafeKey::new(unbound_key);
        
        // Generate a random nonce using SystemRandom
        let mut nonce_bytes = [0u8; 12];
        rand::SystemRandom::new()
            .fill(&mut nonce_bytes)
            .map_err(|_| "Nonce generation error")?;
            
        let nonce = aead::Nonce::assume_unique_for_key(nonce_bytes);
        
        // Prepare buffer with data
        let mut in_out = data.to_vec();
        
        // Encrypt and get the tag separately
        let tag = sealing_key.seal_in_place_separate_tag(nonce, aead::Aad::empty(), &mut in_out)
            .map_err(|_| "Encryption error")?;
            
        // Append the tag to the ciphertext
        in_out.extend_from_slice(tag.as_ref());
        
        Ok((nonce_bytes.to_vec(), in_out))
    }

    pub fn decrypt(
        ciphertext: &[u8],
        nonce: &[u8],
        key: &[u8],
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if nonce.len() != 12 {
            return Err("Invalid nonce length".into());
        }
        
        // Need to separate the tag from the ciphertext
        if ciphertext.len() < 16 { // AES-GCM tag is 16 bytes
            return Err("Ciphertext too short".into());
        }
        
        let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, key)
            .map_err(|_| "Key error")?;
            
        let opening_key = aead::LessSafeKey::new(unbound_key);
        let nonce_bytes: [u8; 12] = nonce.try_into().map_err(|_| "Invalid nonce")?;
        let nonce = aead::Nonce::assume_unique_for_key(nonce_bytes);
        
        // Clone the ciphertext for in-place decryption
        let mut in_out = ciphertext.to_vec();
        
        // Decrypt in place
        let plaintext = opening_key.open_in_place(nonce, aead::Aad::empty(), &mut in_out)
            .map_err(|_| "Decryption error")?;
            
        Ok(plaintext.to_vec())
    }

    pub fn generate_salt() -> Vec<u8> {
        // Generate salt using SystemRandom
        let mut salt = [0u8; 32];
        rand::SystemRandom::new()
            .fill(&mut salt)
            .expect("Salt generation failed");
        salt.to_vec()
    }
}