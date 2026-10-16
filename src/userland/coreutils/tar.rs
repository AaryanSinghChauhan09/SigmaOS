//! tar - tape archiver utility (POSIX USTAR format)
//! Creates, lists, and extracts tar archive files

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
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
        let n = file
            .read(&mut block)
            .map_err(|e| format!("tar read: {}", e))?;
        if n < TAR_BLOCK_SIZE {
            break;
        }

        match TarHeader::parse(&block)? {
            Some(header) => {
                let size = header.size;
                headers.push(header);

                // Zero-allocation O(1) seek to skip file data blocks without reading bytes into heap buffers
                if size > 0 {
                    let blocks_to_skip = (size + TAR_BLOCK_SIZE - 1) / TAR_BLOCK_SIZE;
                    let bytes_to_skip = (blocks_to_skip * TAR_BLOCK_SIZE) as i64;
                    file.seek(SeekFrom::Current(bytes_to_skip))
                        .map_err(|e| format!("tar seek: {}", e))?;
                }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_tar_header_parse_all_zero_block() {
        let block = [0u8; TAR_BLOCK_SIZE];
        let header = TarHeader::parse(&block).expect("Parsing zero block should succeed");
        assert!(header.is_none());
    }

    #[test]
    fn test_tar_header_parse_valid() {
        let mut block = [0u8; TAR_BLOCK_SIZE];
        block[0..12].copy_from_slice(b"testfile.txt");
        block[100..107].copy_from_slice(b"0000644");
        block[108..115].copy_from_slice(b"0001000");
        block[116..123].copy_from_slice(b"0001000");
        block[124..135].copy_from_slice(b"00000000020"); // 16 bytes octal
        block[156] = b'0'; // regular file

        let header = TarHeader::parse(&block)
            .expect("Parsing header should succeed")
            .expect("Header should be Some");
        assert_eq!(header.name, "testfile.txt");
        assert_eq!(header.mode, 0o644);
        assert_eq!(header.uid, 0o1000);
        assert_eq!(header.gid, 0o1000);
        assert_eq!(header.size, 16);
        assert_eq!(header.typeflag, b'0');
    }

    #[test]
    fn test_list_archive_with_seek() {
        let dir = std::env::temp_dir();
        let tar_path = dir.join("test_archive.tar");
        let path_str = tar_path.to_str().unwrap();

        let mut file = File::create(&tar_path).expect("Failed to create temp tar file");

        // Header 1: file1.txt, size = 600 bytes (spans 2 data blocks = 1024 bytes)
        let mut header1 = [0u8; TAR_BLOCK_SIZE];
        header1[0..9].copy_from_slice(b"file1.txt");
        header1[100..107].copy_from_slice(b"0000644");
        header1[124..135].copy_from_slice(b"00000001130"); // 600 in octal
        file.write_all(&header1).unwrap();
        // Write 1024 data bytes for file1
        file.write_all(&[0xAAu8; 1024]).unwrap();

        // Header 2: file2.txt, size = 0 bytes
        let mut header2 = [0u8; TAR_BLOCK_SIZE];
        header2[0..9].copy_from_slice(b"file2.txt");
        header2[100..107].copy_from_slice(b"0000644");
        header2[124..135].copy_from_slice(b"00000000000");
        file.write_all(&header2).unwrap();

        // EOF block (all zeros)
        file.write_all(&[0u8; TAR_BLOCK_SIZE]).unwrap();
        file.flush().unwrap();

        let headers = list_archive(path_str).expect("list_archive should succeed");
        assert_eq!(headers.len(), 2);
        assert_eq!(headers[0].name, "file1.txt");
        assert_eq!(headers[0].size, 600);
        assert_eq!(headers[1].name, "file2.txt");
        assert_eq!(headers[1].size, 0);

        let _ = std::fs::remove_file(tar_path);
    }
}
