
use std::env;
use std::fs::{File};
use std::path::PathBuf;
use std::sync::OnceLock;
use std::path::Path;

use crate::key_derivation::verify_password;
use crate::vault_header::VaultHeader;


struct VaultEntry {}

static DEFAULT_VAULT_PATH: OnceLock<PathBuf> = OnceLock::new();

pub fn vault_path() -> &'static PathBuf {
    DEFAULT_VAULT_PATH.get_or_init(|| {
        #[cfg(target_os = "windows")]
        {
            PathBuf::from(env::var("LOCALAPPDATA").expect("Could not get LOCALAPPDATA"))
                .join("PasswordManager")
                .join("vault.bin")
        }

        #[cfg(target_os = "macos")]
        {
            PathBuf::from(env::var("HOME").expect("Could not get HOME"))
                .join("Library")
                .join("Application Support")
                .join("PasswordManager")
                .join("vault.bin")
        }

        #[cfg(target_os = "linux")]
        {
            PathBuf::from(env::var("HOME").expect("Could not get HOME"))
                .join(".local")
                .join("share")
                .join("PasswordManager")
                .join("vault.bin")
        }
    })
}

pub struct Vault {
    header: VaultHeader,
    entries: Vec<VaultEntry>,
}

impl Vault {
    // pub fn new(master_password: &str) -> Result<Self, std::io::Error> {}
    
    pub fn open_existing(master_password: &str) -> Result<Self, std::io::Error> {
        let file = File::open(vault_path())?;
        let vault_header = VaultHeader::read(file)?;

        if !verify_password(master_password, &vault_header.salt, &vault_header.verifier) {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid password"));
        }

        Ok(Vault {
            header: vault_header,
            entries: Vec::new(),
        })
    }

    pub fn file_exists() -> bool {
        let path = Path::new(vault_path());
        if path.exists() && path.is_file() {
            return true;
        }
        
        false
    }

    // pub fn save(&self, path: &str) {
    //     // Logic to save the vault to the given path
    // }

    // pub fn add_entry(&mut self, entry: VaultEntry) {
    //     self.entries.push(entry);
    // }

    // pub fn get_entries(&self) -> &Vec<VaultEntry> {
    //     &self.entries
    // }
}