use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, rand_core::RngCore},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Encryption error: {0}")]
    Encryption(String),
    #[error("Decryption error: {0}")]
    Decryption(String),
    #[error("Config directory not found")]
    ConfigDirNotFound,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailSettings {
    pub smtp_host: String,
    pub smtp_port: String,
    pub from_email: String,
    pub to_email: String,
    pub app_password: String,
}

impl Default for EmailSettings {
    fn default() -> Self {
        Self {
            smtp_host: "smtp.gmail.com".to_string(),
            smtp_port: "587".to_string(),
            from_email: String::new(),
            to_email: String::new(),
            app_password: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct EncryptedConfig {
    data: String,
    nonce: String,
}

#[derive(Clone)]
pub struct ConfigManager {
    config_path: PathBuf,
    key: [u8; 32],
}

impl ConfigManager {
    pub fn new() -> Result<Self, ConfigError> {
        let config_dir = dirs::config_dir()
            .ok_or(ConfigError::ConfigDirNotFound)?
            .join("send-to-goodnotes");
        
        fs::create_dir_all(&config_dir)?;
        
        let config_path = config_dir.join("settings.json");
        
        // Generate a simple key based on machine characteristics
        // In a real app, you might want to use a more sophisticated key derivation
        let machine_id = whoami::username();
        let mut key = [0u8; 32];
        let machine_bytes = machine_id.as_bytes();
        for (i, &byte) in machine_bytes.iter().enumerate() {
            if i >= 32 { break; }
            key[i] = byte;
        }
        
        // Fill remaining bytes with a pattern if machine_id is short
        for (i, slot) in key.iter_mut().enumerate().skip(machine_bytes.len()) {
            *slot = (i as u8).wrapping_mul(7).wrapping_add(42);
        }
        
        Ok(Self { config_path, key })
    }
    
    pub fn load_settings(&self) -> Result<EmailSettings, ConfigError> {
        if !self.config_path.exists() {
            return Ok(EmailSettings::default());
        }
        
        let encrypted_data = fs::read_to_string(&self.config_path)?;
        let encrypted_config: EncryptedConfig = serde_json::from_str(&encrypted_data)?;
        
        let cipher = Aes256Gcm::new_from_slice(&self.key)
            .map_err(|e| ConfigError::Decryption(e.to_string()))?;
        
        let nonce_bytes = general_purpose::STANDARD
            .decode(&encrypted_config.nonce)
            .map_err(|e| ConfigError::Decryption(e.to_string()))?;
        
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let encrypted_bytes = general_purpose::STANDARD
            .decode(&encrypted_config.data)
            .map_err(|e| ConfigError::Decryption(e.to_string()))?;
        
        let decrypted_bytes = cipher
            .decrypt(nonce, encrypted_bytes.as_ref())
            .map_err(|e| ConfigError::Decryption(e.to_string()))?;
        
        let settings: EmailSettings = serde_json::from_slice(&decrypted_bytes)?;
        Ok(settings)
    }
    
    pub fn save_settings(&self, settings: &EmailSettings) -> Result<(), ConfigError> {
        let cipher = Aes256Gcm::new_from_slice(&self.key)
            .map_err(|e| ConfigError::Encryption(e.to_string()))?;
        
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let settings_json = serde_json::to_vec(settings)?;
        
        let encrypted_bytes = cipher
            .encrypt(nonce, settings_json.as_ref())
            .map_err(|e| ConfigError::Encryption(e.to_string()))?;
        
        let encrypted_config = EncryptedConfig {
            data: general_purpose::STANDARD.encode(&encrypted_bytes),
            nonce: general_purpose::STANDARD.encode(nonce),
        };
        
        let encrypted_json = serde_json::to_string_pretty(&encrypted_config)?;
        fs::write(&self.config_path, encrypted_json)?;
        
        Ok(())
    }
}
