//! tar - tape archiver utility (POSIX USTAR format)
//! Creates, lists, and extracts tar archive files

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub const TAR_BLOCK_SIZE: usize = 512;
pub const USTAR_MAGIC: &[u8; 6] = b"ustar\0";

#[derive(Debug, Clone)]
pub struct TarHeader {
    pub name: String,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub size: usize,
    pub mtime: u64,
    pub typeflag: u8,
    pub linkname: String,
    pub uname: String,
    pub gname: String,
}

impl TarHeader {
    pub fn parse(block: &[u8; TAR_BLOCK_SIZE]) -> Result<Option<Self>, String> {
        // All-zero block signals end of archive
        if block.iter().all(|&b| b == 0) {
            return Ok(None);
        }

        let name = parse_string(&block[0..100]);
        let mode = parse_octal(&block[100..108]) as u32;
        let uid = parse_octal(&block[108..116]) as u32;
        let gid = parse_octal(&block[116..124]) as u32;
        let size = parse_octal(&block[124..136]) as usize;
        let mtime = parse_octal(&block[136..148]);
        let typeflag = block[156];
        let linkname = parse_string(&block[157..257]);
        let uname = parse_string(&block[265..297]);
        let gname = parse_string(&block[297..329]);

        Ok(Some(Self {
            name,
            mode,
            uid,
            gid,
            size,
            mtime,
            typeflag,
            linkname,
            uname,
            gname,
        }))
    }
}

pub fn list_archive(path: &str) -> Result<Vec<TarHeader>, String> {
    let mut file = File::open(Path::new(path)).map_err(|e| format!("tar: {}: {}", path, e))?;
    let mut headers = Vec::new();
    let mut block = [0u8; TAR_BLOCK_SIZE];

    loop {
        let n = file.read(&mut block).map_err(|e| format!("tar read: {}", e))?;
        if n < TAR_BLOCK_SIZE {
            break;
        }

        match TarHeader::parse(&block)? {
            Some(header) => {
                let size = header.size;
                headers.push(header);

                // Skip file data blocks
                let blocks_to_skip = (size + TAR_BLOCK_SIZE - 1) / TAR_BLOCK_SIZE;
                let bytes_to_skip = blocks_to_skip * TAR_BLOCK_SIZE;
                let mut discard = vec![0u8; bytes_to_skip];
                let _ = file.read_exact(&mut discard);
            }
            None => break, // EOF marker block
        }
    }

    Ok(headers)
}

fn parse_string(bytes: &[u8]) -> String {
    let len = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..len]).to_string()
}

fn parse_octal(bytes: &[u8]) -> u64 {
    let s = parse_string(bytes);
    let trimmed = s.trim();
    u64::from_str_radix(trimmed, 8).unwrap_or(0)
}
