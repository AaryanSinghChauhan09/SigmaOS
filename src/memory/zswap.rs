// SigmaOS Zswap / ZRAM Compressed RAM Swap Pool
// Intercepts swap page-outs and stores compressed page payloads in RAM to reduce disk I/O.

#![allow(dead_code)]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

pub const PAGE_SIZE: usize = 4096;

/// A compressed page frame in the zswap pool
#[derive(Debug, Clone)]
pub struct CompressedPage {
    pub pfn: u64,
    pub compressed_data: Vec<u8>,
    pub original_size: usize,
    pub compression_ratio: u32, // Percentage (e.g. 45%)
}

/// Zswap pool configuration and statistics
#[derive(Debug, Clone)]
pub struct ZswapPool {
    pub max_pool_bytes: usize,
    pub current_bytes: usize,
    pub pages: BTreeMap<u64, CompressedPage>,
    pub stored_pages_count: u64,
    pub rejected_pages_count: u64,
}

impl ZswapPool {
    pub fn new(max_pool_bytes: usize) -> Self {
        Self {
            max_pool_bytes,
            current_bytes: 0,
            pages: BTreeMap::new(),
            stored_pages_count: 0,
            rejected_pages_count: 0,
        }
    }

    /// Run-Length Encoding compressor suitable for no_std kernel space
    pub fn compress_page(data: &[u8; PAGE_SIZE]) -> Vec<u8> {
        let mut out = Vec::with_capacity(PAGE_SIZE);
        let mut i = 0;
        while i < data.len() {
            let byte = data[i];
            let mut run = 1u8;
            while i + (run as usize) < data.len() && data[i + (run as usize)] == byte && run < 255 {
                run += 1;
            }
            out.push(run);
            out.push(byte);
            i += run as usize;
        }
        out
    }

    /// RLE decompressor restoring the 4096-byte page
    pub fn decompress_page(compressed: &[u8], out: &mut [u8; PAGE_SIZE]) -> bool {
        let mut out_idx = 0;
        let mut in_idx = 0;
        while in_idx + 1 < compressed.len() && out_idx < PAGE_SIZE {
            let run = compressed[in_idx] as usize;
            let byte = compressed[in_idx + 1];
            for _ in 0..run {
                if out_idx < PAGE_SIZE {
                    out[out_idx] = byte;
                    out_idx += 1;
                }
            }
            in_idx += 2;
        }
        out_idx == PAGE_SIZE
    }

    /// Attempt to store a dirty page into the zswap pool
    pub fn store(&mut self, pfn: u64, data: &[u8; PAGE_SIZE]) -> Result<u32, &'static str> {
        let compressed = Self::compress_page(data);
        if compressed.len() >= PAGE_SIZE
            || self.current_bytes + compressed.len() > self.max_pool_bytes
        {
            self.rejected_pages_count += 1;
            return Err("Page not compressible enough or zswap pool is full");
        }

        let ratio = ((compressed.len() * 100) / PAGE_SIZE) as u32;
        let comp_len = compressed.len();

        let page = CompressedPage {
            pfn,
            compressed_data: compressed,
            original_size: PAGE_SIZE,
            compression_ratio: ratio,
        };

        if let Some(old) = self.pages.insert(pfn, page) {
            self.current_bytes -= old.compressed_data.len();
        }

        self.current_bytes += comp_len;
        self.stored_pages_count += 1;

        Ok(ratio)
    }

    /// Load and decompress a page from the zswap pool
    pub fn load(&mut self, pfn: u64, out: &mut [u8; PAGE_SIZE]) -> bool {
        if let Some(page) = self.pages.remove(&pfn) {
            self.current_bytes -= page.compressed_data.len();
            Self::decompress_page(&page.compressed_data, out)
        } else {
            false
        }
    }

    /// Evict a page if pool needs shrinking
    pub fn evict_one(&mut self) -> Option<u64> {
        let first_key = self.pages.keys().next().copied()?;
        if let Some(page) = self.pages.remove(&first_key) {
            self.current_bytes -= page.compressed_data.len();
            Some(first_key)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zswap_compression_and_store() {
        let mut pool = ZswapPool::new(1024 * 1024); // 1MB pool
        let mut page_data = [0u8; PAGE_SIZE];
        // Fill first 1000 bytes with 'A', rest zero
        for i in 0..1000 {
            page_data[i] = b'A';
        }

        let ratio = pool
            .store(100, &page_data)
            .expect("Should compress and store");
        assert!(ratio < 10); // Highly compressible

        let mut out = [0u8; PAGE_SIZE];
        assert!(pool.load(100, &mut out));
        assert_eq!(&out[..1000], &page_data[..1000]);
        assert_eq!(&out[1000..], &page_data[1000..]);
    }
}
