//! gzip - compress or expand files (RFC 1952)
//! POSIX gzip file header/trailer verification and processing utility

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub const GZIP_MAGIC: [u8; 2] = [0x1f, 0x8b];
pub const GZIP_DEFLATE: u8 = 8;

pub const FTEXT: u8 = 1 << 0;
pub const FHCRC: u8 = 1 << 1;
pub const FEXTRA: u8 = 1 << 2;
pub const FNAME: u8 = 1 << 3;
pub const FCOMMENT: u8 = 1 << 4;

#[derive(Debug, Clone)]
pub struct GzipHeader {
    pub compression_method: u8,
    pub flags: u8,
    pub mtime: u32,
    pub extra_flags: u8,
    pub os: u8,
    pub filename: Option<String>,
}

impl GzipHeader {
    pub fn parse(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 10 {
            return Err("gzip: data too short for header".to_string());
        }

        if bytes[0] != GZIP_MAGIC[0] || bytes[1] != GZIP_MAGIC[1] {
            return Err("gzip: not in gzip format (invalid magic)".to_string());
        }

        let compression_method = bytes[2];
        let flags = bytes[3];
        let mtime = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
        let extra_flags = bytes[8];
        let os = bytes[9];

        let mut offset = 10;
        let mut filename = None;

        if (flags & FEXTRA) != 0 && offset + 2 <= bytes.len() {
            let xlen = u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap()) as usize;
            offset += 2 + xlen;
        }

        if (flags & FNAME) != 0 && offset < bytes.len() {
            let start = offset;
            while offset < bytes.len() && bytes[offset] != 0 {
                offset += 1;
            }
            filename = Some(String::from_utf8_lossy(&bytes[start..offset]).to_string());
        }

        Ok(Self {
            compression_method,
            flags,
            mtime,
            extra_flags,
            os,
            filename,
        })
    }
}

pub fn inspect_gzip(path: &str) -> Result<GzipHeader, String> {
    let mut file = File::open(Path::new(path)).map_err(|e| format!("gzip: {}: {}", path, e))?;
    let mut buf = [0u8; 512];
    let n = file
        .read(&mut buf)
        .map_err(|e| format!("gzip read: {}", e))?;
    GzipHeader::parse(&buf[..n])
}
