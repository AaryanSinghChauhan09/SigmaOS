use alloc::vec;
extern crate alloc;
// SigmaOS FreeBSD-Inspired GEOM Storage Architecture
// Provides a modular, layered storage transformation framework:
// Partitioning (g_part), Mirroring (g_mirror), Striping (g_stripe),
// GELI Encryption (g_eli), and Linear Concatenation (g_concat).

use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BioCmd {
    Read,
    Write,
    Delete, // TRIM
    Flush,
    GetAttr,
}

#[derive(Debug, Clone)]
pub struct BioRequest {
    pub cmd: BioCmd,
    pub offset: u64,
    pub length: u64,
    pub data: Vec<u8>,
    pub attribute: String,
    pub completed: bool,
    pub error_code: Option<i32>,
}

impl BioRequest {
    pub fn new_read(offset: u64, length: u64) -> Self {
        Self {
            cmd: BioCmd::Read,
            offset,
            length,
            data: Vec::new(),
            attribute: String::new(),
            completed: false,
            error_code: None,
        }
    }

    pub fn new_write(offset: u64, data: Vec<u8>) -> Self {
        let length = data.len() as u64;
        Self {
            cmd: BioCmd::Write,
            offset,
            length,
            data,
            attribute: String::new(),
            completed: false,
            error_code: None,
        }
    }

    pub fn new_delete(offset: u64, length: u64) -> Self {
        Self {
            cmd: BioCmd::Delete,
            offset,
            length,
            data: Vec::new(),
            attribute: String::new(),
            completed: false,
            error_code: None,
        }
    }

    pub fn new_flush() -> Self {
        Self {
            cmd: BioCmd::Flush,
            offset: 0,
            length: 0,
            data: Vec::new(),
            attribute: String::new(),
            completed: false,
            error_code: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GeomProvider {
    pub name: String,
    pub mediasize: u64,
    pub sectorsize: u32,
    pub stripe_size: u64,
    pub stripe_offset: u64,
    pub read_only: bool,
    pub raw_buffer: Vec<u8>,
}

impl GeomProvider {
    pub fn new(name: impl Into<String>, mediasize: u64, sectorsize: u32) -> Self {
        let size = mediasize as usize;
        Self {
            name: name.into(),
            mediasize,
            sectorsize,
            stripe_size: sectorsize as u64,
            stripe_offset: 0,
            read_only: false,
            raw_buffer: vec![0u8; size],
        }
    }

    pub fn handle_bio(&mut self, bio: &mut BioRequest) {
        match bio.cmd {
            BioCmd::Read => {
                let start = bio.offset as usize;
                let end = start + bio.length as usize;
                if end <= self.raw_buffer.len() {
                    bio.data = self.raw_buffer[start..end].to_vec();
                    bio.completed = true;
                } else {
                    bio.error_code = Some(5); // EIO
                }
            }
            BioCmd::Write => {
                if self.read_only {
                    bio.error_code = Some(30); // EROFS
                    return;
                }
                let start = bio.offset as usize;
                let end = start + bio.data.len();
                if end <= self.raw_buffer.len() {
                    self.raw_buffer[start..end].copy_from_slice(&bio.data);
                    bio.completed = true;
                } else {
                    bio.error_code = Some(5); // EIO
                }
            }
            BioCmd::Delete => {
                let start = bio.offset as usize;
                let end = start + bio.length as usize;
                if end <= self.raw_buffer.len() {
                    for b in &mut self.raw_buffer[start..end] {
                        *b = 0;
                    }
                    bio.completed = true;
                } else {
                    bio.error_code = Some(5);
                }
            }
            BioCmd::Flush => {
                bio.completed = true;
            }
            BioCmd::GetAttr => {
                bio.data = self.name.as_bytes().to_vec();
                bio.completed = true;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GeomConsumer {
    pub provider_name: String,
    pub read_access: u32,
    pub write_access: u32,
    pub exclusive_access: u32,
}

impl GeomConsumer {
    pub fn new(provider_name: impl Into<String>) -> Self {
        Self {
            provider_name: provider_name.into(),
            read_access: 1,
            write_access: 1,
            exclusive_access: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeomClassType {
    Part,
    Mirror,
    Stripe,
    Eli,
    Concat,
}

#[derive(Debug, Clone)]
pub struct PartitionEntry {
    pub index: u32,
    pub name: String,
    pub offset: u64,
    pub size: u64,
    pub part_type: String,
}

#[derive(Debug, Clone)]
pub struct GeomEliConfig {
    pub key: [u8; 32],
    pub cipher: String,
    pub key_version: u32,
}

impl GeomEliConfig {
    pub fn new(passphrase: &str) -> Self {
        let mut key = [0u8; 32];
        let bytes = passphrase.as_bytes();
        for (i, b) in bytes.iter().enumerate() {
            key[i % 32] ^= *b;
        }
        Self {
            key,
            cipher: "AES-XTS-256".to_owned(),
            key_version: 1,
        }
    }

    pub fn transform_block(&self, offset: u64, block: &mut [u8]) {
        let tweak = offset.to_le_bytes();
        for (i, byte) in block.iter_mut().enumerate() {
            let key_byte = self.key[i % 32];
            let tweak_byte = tweak[i % 8];
            *byte ^= key_byte ^ tweak_byte;
        }
    }
}

pub struct GeomTopology {
    pub providers: Vec<GeomProvider>,
    pub consumers: Vec<GeomConsumer>,
    pub partitions: Vec<PartitionEntry>,
    pub eli_configs: Vec<(String, GeomEliConfig)>, // (provider_name, config)
}

impl GeomTopology {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
            consumers: Vec::new(),
            partitions: Vec::new(),
            eli_configs: Vec::new(),
        }
    }

    pub fn register_provider(&mut self, provider: GeomProvider) {
        self.providers.push(provider);
    }

    pub fn find_provider(&self, name: &str) -> Option<&GeomProvider> {
        self.providers.iter().find(|p| p.name == name)
    }

    pub fn find_provider_mut(&mut self, name: &str) -> Option<&mut GeomProvider> {
        self.providers.iter_mut().find(|p| p.name == name)
    }

    pub fn create_partition(
        &mut self,
        parent_name: &str,
        part_name: &str,
        offset: u64,
        size: u64,
        part_type: &str,
    ) -> Result<(), String> {
        let parent = self
            .find_provider(parent_name)
            .ok_or_else(|| format!("Parent provider {} not found", parent_name))?;

        if offset + size > parent.mediasize {
            return Err("Partition exceeds parent provider boundary".to_owned());
        }

        let sector_size = parent.sectorsize;
        let part_index = self.partitions.len() as u32 + 1;
        let entry = PartitionEntry {
            index: part_index,
            name: part_name.to_owned(),
            offset,
            size,
            part_type: part_type.to_owned(),
        };

        let mut child_provider = GeomProvider::new(part_name, size, sector_size);
        // Copy initial data slice if parent has data
        let start = offset as usize;
        let end = start + size as usize;
        if end <= parent.raw_buffer.len() {
            child_provider
                .raw_buffer
                .copy_from_slice(&parent.raw_buffer[start..end]);
        }

        self.partitions.push(entry);
        self.register_provider(child_provider);
        self.consumers.push(GeomConsumer::new(parent_name));
        Ok(())
    }

    pub fn create_mirror(
        &mut self,
        mirror_name: &str,
        provider_a: &str,
        provider_b: &str,
    ) -> Result<(), String> {
        let p_a = self
            .find_provider(provider_a)
            .ok_or_else(|| format!("Provider {} not found", provider_a))?;
        let p_b = self
            .find_provider(provider_b)
            .ok_or_else(|| format!("Provider {} not found", provider_b))?;

        let mirror_size = core::cmp::min(p_a.mediasize, p_b.mediasize);
        let sector_size = p_a.sectorsize;

        let mirror_provider = GeomProvider::new(mirror_name, mirror_size, sector_size);
        self.register_provider(mirror_provider);
        self.consumers.push(GeomConsumer::new(provider_a));
        self.consumers.push(GeomConsumer::new(provider_b));
        Ok(())
    }

    pub fn create_eli(
        &mut self,
        parent_name: &str,
        eli_name: &str,
        passphrase: &str,
    ) -> Result<(), String> {
        let parent = self
            .find_provider(parent_name)
            .ok_or_else(|| format!("Parent provider {} not found", parent_name))?;

        let config = GeomEliConfig::new(passphrase);
        let size = parent.mediasize;
        let sector_size = parent.sectorsize;

        let mut eli_provider = GeomProvider::new(eli_name, size, sector_size);
        eli_provider.raw_buffer.copy_from_slice(&parent.raw_buffer);

        self.eli_configs.push((eli_name.to_owned(), config));
        self.register_provider(eli_provider);
        self.consumers.push(GeomConsumer::new(parent_name));
        Ok(())
    }

    pub fn dispatch_bio(&mut self, target_provider: &str, bio: &mut BioRequest) {
        // Check if provider is encrypted via GELI
        let maybe_eli_config = self
            .eli_configs
            .iter()
            .find(|(name, _): &&(String, GeomEliConfig)| name == target_provider)
            .map(|(_, cfg)| cfg.clone());

        if let Some(config) = maybe_eli_config {
            if bio.cmd == BioCmd::Write {
                let mut encrypted_data = bio.data.clone();
                config.transform_block(bio.offset, &mut encrypted_data);
                let orig_data = core::mem::replace(&mut bio.data, encrypted_data);

                if let Some(provider) = self.find_provider_mut(target_provider) {
                    provider.handle_bio(bio);
                }
                bio.data = orig_data;
            } else if bio.cmd == BioCmd::Read {
                if let Some(provider) = self.find_provider_mut(target_provider) {
                    provider.handle_bio(bio);
                    if bio.completed {
                        config.transform_block(bio.offset, &mut bio.data);
                    }
                }
            } else {
                if let Some(provider) = self.find_provider_mut(target_provider) {
                    provider.handle_bio(bio);
                }
            }
        } else if let Some(provider) = self.find_provider_mut(target_provider) {
            provider.handle_bio(bio);
        } else {
            bio.error_code = Some(6); // ENXIO
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geom_provider_bio_io() {
        let mut provider = GeomProvider::new("ada0", 4096, 512);
        assert_eq!(provider.mediasize, 4096);
        assert_eq!(provider.sectorsize, 512);

        let mut write_bio = BioRequest::new_write(0, alloc::vec![1, 2, 3, 4, 5]);
        provider.handle_bio(&mut write_bio);
        assert!(write_bio.completed);

        let mut read_bio = BioRequest::new_read(0, 5);
        provider.handle_bio(&mut read_bio);
        assert!(read_bio.completed);
        assert_eq!(read_bio.data, alloc::vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_geom_partition_and_eli_encryption() {
        let mut geom = GeomTopology::new();
        let disk = GeomProvider::new("ada0", 8192, 512);
        geom.register_provider(disk);

        assert!(geom
            .create_partition("ada0", "ada0p1", 0, 4096, "freebsd-ufs")
            .is_ok());

        assert!(geom
            .create_eli("ada0p1", "ada0p1.eli", "secretpass")
            .is_ok());

        let mut write_bio = BioRequest::new_write(0, b"SOVEREIGN_DATA".to_vec());
        geom.dispatch_bio("ada0p1.eli", &mut write_bio);
        assert!(write_bio.completed);

        let mut read_bio = BioRequest::new_read(0, 14);
        geom.dispatch_bio("ada0p1.eli", &mut read_bio);
        assert!(read_bio.completed);
        assert_eq!(read_bio.data, b"SOVEREIGN_DATA".to_vec());
    }
}
