// SigmaOS CPIO Newc Initramfs / Initrd Image Builder
// Provides sovereign in-kernel and userland packaging for micro-initramfs root images.

#![allow(dead_code)]
extern crate alloc;

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::Write;

/// CPIO newc magic constant ("070701")
pub const CPIO_NEWC_MAGIC: &[u8; 6] = b"070701";

/// Trailer filename indicating end of archive
pub const CPIO_TRAILER_NAME: &str = "TRAILER!!!";

/// File mode constants for standard CPIO archives
pub const CPIO_MODE_DIR: u32 = 0o040755;
pub const CPIO_MODE_FILE: u32 = 0o100644;
pub const CPIO_MODE_EXEC: u32 = 0o100755;

/// An entry in the initramfs
#[derive(Debug, Clone)]
pub struct InitramfsEntry {
    pub path: String,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub mtime: u32,
    pub data: Vec<u8>,
}

/// Builder for constructing CPIO newc format initramfs archives
#[derive(Debug, Clone, Default)]
pub struct InitramfsBuilder {
    entries: Vec<InitramfsEntry>,
    next_inode: u32,
}

impl InitramfsBuilder {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_inode: 1,
        }
    }

    /// Add a directory to the archive
    pub fn add_dir(&mut self, path: &str) -> &mut Self {
        self.entries.push(InitramfsEntry {
            path: String::from(path.trim_start_matches('/')),
            mode: CPIO_MODE_DIR,
            uid: 0,
            gid: 0,
            mtime: 0,
            data: Vec::new(),
        });
        self
    }

    /// Add a regular data file to the archive
    pub fn add_file(&mut self, path: &str, data: &[u8]) -> &mut Self {
        self.entries.push(InitramfsEntry {
            path: String::from(path.trim_start_matches('/')),
            mode: CPIO_MODE_FILE,
            uid: 0,
            gid: 0,
            mtime: 0,
            data: data.to_vec(),
        });
        self
    }

    /// Add an executable binary file to the archive
    pub fn add_executable(&mut self, path: &str, data: &[u8]) -> &mut Self {
        self.entries.push(InitramfsEntry {
            path: String::from(path.trim_start_matches('/')),
            mode: CPIO_MODE_EXEC,
            uid: 0,
            gid: 0,
            mtime: 0,
            data: data.to_vec(),
        });
        self
    }

    /// Pad buffer to a 4-byte boundary with zeroes
    fn pad4(buf: &mut Vec<u8>) {
        let remainder = buf.len() % 4;
        if remainder != 0 {
            let pad_len = 4 - remainder;
            buf.extend(core::iter::repeat(0u8).take(pad_len));
        }
    }

    /// Format a single 110-byte CPIO newc header
    fn write_header(
        buf: &mut Vec<u8>,
        inode: u32,
        mode: u32,
        uid: u32,
        gid: u32,
        nlink: u32,
        mtime: u32,
        filesize: u32,
        namesize: u32,
    ) {
        let mut header = String::new();
        let _ = write!(
            header,
            "070701{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}00000000",
            inode,
            mode,
            uid,
            gid,
            nlink,
            mtime,
            filesize,
            0, // devmajor
            0, // devminor
            0, // rdevmajor
            0, // rdevminor
            namesize,
        );
        buf.extend_from_slice(header.as_bytes());
    }

    /// Build the complete binary CPIO newc image
    pub fn build(&self) -> Vec<u8> {
        let mut out = Vec::new();
        let mut ino = 1u32;

        for entry in &self.entries {
            let name_bytes = entry.path.as_bytes();
            let namesize = (name_bytes.len() + 1) as u32; // including null terminator
            let filesize = entry.data.len() as u32;

            Self::write_header(
                &mut out,
                ino,
                entry.mode,
                entry.uid,
                entry.gid,
                1,
                entry.mtime,
                filesize,
                namesize,
            );

            // Write name + NUL
            out.extend_from_slice(name_bytes);
            out.push(0);
            Self::pad4(&mut out);

            // Write file payload
            out.extend_from_slice(&entry.data);
            Self::pad4(&mut out);

            ino += 1;
        }

        // Trailer entry
        let trailer_bytes = CPIO_TRAILER_NAME.as_bytes();
        let trailer_namesize = (trailer_bytes.len() + 1) as u32;
        Self::write_header(&mut out, 0, 0, 0, 0, 1, 0, 0, trailer_namesize);
        out.extend_from_slice(trailer_bytes);
        out.push(0);
        Self::pad4(&mut out);

        out
    }

    /// Number of entries currently staged
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initramfs_builder() {
        let mut builder = InitramfsBuilder::new();
        builder
            .add_dir("/bin")
            .add_dir("/etc")
            .add_file("/etc/os-release", b"NAME=SigmaOS\nID=sigmaos\n")
            .add_executable("/bin/init", b"\x7fELFfakeinit");

        assert_eq!(builder.entry_count(), 4);
        let archive = builder.build();

        assert!(archive.len() > 110);
        assert_eq!(&archive[0..6], CPIO_NEWC_MAGIC);

        // Verify trailer presence
        let trailer_pos = archive
            .windows(CPIO_TRAILER_NAME.len())
            .position(|w| w == CPIO_TRAILER_NAME.as_bytes());
        assert!(trailer_pos.is_some());
    }
}
