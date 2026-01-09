use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::OnceLock;

use aes_gcm::Aes256Gcm;
use aes_gcm::KeyInit;
use aes_gcm::Nonce;
use aes_gcm::aead::Aead;
use rand::Rng;

use crate::key_derivation;
use crate::vault_entry::VaultEntry;
use crate::vault_header::VaultHeader;

static DEFAULT_VAULT_PATH: OnceLock<PathBuf> = OnceLock::new();

fn vault_path() -> &'static PathBuf {
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

fn ensure_parents_exist() -> std::io::Result<()> {
    let path = Path::new(vault_path());

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    Ok(())
}

pub struct Vault {
    header: VaultHeader,
    entries: Vec<VaultEntry>,
}

impl Vault {
    pub fn new(master_password: &str) -> Result<Self, std::io::Error> {
        let (derive_key, salt) = key_derivation::derive_key(master_password);
        let verifier = key_derivation::create_verifier(&derive_key);

        let mut nonce = [0u8; 12];
        rand::thread_rng().fill(&mut nonce);

        let vault = Vault {
            header: VaultHeader::new(salt, verifier, nonce),
            entries: Vec::new(),
        };

        vault.save()?;
        Ok(vault)
    }

    pub fn open_existing(master_password: &str) -> Result<Self, std::io::Error> {
        let mut file = File::open(vault_path())?;
        let vault_header = VaultHeader::read(&file)?;

        if !key_derivation::verify_password(
            master_password,
            &vault_header.salt,
            &vault_header.verifier,
        ) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid password",
            ));
        }

        let mut data = Vec::<u8>::new();
        file.read_to_end(&mut data)?;

        let decrypted_data = Self::decrypt(&vault_header.none, &data)?;
        let entries = Self::deserialize(&decrypted_data)?;

        Ok(Vault {
            header: vault_header,
            entries: entries,
        })
    }

    pub fn file_exists() -> bool {
        let path = Path::new(vault_path());
        if path.exists() && path.is_file() {
            return true;
        }

        false
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        ensure_parents_exist()?;
        let mut file = File::create(vault_path())?;
        self.header.write(&file)?;
        let data = self.serialize();
        let encrypted_data = self.encrypt(&data)?;
        file.write_all(&encrypted_data)?;

        Ok(())
    }

    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::<u8>::new();

        data.push(self.entries.len() as u8);
        let mut data = Vec::<u8>::new();
        data.push(self.entries.len() as u8);

        for entry in &self.entries {
            let serialized_entry = entry.serialize();
            data.extend_from_slice(&serialized_entry);
        }

        data
    }

    fn deserialize(_data: &[u8]) -> Result<Vec<VaultEntry>, std::io::Error> {
        let mut index = 0;
        let entries_count = _data[index] as usize;
        index += 1;

        let mut entries = Vec::<VaultEntry>::new();

        for _ in 0..entries_count {
            let entry = VaultEntry::deserialize(&_data[index..]);
            index += 1
                + entry.service.len()
                + 1
                + entry.username.as_ref().map_or(0, |u| u.len())
                + 1
                + entry.password.len();
            entries.push(entry);
        }

        Ok(entries)
    }

    fn encrypt(&self, _data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
        let cipher = Aes256Gcm::new_from_slice(b"ma_cle_secrete012345678915478963").unwrap();
        let nonce = Nonce::from_slice(&self.header.none);
        cipher
            .encrypt(nonce, _data)
            .map_err(|_e| std::io::Error::new(std::io::ErrorKind::Other, "Encryption error"))
    }

    fn decrypt(nonce_bytes: &[u8; 12], _data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
        let cipher = Aes256Gcm::new_from_slice(b"ma_cle_secrete012345678915478963").unwrap();
        let nonce = Nonce::from_slice(nonce_bytes.as_slice());
        cipher
            .decrypt(nonce, _data)
            .map_err(|_e| std::io::Error::new(std::io::ErrorKind::Other, "Decryption error"))
    }

    pub fn add_entry(&mut self, entry: VaultEntry) {
        self.entries.push(entry);
        self.save().expect("Error saving vault after adding entry");
    }

    pub fn get_entries(&self) -> &Vec<VaultEntry> {
        &self.entries
    }

    pub fn get_entry(&self, index: usize) -> Option<&VaultEntry> {
        self.entries.get(index).to_owned()
    }
}
