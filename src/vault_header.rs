use std::io::{Read, Write};

use subtle::ConstantTimeEq;

pub struct VaultHeader {
    magic: [u8; 4],
    version: u16,
    pub salt: [u8; 16],
    pub verifier: [u8; 32],
    none: [u8; 12],

    /*
    uint8_t kdf;
    uint32_t opsLimit;
    uint32_t memLimit;
     */
}

impl VaultHeader {
    pub fn write<W: Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(self.magic.as_slice())?;
        writer.write_all(self.version.to_le_bytes().as_slice())?;
        writer.write_all(self.salt.as_slice())?;
        writer.write_all(self.verifier.as_slice())?;
        writer.write_all(self.none.as_slice())?;

        Ok(())
    }

    pub fn read<R: Read>(mut reader: R) -> std::io::Result<Self> {
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;

        if magic.ct_ne(b"PMGR").unwrap_u8() == 1 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid magic"));
        }

        let mut version = [0u8; 2];
        reader.read_exact(&mut version)?;

        if u16::from_le_bytes(version) != 1 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid version"));
        }
        
        let mut salt = [0u8; 16];
        reader.read_exact(&mut salt)?;

        let mut verifier = [0u8; 32];
        reader.read_exact(&mut verifier)?;

        let mut none = [0u8; 12];
        reader.read_exact(&mut none)?;

        Ok(Self { magic, version: u16::from_le_bytes(version), salt, verifier, none })
    }
}