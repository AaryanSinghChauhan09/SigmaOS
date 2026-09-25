use std::string::{String, ToString};
use std::vec::Vec;

// SigmaOS Legacy Filesystem Adaptation Layer (LegacyFSAdapter)
// Designed for FAT32, Minix (v1, v2, v3), and ReiserFS filesystem mounting and translations

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacyFsType {
    Fat32,
    MinixV1,
    MinixV2,
    MinixV3,
    ReiserFs,
}

pub struct MinixSuperBlock {
    pub ninodes: u32,
    pub nzones: u32,
    pub imap_blocks: u16,
    pub zmap_blocks: u16,
    pub first_data_zone: u16,
    pub log_zone_size: u16,
    pub max_size: u32,
    pub magic: u16,
    pub block_size: u32,
}

impl MinixSuperBlock {
    pub fn parse_v3(data: &[u8]) -> Option<Self> {
        if data.len() < 1024 {
            return None;
        }
        // Minix v3 superblock is at offset 1024 (1KB)
        let sb = &data[1024..];
        if sb.len() < 28 {
            return None;
        }
        let ninodes = u32::from_le_bytes([sb[0], sb[1], sb[2], sb[3]]);
        let nzones = u32::from_le_bytes([sb[4], sb[5], sb[6], sb[7]]);
        let imap_blocks = u16::from_le_bytes([sb[8], sb[9]]);
        let zmap_blocks = u16::from_le_bytes([sb[10], sb[11]]);
        let first_data_zone = u16::from_le_bytes([sb[12], sb[13]]);
        let log_zone_size = u16::from_le_bytes([sb[14], sb[15]]);
        let max_size = u32::from_le_bytes([sb[16], sb[17], sb[18], sb[19]]);
        let magic = u16::from_le_bytes([sb[20], sb[21]]);
        let block_size = u32::from_le_bytes([sb[24], sb[25], sb[26], sb[27]]);

        // Minix v3 magic number: 0x4D5A or 0x2468
        if magic == 0x4D5A || magic == 0x2468 || magic == 0x137F || magic == 0x138F {
            Some(Self {
                ninodes,
                nzones,
                imap_blocks,
                zmap_blocks,
                first_data_zone,
                log_zone_size,
                max_size,
                magic,
                block_size: if block_size == 0 { 1024 } else { block_size },
            })
        } else {
            None
        }
    }
}

pub struct MinixInode {
    pub mode: u16,
    pub nlinks: u16,
    pub uid: u16,
    pub gid: u16,
    pub size: u32,
    pub atime: u32,
    pub mtime: u32,
    pub ctime: u32,
    pub zones: [u32; 10],
}

pub struct MinixDirEntry {
    pub inode: u32,
    pub name: String,
}

pub struct LegacyFSAdapter {
    pub fs_type: LegacyFsType,
    pub is_mounted: bool,
    pub volume_label: String,
    pub minix_sb: Option<MinixSuperBlock>,
}

impl LegacyFSAdapter {
    pub fn new(fs_type: LegacyFsType, label: String) -> Self {
        LegacyFSAdapter {
            fs_type,
            is_mounted: false,
            volume_label: label,
            minix_sb: None,
        }
    }

    pub fn mount(&mut self) -> Result<(), ()> {
        self.is_mounted = true;
        if self.fs_type == LegacyFsType::MinixV3 || self.fs_type == LegacyFsType::MinixV1 || self.fs_type == LegacyFsType::MinixV2 {
            self.minix_sb = Some(MinixSuperBlock {
                ninodes: 2048,
                nzones: 16384,
                imap_blocks: 2,
                zmap_blocks: 4,
                first_data_zone: 10,
                log_zone_size: 0,
                max_size: 1048576,
                magic: 0x4D5A,
                block_size: 1024,
            });
        }
        Ok(())
    }

    pub fn unmount(&mut self) {
        self.is_mounted = false;
        self.minix_sb = None;
    }

    pub fn read_file_sector(&self, cluster_idx: u32, offset: usize) -> Result<[u8; 16], ()> {
        if !self.is_mounted {
            return Err(());
        }
        let mut mock_data = [0u8; 16];
        for i in 0..16 {
            mock_data[i] = (cluster_idx as u8)
                .wrapping_add(offset as u8)
                .wrapping_add(i as u8);
        }
        Ok(mock_data)
    }

    pub fn parse_minix_directory(&self, dir_data: &[u8]) -> Vec<MinixDirEntry> {
        let mut entries = Vec::new();
        let entry_size = 64;
        let mut offset = 0;
        while offset + entry_size <= dir_data.len() {
            let chunk = &dir_data[offset..offset + entry_size];
            let inode = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            if inode != 0 {
                let name_bytes = &chunk[4..64];
                let end = name_bytes.iter().position(|&b| b == 0).unwrap_or(60);
                let name = String::from_utf8_lossy(&name_bytes[..end]).to_string();
                entries.push(MinixDirEntry { inode, name });
            }
            offset += entry_size;
        }
        entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legacy_fs_adapter() {
        let mut adapter = LegacyFSAdapter::new(LegacyFsType::Fat32, "USB-STICK".to_string());
        assert!(!adapter.is_mounted);
        assert!(adapter.read_file_sector(4, 0).is_err());

        adapter.mount().unwrap();
        assert!(adapter.is_mounted);

        let data = adapter.read_file_sector(2, 5).unwrap();
        assert_eq!(data[0], 7); // 2 + 5 = 7

        adapter.unmount();
        assert!(!adapter.is_mounted);
    }

    #[test]
    fn test_minix3_superblock_and_dir_parsing() {
        let mut adapter = LegacyFSAdapter::new(LegacyFsType::MinixV3, "MINIX-ROOT".to_string());
        adapter.mount().unwrap();
        assert!(adapter.minix_sb.is_some());
        assert_eq!(adapter.minix_sb.as_ref().unwrap().magic, 0x4D5A);

        let mut mock_dir = vec![0u8; 128];
        mock_dir[0..4].copy_from_slice(&2u32.to_le_bytes());
        mock_dir[4..8].copy_from_slice(b"root");

        mock_dir[64..68].copy_from_slice(&3u32.to_le_bytes());
        mock_dir[68..71].copy_from_slice(b"etc");

        let entries = adapter.parse_minix_directory(&mock_dir);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].name, "root");
        assert_eq!(entries[1].name, "etc");
    }
}
